#[cfg(feature = "diesel_impl")]
use diesel_derives::Queryable;

#[cfg(feature = "juniper_impl")]
use juniper::GraphQLObject;

#[cfg(feature = "serde_impl")]
use serde_derive::{Deserialize, Serialize};

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "diesel_impl", derive(Queryable))]
#[cfg_attr(feature = "juniper_impl", derive(GraphQLObject))]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct TextContent {
    pub tag: String,
    pub lang: String,
    pub text: String,
}
