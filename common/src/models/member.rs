#[cfg(feature = "diesel_impl")]
use diesel_derives::Queryable;

#[cfg(feature = "juniper_impl")]
use juniper::{GraphQLInputObject, GraphQLObject};

#[cfg(feature = "serde_impl")]
use serde_derive::{Deserialize, Serialize};

pub type MemberId = i32;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "diesel_impl", derive(Queryable))]
#[cfg_attr(feature = "juniper_impl", derive(GraphQLObject))]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct Member {
    pub id: MemberId,
    pub first_name: String,
    pub last_name: String,
    pub nickname: Option<String>,
    pub pid: String,
    pub email: String,
    pub phone_number: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "diesel_impl", derive(Queryable))]
#[cfg_attr(feature = "juniper_impl", derive(GraphQLObject))]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct PublicMember {
    pub id: MemberId,
    pub first_name: String,
    pub last_name: String,
    pub nickname: Option<String>,
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "juniper_impl", derive(GraphQLInputObject))]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Eq)]
pub struct NewMember {
    pub first_name: String,
    pub last_name: String,
    pub nickname: Option<String>,
    pub pid: String,
    pub email: String,
    pub phone_number: String,
    pub address: String,
    pub zip_code: String,
    pub city: String,
}

impl From<Member> for PublicMember {
    fn from(member: Member) -> Self {
        PublicMember {
            id: member.id,
            first_name: member.first_name,
            last_name: member.last_name,
            nickname: member.nickname,
        }
    }
}
