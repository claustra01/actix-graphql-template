use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use objects::{QuerySchema, Query, Mutation, Subscription};
use once_cell::sync::OnceCell;
use tokio::sync::{mpsc::UnboundedReceiver, Mutex};

mod db;
mod objects;

static RECEIVER: OnceCell<Mutex<UnboundedReceiver<String>>> = OnceCell::new();

async fn index(schema: web::Data<QuerySchema>, req: GraphQLRequest) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn index_ws(
  schema: web::Data<QuerySchema>,
  req: HttpRequest,
  payload: web::Payload,
) -> Result<HttpResponse> {
  GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_graphiql() -> Result<HttpResponse> {
  Ok(HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

  // DB connection
  let pool: db::Pool = db::establish_connection();

  // subscription reciever
  let (cmt, rx) = tokio::sync::mpsc::unbounded_channel::<String>();
  RECEIVER.set(Mutex::new(rx)).unwrap();

  // schema setup
  let schema = Schema::build(Query, Mutation, Subscription)
    .data(pool.clone())
    .data(cmt)
    .finish();

  println!("GraphiQL: http://localhost:8080");

  HttpServer::new(move || {
      App::new()
          .app_data(web::Data::new(schema.to_owned()))
          .service(web::resource("/").guard(guard::Post()).to(index))
          .service(web::resource("/").guard(guard::Get()).guard(guard::Header("upgrade", "websocket")).to(index_ws))
          .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}