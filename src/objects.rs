use async_graphql::{Schema, Context, Object, Subscription, SimpleObject};
use futures_util::Stream;

use crate::RECEIVER;
use crate::db;

pub struct Query;
pub struct Mutation;
pub struct Subscription;
pub type SimpleQuerySchema = Schema<Query, Mutation, Subscription>;

#[derive(SimpleObject)]
pub struct Post {
  pub text: String,
}

#[Object]
impl Query {
  async fn echo<'ctx>(&self, ctx: &Context<'ctx>, message: String) -> String {
    // select or delete database with this pool
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    message
  }
}

#[Object]
impl Mutation {
  async fn post<'ctx>(&self, ctx: &Context<'ctx>) -> bool {
    // select or delete database with this pool
    let conn = &mut ctx.data_unchecked::<db::Pool>().get().unwrap();
    true
  }
}

#[Subscription]
impl Subscription {
  async fn timeline(&self) -> impl Stream<Item = String> {
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
