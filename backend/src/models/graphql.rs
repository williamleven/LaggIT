use crate::models::event::Event;
use crate::routes::graphql::context::Context;
use juniper::graphql_object;
use serde_derive::{Deserialize, Serialize};

/// ...Because graphql doesn't support maps.
#[derive(Serialize, Deserialize, Debug)]
pub struct IndexedEvent {
    pub index: i64,
    pub event: Event,
}

graphql_object!(IndexedEvent: Context |&self| {
    field index() -> i32 {
        self.index as i32
    }

    field event() -> &Event {
        &self.event
    }
});

impl From<(i64, Event)> for IndexedEvent {
    fn from((index, event): (i64, Event)) -> IndexedEvent {
        IndexedEvent { index, event }
    }
}
