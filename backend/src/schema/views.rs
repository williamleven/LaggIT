// Bindings to database views aren't automatically generated by diesel.
// This file has to be updated manually.

table! {
    events_with_signups (id) {
        id -> Int4,
        title -> Text,
        background -> Text,
        location -> Text,
        start_time -> Timestamp,
        end_time -> Timestamp,
        price -> Int4,
        published -> Bool,
        signups -> Int8,
        description -> Text,
        short_description -> Text,
    }
}

table! {
    inventory_stock (name) {
        id -> Int4,
        name -> Text,
        price -> Nullable<Int4>,
        stock -> Int4,
    }
}
