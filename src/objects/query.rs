use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
  async fn echo(&self, message: String) -> String {
    message
  }
}