use super::context::Context;
use crate::database::event::{get_event, get_event_range};
use crate::database::member::add_member;
use crate::database::working_group::{get_working_group, get_working_group_years};
use crate::models::event::{Event, NewEvent};
use crate::models::graphql::IndexedEvent;
use crate::models::signup::{NewSignup, Signup};
use diesel::prelude::*;
use juniper::{graphql_object, graphql_value, FieldError, FieldResult};
use laggit_api::member::{Member, NewMember};
pub use laggit_api::text_content::TextContent;

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

pub struct RootQuery;
graphql_object!(RootQuery: Context |&self| {
    field api_version() -> &str {
        env!("CARGO_PKG_VERSION")
    }

    field event(&executor, id: i32) -> FieldResult<Event>
        as "Get a specific event by ID" {
        let has_auth = gql_auth!(executor, Events(List(Read))).is_ok();

        Ok(get_event(
            &executor.context().pool.get()?,
            id,
            !has_auth,
        )?)
    }

    field events(&executor, low: i32, high: i32) -> FieldResult<Vec<IndexedEvent>>
        as "Get a number of past and/or future events" {
        let has_auth = gql_auth!(executor, Events(List(Read))).is_ok();

        if low >= high {
            return Err(FieldError::new(
                "high must be higher than low",
                graphql_value!({ "bad_request": "Invalid range" })
            ));
        }

        Ok(get_event_range(
            &executor.context().pool.get()?,
            low.into(),
            high.into(),
            !has_auth,
        )?.into_iter()
            .map(IndexedEvent::from)
            .collect())
    }

    field text_content(&executor, tag: String, lang: String) -> FieldResult<TextContent> {
        use crate::schema::tables::text_content::dsl::{text_content};
        let connection = executor.context().pool.get()?;
        let result: TextContent = text_content.find((tag, lang)).first(&connection)?;
        Ok(result)
    }

    field signup(&executor, id: i32) -> FieldResult<Signup> {
        use crate::schema::tables::event_signups::dsl::{event_signups};
        gql_auth!(executor, Events(SignupRead))?;
        let connection = executor.context().pool.get()?;
        let result: Signup = event_signups.find(id).first(&connection)?;
        Ok(result)
    }

    field working_group(&executor, year: i32) -> FieldResult<obj::WorkingGroup> {
        let connection = executor.context().pool.get()?;
        Ok(get_working_group(&connection, year)?)
    }

    field working_group_years(&executor) -> FieldResult<Vec<i32>> {
        let connection = executor.context().pool.get()?;
        Ok(get_working_group_years(&connection)?)
    }
});

pub struct RootMutation;
graphql_object!(RootMutation: Context |&self| {
    field create_member(&executor, new_member: NewMember) -> FieldResult<Member> {
        // TODO: Some sort of captcha
        gql_auth!(executor, Events(SignupRead))?;

        let connection = executor.context().pool.get()?;
        Ok(add_member(&connection, new_member)?)
    }

    field create_event(&executor, new_event: NewEvent) -> FieldResult<Event> {
        use crate::schema::tables::events;
        gql_auth!(executor, Events(List(Write)))?;
        let connection = executor.context().pool.get()?;
        let event: Event = diesel::insert_into(events::table)
            .values(new_event)
            .get_result(&connection)?;
        Ok(event.into())
    }

    field edit_event(&executor, event_id: i32, event: NewEvent) -> FieldResult<Event> {
        use crate::schema::tables::events::dsl::{events, id};
        gql_auth!(executor, Events(List(Write)))?;
        let connection = executor.context().pool.get()?;
        let event: Event = diesel::update(events)
            .filter(id.eq(event_id))
            .set(&event)
            .get_result(&connection)?;
        Ok(event.into())
    }

    field create_signup(&executor, new_signup: NewSignup) -> FieldResult<Signup> {
        use crate::schema::tables::event_signups;

        // TODO: Some sort of captcha
        gql_auth!(executor, Events(SignupRead))?;

        let connection = executor.context().pool.get()?;
        let signup: Signup = diesel::insert_into(event_signups::table)
            .values(new_signup)
            .get_result(&connection)?;
        Ok(signup.into())
    }

    field set_text_content(&executor, tag: String, lang: String, text: String)
        -> FieldResult<TextContent>
    {
        use crate::schema::tables::text_content::dsl::{self, text_content};
        let connection = executor.context().pool.get()?;
        Ok(diesel::insert_into(text_content)
           .values((
                dsl::tag.eq(&tag),
                dsl::lang.eq(&lang),
                dsl::text.eq(&text),
            ))
           .on_conflict((dsl::tag, dsl::lang))
           .do_update()
           .set(dsl::text.eq(&text))
           .get_result(&connection)?)
    }
});

#[cfg(test)]
mod tests {
    use crate::models::{user::JWTConfig, Event, NewEvent};
    use crate::routes::graphql;
    use crate::schema::tables::events;
    use crate::util::catchers::catchers;
    use crate::util::testing::{DatabaseState, UserSession};
    use chrono::{DateTime, NaiveDateTime, Utc};
    use diesel::RunQueryDsl;
    use rocket::http::{ContentType, Header};
    use rocket::local::Client;
    use rocket::routes;
    use rocket_contrib::json;

    #[test]
    fn test_create_event() {
        let (_state, db_pool) = DatabaseState::new();
        let jwt_config = JWTConfig::testing_config();
        let user_session = UserSession::new(&db_pool, &jwt_config);

        let rocket = rocket::ignite()
            .manage(db_pool)
            .manage(graphql::create_schema())
            .manage(jwt_config)
            .register(catchers())
            .mount(
                "/",
                routes![
                    graphql::post_graphql_handler_auth,
                    graphql::post_graphql_handler,
                ],
            );
        let client = Client::new(rocket).unwrap();

        let new_event = json!({
            "title": "Test Event",
            "description": "Test Description",
            "shortDescription": "Test Short Description",
            "title": "Test Event",
            "background": "http://test.ru/jpg.png",
            "location": "Foobar CA",
            "startTime": "3010-01-01T17:00:42Z",
            "endTime": "3010-01-01T23:59:59Z",
        });
        println!("Request Data: {:#?}\n", &new_event);

        let mut response = client
            .post("/graphql")
            .header(ContentType::JSON)
            .header(Header::new("Authorization", user_session.bearer))
            .body(
                json!({
                    "operationName": "CreateEvent",
                    "query": "mutation CreateEvent($ev:NewEvent!) {\n\
                            createEvent(newEvent: $ev) {\n\
                                id               \n\
                                title            \n\
                                description      \n\
                                shortDescription \n\
                                background       \n\
                                location         \n\
                                startTime        \n\
                                endTime          \n\
                                price            \n\
                                published        \n\
                            }\n\
                        }",
                    "variables": {
                        "ev": new_event,
                    }
                })
                .to_string(),
            )
            .dispatch();

        let body = response.body_string().expect("Response has no body");
        let data: serde_json::Value =
            serde_json::from_str(&body).expect(&format!("Could not deserialize JSON: {}", body));

        assert!(data.is_object());
        let json = data.as_object().unwrap();
        println!("Response Data: {:#?}\n", json);
        assert!(json.contains_key("data"));
        let graphql_data = json.get("data").unwrap().as_object().unwrap();

        assert!(graphql_data.contains_key("createEvent"));
        let graphql_data = graphql_data
            .get("createEvent")
            .unwrap()
            .as_object()
            .unwrap();

        assert!(graphql_data.contains_key("id"));
        assert!(graphql_data.contains_key("title"));
        assert!(graphql_data.contains_key("background"));
        assert!(graphql_data.contains_key("location"));
        assert!(graphql_data.contains_key("startTime"));
        assert!(graphql_data.contains_key("endTime"));
        assert!(graphql_data.contains_key("price"));
        assert!(graphql_data.contains_key("published"));
    }

    #[test]
    fn test_get_event() {
        let (_state, db_pool) = DatabaseState::new();
        let jwt_config = JWTConfig::testing_config();
        let user_session = UserSession::new(&db_pool, &jwt_config);

        let new_event = NewEvent {
            title: "Test Event 2".into(),
            description: "Come to this event!".into(),
            short_description: "Blergh".into(),
            background: "http://image.ru/png.jpg".into(),
            location: "Fizzbuzz TX".into(),
            start_time: DateTime::parse_from_rfc3339("3010-01-01T17:00:42Z"),
            end_time: DateTime::parse_from_rfc3339("3010-01-01T23:59:59Z"),
            price: None,
        };
        println!("Request Data: {:#?}\n", &new_event);

        let connection = db_pool.get().expect("Could not get database connection");
        let event: Event = diesel::insert_into(events::table)
            .values(&new_event)
            .get_result(&connection)
            .expect("Could not add new user for testing");

        let rocket = rocket::ignite()
            .manage(db_pool)
            .manage(graphql::create_schema())
            .manage(jwt_config)
            .register(catchers())
            .mount(
                "/",
                routes![
                    graphql::post_graphql_handler_auth,
                    graphql::post_graphql_handler,
                ],
            );
        let client = Client::new(rocket).unwrap();

        let mut response = client
            .post("/graphql")
            .header(ContentType::JSON)
            .header(Header::new("Authorization", user_session.bearer))
            .body(
                json!({
                "operationName": "GetEvent",
                "query": format!("query GetEvent {{\n\
                        event(id: {}) {{\n\
                            id               \n\
                            title            \n\
                            description      \n\
                            shortDescription \n\
                            background       \n\
                            startTime        \n\
                            endTime          \n\
                            price            \n\
                        }}\n\
                    }}", event.id),
                })
                .to_string(),
            )
            .dispatch();

        let body = response.body_string().expect("Response has no body");
        let data: serde_json::Value =
            serde_json::from_str(&body).expect(&format!("Could not deserialize JSON: {}", body));

        assert!(data.is_object());
        let json = data.as_object().unwrap();
        println!("Response Data: {:#?}\n", json);
        assert!(json.contains_key("data"));
        let graphql_data = json.get("data").unwrap().as_object().unwrap();

        assert!(graphql_data.contains_key("event"));
        let graphql_data = graphql_data.get("event").unwrap().as_object().unwrap();

        assert!(graphql_data.contains_key("id"));
        assert!(graphql_data.contains_key("title"));
        assert!(graphql_data.contains_key("description"));
        assert!(graphql_data.contains_key("shortDescription"));
        assert!(graphql_data.contains_key("background"));
        assert!(!graphql_data.contains_key("location"));
        assert!(graphql_data.contains_key("startTime"));
        assert!(graphql_data.contains_key("endTime"));
        assert!(graphql_data.contains_key("price"));
        assert!(!graphql_data.contains_key("published"));
    }
}
