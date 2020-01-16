use crate::database::DatabaseConn;
use diesel::prelude::*;
use diesel::result::{Error, QueryResult as Result};
use laggit_api::member::Member;

// Object-based data types, for the public api
mod obj {
    pub use laggit_api::working_group::{WorkingGroup, WorkingGroupMember};
}

// Relational data types, from the database
mod rel {
    pub use crate::models::working_group::{
        WorkingGroup, WorkingGroupMember, WorkingGroupMembership,
    };
}

pub fn get_working_group_years(connection: &DatabaseConn) -> Result<Vec<i32>> {
    use crate::schema::tables::working_group::dsl::{working_group, year};
    working_group.select(year).load(connection)
}

pub fn get_working_group(connection: &DatabaseConn, year: i32) -> Result<obj::WorkingGroup> {
    let wgs: Vec<(
        rel::WorkingGroup,
        Option<rel::WorkingGroupMembership>,
        Option<rel::WorkingGroupMember>,
        Option<Member>,
    )> = {
        use crate::schema::tables::members::dsl::{id as id_members, members};
        use crate::schema::tables::working_group::dsl::{working_group, year as year_gr};
        use crate::schema::tables::working_group_members::dsl::{
            id as id_working_group_members, member_id, working_group_members,
        };
        use crate::schema::tables::working_group_membership::dsl::{
            working_group_member_id, working_group_membership, year as year_mem,
        };

        working_group
            .filter(year_gr.eq(year))
            .left_join(working_group_membership.on(year_mem.eq(year_gr)))
            .left_join(
                working_group_members.on(id_working_group_members.eq(working_group_member_id)),
            )
            .left_join(members.on(id_members.eq(member_id)))
            .load(connection)?
    };

    let year = wgs.first().ok_or(Error::NotFound)?.0.year;
    let members: Vec<_> = wgs
        .into_iter()
        .filter_map(|(_, ms, wg_member, member)| {
            debug_assert!(ms.is_some());
            wg_member.map(|wgm| member.map(|m| (wgm, m)))
        })
        .flatten()
        .map(|(wg_member, member)| obj::WorkingGroupMember {
            id: wg_member.id,
            member: member.into(),
        })
        .collect();

    Ok(obj::WorkingGroup { year, members })
}
