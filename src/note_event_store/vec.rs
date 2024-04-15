use super::{
    base::NoteEventStore,
    event::{NoteEvent, NoteEventUpdate},
};

pub struct VecNoteEventStore {
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

    fn add_events(&mut self, events: Vec<NoteEvent>) {
        self.store.extend(events);
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

    fn update_events(&mut self, events: Vec<NoteEventUpdate>) {
        for event in events {
            self.update_event(event);
        }
    }

    fn delete_event(&mut self, id: &str) {
        self.store.retain(|e| e.id != id);
    }

    fn delete_events(&mut self, ids: Vec<&str>) {
        self.store.retain(|e| !ids.contains(&e.id.as_str()));
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
    use crate::note_event_store::test_util;

    use super::*;

    #[test]
    fn test_add_event() {
        let mut store = VecNoteEventStore::new();
        test_util::add_event(&mut store);
    }

    #[test]
    fn test_add_events() {
        let mut store = VecNoteEventStore::new();
        test_util::add_events(&mut store);
    }

    #[test]
    fn test_update_event_all_fields() {
        let mut store = VecNoteEventStore::new();
        test_util::update_event_all_fields(&mut store);
    }

    #[test]
    fn test_update_event_partial() {
        let mut store = VecNoteEventStore::new();
        test_util::update_event_partial(&mut store);
    }

    #[test]
    fn test_update_event_all_none() {
        let mut store = VecNoteEventStore::new();
        test_util::update_event_all_none(&mut store);
    }

    #[test]
    fn test_update_event_not_found() {
        let mut store = VecNoteEventStore::new();
        test_util::update_event_not_found(&mut store);
    }

    #[test]
    fn test_update_events() {
        let mut store = VecNoteEventStore::new();
        test_util::update_events(&mut store);
    }

    #[test]
    fn test_delete_event() {
        let mut store = VecNoteEventStore::new();
        test_util::delete_event(&mut store);
    }

    #[test]
    fn test_delete_event_not_found() {
        let mut store = VecNoteEventStore::new();
        test_util::delete_event_not_found(&mut store);
    }

    #[test]
    fn test_delete_events() {
        let mut store = VecNoteEventStore::new();
        test_util::delete_events(&mut store);
    }

    #[test]
    fn test_get_event() {
        let mut store = VecNoteEventStore::new();
        test_util::get_event(&mut store);
    }

    #[test]
    fn test_get_events_by_range() {
        let mut store = VecNoteEventStore::new();
        test_util::get_events_by_range(&mut store);
    }
}
