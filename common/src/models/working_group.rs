#[cfg(feature = "juniper_impl")]
use juniper::GraphQLObject;

#[cfg(feature = "serde_impl")]
use serde_derive::{Deserialize, Serialize};

use crate::member::PublicMember;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "juniper_impl", derive(GraphQLObject))]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct WorkingGroup {
    pub year: i32,
    pub members: Vec<WorkingGroupMember>,
}

pub type WorkingGroupMemberId = i32;

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "juniper_impl", derive(GraphQLObject))]
#[cfg_attr(feature = "serde_impl", derive(Serialize, Deserialize))]
pub struct WorkingGroupMember {
    pub id: WorkingGroupMemberId,
    pub member: PublicMember,
}
