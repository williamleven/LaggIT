pub use laggit_api::member::MemberId;
pub use laggit_api::working_group::WorkingGroupMemberId;

#[derive(Debug, PartialEq, Eq, Queryable)]
pub struct WorkingGroup {
    pub year: i32,
}

#[derive(Debug, PartialEq, Eq, Queryable)]
pub struct WorkingGroupMember {
    pub id: WorkingGroupMemberId,
    pub member: MemberId,
}

#[derive(Debug, PartialEq, Eq, Queryable)]
pub struct WorkingGroupMembership {
    pub year: i32,
    pub working_group_member_id: WorkingGroupMemberId,
}
