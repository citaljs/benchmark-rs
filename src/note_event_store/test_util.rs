use crate::note_event_store::event::NoteEventUpdate;

use super::{base::NoteEventStore, event::NoteEvent};

#[allow(dead_code)]
pub fn add_event(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });

    let event = store.get_event("0").unwrap();

    assert_eq!(event.id, "0");
    assert_eq!(event.start_ticks, 0);
    assert_eq!(event.end_ticks, 10);
    assert_eq!(event.note_number, 60);
    assert_eq!(event.velocity, 100);
}

#[allow(dead_code)]
pub fn add_events(store: &mut impl NoteEventStore) {
    store.add_events(vec![
        NoteEvent {
            id: "0".to_string(),
            start_ticks: 0,
            end_ticks: 10,
            note_number: 60,
            velocity: 100,
        },
        NoteEvent {
            id: "1".to_string(),
            start_ticks: 10,
            end_ticks: 20,
            note_number: 70,
            velocity: 90,
        },
    ]);

    let event_0 = store.get_event("0").unwrap();
    let event_1 = store.get_event("1").unwrap();

    assert_eq!(event_0.id, "0");
    assert_eq!(event_0.start_ticks, 0);
    assert_eq!(event_0.end_ticks, 10);
    assert_eq!(event_0.note_number, 60);
    assert_eq!(event_0.velocity, 100);

    assert_eq!(event_1.id, "1");
    assert_eq!(event_1.start_ticks, 10);
    assert_eq!(event_1.end_ticks, 20);
    assert_eq!(event_1.note_number, 70);
    assert_eq!(event_1.velocity, 90);
}

#[allow(dead_code)]
pub fn update_event_all_fields(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });

    store.update_event(NoteEventUpdate {
        id: "0".to_string(),
        start_ticks: Some(5),
        end_ticks: Some(15),
        note_number: Some(70),
        velocity: Some(90),
    });

    let event = store.get_event("0").unwrap();

    assert_eq!(event.id, "0");
    assert_eq!(event.start_ticks, 5);
    assert_eq!(event.end_ticks, 15);
    assert_eq!(event.note_number, 70);
    assert_eq!(event.velocity, 90);
}

#[allow(dead_code)]
pub fn update_event_partial(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });

    store.update_event(NoteEventUpdate {
        id: "0".to_string(),
        start_ticks: Some(5),
        end_ticks: None,
        note_number: None,
        velocity: Some(90),
    });

    let event = store.get_event("0").unwrap();

    assert_eq!(event.id, "0");
    assert_eq!(event.start_ticks, 5);
    assert_eq!(event.end_ticks, 10);
    assert_eq!(event.note_number, 60);
    assert_eq!(event.velocity, 90);
}

#[allow(dead_code)]
pub fn update_event_all_none(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });

    store.update_event(NoteEventUpdate {
        id: "0".to_string(),
        start_ticks: None,
        end_ticks: None,
        note_number: None,
        velocity: None,
    });

    let event = store.get_event("0").unwrap();

    assert_eq!(event.id, "0");
    assert_eq!(event.start_ticks, 0);
    assert_eq!(event.end_ticks, 10);
    assert_eq!(event.note_number, 60);
    assert_eq!(event.velocity, 100);
}

#[allow(dead_code)]
pub fn update_event_not_found(store: &mut impl NoteEventStore) {
    store.update_event(NoteEventUpdate {
        id: "0".to_string(),
        start_ticks: Some(5),
        end_ticks: Some(15),
        note_number: Some(70),
        velocity: Some(90),
    });

    assert!(store.get_event("0").is_none());
}

#[allow(dead_code)]
pub fn update_events(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });
    store.add_event(NoteEvent {
        id: "1".to_string(),
        start_ticks: 10,
        end_ticks: 20,
        note_number: 70,
        velocity: 90,
    });

    store.update_events(vec![
        NoteEventUpdate {
            id: "0".to_string(),
            start_ticks: Some(5),
            end_ticks: Some(15),
            note_number: Some(70),
            velocity: Some(90),
        },
        NoteEventUpdate {
            id: "1".to_string(),
            start_ticks: Some(15),
            end_ticks: Some(25),
            note_number: Some(80),
            velocity: Some(80),
        },
    ]);

    let event_0 = store.get_event("0").unwrap();
    let event_1 = store.get_event("1").unwrap();

    assert_eq!(event_0.id, "0");
    assert_eq!(event_0.start_ticks, 5);
    assert_eq!(event_0.end_ticks, 15);
    assert_eq!(event_0.note_number, 70);
    assert_eq!(event_0.velocity, 90);

    assert_eq!(event_1.id, "1");
    assert_eq!(event_1.start_ticks, 15);
    assert_eq!(event_1.end_ticks, 25);
    assert_eq!(event_1.note_number, 80);
    assert_eq!(event_1.velocity, 80);
}

#[allow(dead_code)]
pub fn delete_event(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });

    store.delete_event("0");

    assert!(store.get_event("0").is_none());
}

#[allow(dead_code)]
pub fn delete_event_not_found(store: &mut impl NoteEventStore) {
    store.delete_event("0");

    assert!(store.get_event("0").is_none());
}

#[allow(dead_code)]
pub fn delete_events(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });
    store.add_event(NoteEvent {
        id: "1".to_string(),
        start_ticks: 10,
        end_ticks: 20,
        note_number: 70,
        velocity: 90,
    });

    store.delete_events(vec!["0", "1"]);

    assert!(store.get_event("0").is_none());
    assert!(store.get_event("1").is_none());
}

#[allow(dead_code)]
pub fn get_event(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 10,
        note_number: 60,
        velocity: 100,
    });

    let event = store.get_event("0").unwrap();

    assert_eq!(event.id, "0");
    assert_eq!(event.start_ticks, 0);
    assert_eq!(event.end_ticks, 10);
    assert_eq!(event.note_number, 60);
    assert_eq!(event.velocity, 100);
}

#[allow(dead_code)]
pub fn get_events_by_range(store: &mut impl NoteEventStore) {
    store.add_event(NoteEvent {
        id: "0".to_string(),
        start_ticks: 0,
        end_ticks: 4,
        note_number: 60,
        velocity: 100,
    });

    store.add_event(NoteEvent {
        id: "1".to_string(),
        start_ticks: 0,
        end_ticks: 5,
        note_number: 60,
        velocity: 100,
    });

    store.add_event(NoteEvent {
        id: "2".to_string(),
        start_ticks: 10,
        end_ticks: 15,
        note_number: 60,
        velocity: 100,
    });

    store.add_event(NoteEvent {
        id: "3".to_string(),
        start_ticks: 11,
        end_ticks: 15,
        note_number: 60,
        velocity: 100,
    });

    store.add_event(NoteEvent {
        id: "4".to_string(),
        start_ticks: 0,
        end_ticks: 15,
        note_number: 60,
        velocity: 100,
    });

    store.add_event(NoteEvent {
        id: "5".to_string(),
        start_ticks: 7,
        end_ticks: 8,
        note_number: 60,
        velocity: 100,
    });

    let mut events = store.get_events_by_range(5, 10);
    events.sort_by(|a, b| a.id.cmp(&b.id));

    assert_eq!(events.len(), 4);
    assert_eq!(events[0].id, "1");
    assert_eq!(events[1].id, "2");
    assert_eq!(events[2].id, "4");
    assert_eq!(events[3].id, "5");
}
