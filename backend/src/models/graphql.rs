use crate::models::event::EventWithSignups as EventWS;
use crate::routes::graphql::context::Context;
use juniper::graphql_object;
use serde_derive::{Deserialize, Serialize};

/// ...Because graphql doesn't support maps.
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedEvent {
    pub index: i64,
    pub event: EventWS,
}

graphql_object!(IndexedEvent: Context |&self| {
    field index() -> i32 {
        self.index as i32
    }

    field event() -> &EventWS {
        &self.event
    }
});

impl From<(i64, EventWS)> for IndexedEvent {
    fn from((index, event): (i64, EventWS)) -> IndexedEvent {
        IndexedEvent { index, event }
    }
}
