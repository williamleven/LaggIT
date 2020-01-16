use crate::database::DatabaseConn;
use diesel::prelude::*;
use diesel::result::QueryResult as Result;
use laggit_api::member::{Member, NewMember};

pub fn add_member(connection: &DatabaseConn, new_member: NewMember) -> Result<Member> {
    use crate::schema::tables::members::dsl::*;

    diesel::insert_into(members)
        // Since we can't implement diesel Insertable in another crate
        // we have to do these shenanigans for now.
        .values((
            first_name.eq(&new_member.first_name),
            last_name.eq(&new_member.last_name),
            nickname.eq(&new_member.nickname),
            pid.eq(&new_member.pid),
            email.eq(&new_member.email),
            phone_number.eq(&new_member.phone_number),
            address.eq(&new_member.address),
            zip_code.eq(&new_member.zip_code),
            city.eq(&new_member.city),
        ))
        .get_result(connection)
}
