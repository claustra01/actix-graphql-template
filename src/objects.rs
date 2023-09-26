use async_graphql::{Schema, Context, Object, Subscription, SimpleObject};
use futures_util::Stream;
use tokio::sync::mpsc::UnboundedSender;

use crate::RECEIVER;
use crate::db;

pub struct Query;
pub struct Mutation;
pub struct Subscription;
pub type QuerySchema = Schema<Query, Mutation, Subscription>;

#[derive(SimpleObject)]
pub struct Post {
  pub text: String,
}

#[Object]
impl Query {
  async fn echo<'ctx>(&self, ctx: &Context<'ctx>, message: String) -> String {
    // select or delete database with this pool
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    // return
    message
  }
}

#[Object]
impl Mutation {
  async fn post<'ctx>(&self, ctx: &Context<'ctx>, message: String) -> bool {
    // select or delete database with this pool
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    // push to subscription reciever
    let cmt = ctx.data_unchecked::<UnboundedSender<String>>();
    cmt.send(message.clone()).unwrap();
    // return
    true
  }
}

#[Subscription]
impl Subscription {
  async fn subscribe(&self) -> impl Stream<Item = String> {
    // return item if reciever has new push
    async_stream::stream! {
      loop {
        let mut rx = RECEIVER.get().unwrap().lock().await;
        if let Some(item) = (*rx).recv().await {
          yield item;
        }
      }
    }
  }
}
