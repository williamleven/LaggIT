use crate::database::DatabaseConn;
use crate::models::event::{Event, EventWithSignups as EventWS};
use chrono::Local;
use diesel::prelude::*;
use diesel::result::QueryResult as Result;
use std::collections::HashMap;

pub fn get_event(connection: &DatabaseConn, id: i32, published_only: bool) -> Result<Event> {
    use crate::schema::tables::events::dsl::{events, published};

    Ok(events
        .find(id)
        .filter(published.eq(true).or(!published_only))
        .first(connection)?)
}

pub fn get_event_ws(connection: &DatabaseConn, id: i32, published_only: bool) -> Result<EventWS> {
    use crate::schema::views::events_with_signups::dsl::{events_with_signups, published};

    Ok(events_with_signups
        .find(id)
        .filter(published.eq(true).or(!published_only))
        .first(connection)?)
}

pub fn get_event_range(
    connection: &DatabaseConn,
    low: i64,
    high: i64,
    published_only: bool,
) -> Result<HashMap<i64, Event>> {
    use crate::schema::tables::events::dsl::*;

    assert!(high > low);

    let now = Local::now().naive_local();

    let mut previous: Vec<Event> = if low < 0 {
        events
            .filter(end_time.le(now))
            .filter(published.eq(true).or(!published_only))
            .order_by(start_time.desc())
            .limit(-low)
            .load(connection)?
    } else {
        Vec::new()
    };

    let mut upcoming: Vec<Event> = if high > 0 {
        events
            .filter(end_time.gt(now))
            .filter(published.eq(true).or(!published_only))
            .order_by(start_time.asc())
            .limit(high)
            .load(connection)?
    } else {
        Vec::new()
    };

    if high < 0 {
        if (-high) as usize >= previous.len() {
            previous = Vec::new();
        } else {
            previous.drain(..(-high as usize));
        }
    }

    if low > 0 {
        if low as usize >= upcoming.len() {
            upcoming = Vec::new();
        } else {
            upcoming.drain(..(low as usize));
        }
    }

    let mut map: HashMap<i64, Event> = HashMap::new();

    for (i, ev) in upcoming.into_iter().enumerate() {
        map.insert(i as i64 + 1, ev);
    }
    for (i, ev) in previous.into_iter().enumerate() {
        map.insert(-(i as i64) - 1, ev);
    }

    Ok(map)
}

pub fn get_event_ws_range(
    connection: &DatabaseConn,
    low: i64,
    high: i64,
    published_only: bool,
) -> Result<HashMap<i64, EventWS>> {
    use crate::schema::views::events_with_signups::dsl::*;

    assert!(high > low);

    let now = Local::now().naive_local();

    let mut previous: Vec<EventWS> = if low < 0 {
        events_with_signups
            .filter(end_time.le(now))
            .filter(published.eq(true).or(!published_only))
            .order_by(start_time.desc())
            .limit(-low)
            .load(connection)?
    } else {
        Vec::new()
    };

    let mut upcoming: Vec<EventWS> = if high > 0 {
        events_with_signups
            .filter(end_time.gt(now))
            .filter(published.eq(true).or(!published_only))
            .order_by(start_time.asc())
            .limit(high)
            .load(connection)?
    } else {
        Vec::new()
    };

    if high < 0 {
        if (-high) as usize >= previous.len() {
            previous = Vec::new();
        } else {
            previous.drain(..(-high as usize));
        }
    }

    if low > 0 {
        if low as usize >= upcoming.len() {
            upcoming = Vec::new();
        } else {
            upcoming.drain(..(low as usize));
        }
    }

    let mut map: HashMap<i64, EventWS> = HashMap::new();

    for (i, ev) in upcoming.into_iter().enumerate() {
        map.insert(i as i64 + 1, ev);
    }
    for (i, ev) in previous.into_iter().enumerate() {
        map.insert(-(i as i64) - 1, ev);
    }

    Ok(map)
}
