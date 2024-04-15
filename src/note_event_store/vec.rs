use super::{
    base::NoteEventStore,
    event::{NoteEvent, NoteEventUpdate},
};

struct VecNoteEventStore {
    store: Vec<NoteEvent>,
}

impl VecNoteEventStore {
    pub fn new() -> Self {
        VecNoteEventStore { store: vec![] }
    }
}

impl NoteEventStore for VecNoteEventStore {
    fn add_event(&mut self, event: NoteEvent) {
        self.store.push(event);
    }

    fn update_event(&mut self, event: NoteEventUpdate) {
        if let Some(event_index) = self.store.iter().position(|e| e.id == event.id) {
            let old_event = &self.store[event_index];
            let new_event = NoteEvent {
                id: old_event.id.clone(),
                start_ticks: event.start_ticks.unwrap_or(old_event.start_ticks),
                end_ticks: event.end_ticks.unwrap_or(old_event.end_ticks),
                note_number: event.note_number.unwrap_or(old_event.note_number),
                velocity: event.velocity.unwrap_or(old_event.velocity),
            };
            self.store[event_index] = new_event;
        }
    }

    fn delete_event(&mut self, id: &str) {
        self.store.retain(|e| e.id != id);
    }

    fn get_event(&self, id: &str) -> Option<&NoteEvent> {
        self.store.iter().find(|e| e.id == id)
    }

    fn get_events_by_range(&self, start_ticks: u64, end_ticks: u64) -> Vec<&NoteEvent> {
        self.store
            .iter()
            .filter(|e| e.end_ticks >= start_ticks && e.start_ticks <= end_ticks)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_add_event() {
        let mut store = VecNoteEventStore::new();

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

    #[bench]
    fn bench_add_event(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();

        b.iter(|| {
            store.add_event(NoteEvent {
                id: "0".to_string(),
                start_ticks: 0,
                end_ticks: 10,
                note_number: 60,
                velocity: 100,
            })
        });
    }

    #[test]
    fn test_update_event_all_fields() {
        let mut store = VecNoteEventStore::new();
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

    #[test]
    fn test_update_event_partial() {
        let mut store = VecNoteEventStore::new();
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

    #[test]
    fn test_update_event_all_none() {
        let mut store = VecNoteEventStore::new();
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

    #[test]
    fn test_update_event_not_found() {
        let mut store = VecNoteEventStore::new();

        store.update_event(NoteEventUpdate {
            id: "0".to_string(),
            start_ticks: Some(5),
            end_ticks: Some(15),
            note_number: Some(70),
            velocity: Some(90),
        });

        assert!(store.get_event("0").is_none());
    }

    #[bench]
    fn bench_update_event(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
        store.add_event(NoteEvent {
            id: "0".to_string(),
            start_ticks: 0,
            end_ticks: 10,
            note_number: 60,
            velocity: 100,
        });

        b.iter(|| {
            store.update_event(NoteEventUpdate {
                id: "0".to_string(),
                start_ticks: Some(5),
                end_ticks: Some(15),
                note_number: Some(70),
                velocity: Some(90),
            })
        });
    }

    #[test]
    fn test_delete_event() {
        let mut store = VecNoteEventStore::new();
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

    #[test]
    fn test_delete_event_not_found() {
        let mut store = VecNoteEventStore::new();

        store.delete_event("0");

        assert!(store.get_event("0").is_none());
    }

    #[bench]
    fn bench_delete_event(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
        store.add_event(NoteEvent {
            id: "0".to_string(),
            start_ticks: 0,
            end_ticks: 10,
            note_number: 60,
            velocity: 100,
        });

        b.iter(|| store.delete_event("0"));
    }

    #[test]
    fn test_get_event() {
        let mut store = VecNoteEventStore::new();
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

    #[bench]
    fn bench_get_event(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
        store.add_event(NoteEvent {
            id: "0".to_string(),
            start_ticks: 0,
            end_ticks: 10,
            note_number: 60,
            velocity: 100,
        });

        b.iter(|| store.get_event("0"));
    }

    #[test]
    fn test_get_events_by_range() {
        let mut store = VecNoteEventStore::new();

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

        let events = store.get_events_by_range(5, 10);

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].id, "1");
        assert_eq!(events[1].id, "2");
    }

    #[bench]
    fn bench_get_events_by_range(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();

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

        b.iter(|| store.get_events_by_range(5, 10));
    }
}
