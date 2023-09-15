use async_graphql::{Schema, EmptyMutation, EmptySubscription};

pub mod query;
pub type SimpleQuerySchema = Schema<query::Query, EmptyMutation, EmptySubscription>;