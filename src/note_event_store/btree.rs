use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
};

use super::{
    base::NoteEventStore,
    event::{NoteEvent, NoteEventUpdate},
};

pub struct BTreeNoteEventStore {
    events: HashMap<String, NoteEvent>,
    start_ticks_index: BTreeMap<u64, HashSet<String>>,
    end_ticks_index: BTreeMap<u64, HashSet<String>>,
}

impl BTreeNoteEventStore {
    pub fn new() -> Self {
        BTreeNoteEventStore {
            events: HashMap::new(),
            start_ticks_index: BTreeMap::new(),
            end_ticks_index: BTreeMap::new(),
        }
    }
}

impl NoteEventStore for BTreeNoteEventStore {
    fn add_event(&mut self, event: NoteEvent) {
        let id = event.id.clone();
        self.events.insert(id.clone(), event.clone());

        let start_ticks = event.start_ticks;
        let end_ticks = event.end_ticks;

        let start_ticks_set = self
            .start_ticks_index
            .entry(start_ticks)
            .or_insert(HashSet::new());
        start_ticks_set.insert(id.clone());

        let end_ticks_set = self
            .end_ticks_index
            .entry(end_ticks)
            .or_insert(HashSet::new());
        end_ticks_set.insert(id);
    }

    fn add_events(&mut self, events: Vec<NoteEvent>) {
        for event in events {
            self.add_event(event);
        }
    }

    fn update_event(&mut self, event: NoteEventUpdate) {
        if let Some(existing_event) = self.events.get_mut(&event.id) {
            if let Some(start_ticks) = event.start_ticks {
                if let Some(existing_start_ticks_set) =
                    self.start_ticks_index.get_mut(&existing_event.start_ticks)
                {
                    existing_start_ticks_set.remove(&event.id);
                    if existing_start_ticks_set.is_empty() {
                        self.start_ticks_index.remove(&existing_event.start_ticks);
                    }
                }

                let new_start_ticks_set = self
                    .start_ticks_index
                    .entry(start_ticks)
                    .or_insert(HashSet::new());
                new_start_ticks_set.insert(event.id.clone());

                existing_event.start_ticks = start_ticks;
            }

            if let Some(end_ticks) = event.end_ticks {
                if let Some(existing_end_ticks_set) =
                    self.end_ticks_index.get_mut(&existing_event.end_ticks)
                {
                    existing_end_ticks_set.remove(&event.id);
                    if existing_end_ticks_set.is_empty() {
                        self.end_ticks_index.remove(&existing_event.end_ticks);
                    }
                }

                let new_end_ticks_set = self
                    .end_ticks_index
                    .entry(end_ticks)
                    .or_insert(HashSet::new());
                new_end_ticks_set.insert(event.id.clone());

                existing_event.end_ticks = end_ticks;
            }

            if let Some(note_number) = event.note_number {
                existing_event.note_number = note_number;
            }

            if let Some(velocity) = event.velocity {
                existing_event.velocity = velocity;
            }
        }
    }

    fn update_events(&mut self, events: Vec<NoteEventUpdate>) {
        for event in events {
            self.update_event(event);
        }
    }

    fn delete_event(&mut self, id: &str) {
        if let Some(event) = self.events.remove(id) {
            let start_ticks = event.start_ticks;
            let end_ticks = event.end_ticks;

            if let Some(start_ticks_set) = self.start_ticks_index.get_mut(&start_ticks) {
                start_ticks_set.remove(id);
                if start_ticks_set.is_empty() {
                    self.start_ticks_index.remove(&start_ticks);
                }
            }

            if let Some(end_ticks_set) = self.end_ticks_index.get_mut(&end_ticks) {
                end_ticks_set.remove(id);
                if end_ticks_set.is_empty() {
                    self.end_ticks_index.remove(&end_ticks);
                }
            }
        }
    }

    fn delete_events(&mut self, ids: Vec<&str>) {
        for id in ids {
            self.delete_event(id);
        }
    }

    fn get_event(&self, id: &str) -> Option<&NoteEvent> {
        self.events.get(id)
    }

    fn get_events_by_range(&self, start_ticks: u64, end_ticks: u64) -> Vec<&NoteEvent> {
        let got_event_ids: RefCell<HashSet<String>> = RefCell::new(HashSet::new());

        let hit_by_start_ticks_events_iter = self
            .start_ticks_index
            .range(start_ticks..=end_ticks)
            .flat_map(|(_, ids)| {
                ids.iter().filter_map(|id| {
                    got_event_ids.borrow_mut().insert(id.clone());
                    self.events.get(id)
                })
            });

        let hit_by_end_ticks_events_iter =
            self.end_ticks_index
                .range(start_ticks..)
                .flat_map(|(_, ids)| {
                    ids.iter().filter_map(|id| {
                        if let Some(event) = self.events.get(id) {
                            if event.start_ticks <= end_ticks
                                && !got_event_ids.borrow().contains(id)
                            {
                                return Some(event);
                            }
                        }
                        None
                    })
                });

        hit_by_start_ticks_events_iter
            .chain(hit_by_end_ticks_events_iter)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::note_event_store::test_util;

    use super::*;

    #[test]
    fn test_add_event() {
        let mut store = BTreeNoteEventStore::new();
        test_util::add_event(&mut store);
    }

    #[test]
    fn test_add_events() {
        let mut store = BTreeNoteEventStore::new();
        test_util::add_events(&mut store);
    }

    #[test]
    fn test_update_event_all_fields() {
        let mut store = BTreeNoteEventStore::new();
        test_util::update_event_all_fields(&mut store);
    }

    #[test]
    fn test_update_event_partial() {
        let mut store = BTreeNoteEventStore::new();
        test_util::update_event_partial(&mut store);
    }

    #[test]
    fn test_update_event_all_none() {
        let mut store = BTreeNoteEventStore::new();
        test_util::update_event_all_none(&mut store);
    }

    #[test]
    fn test_update_event_not_found() {
        let mut store = BTreeNoteEventStore::new();
        test_util::update_event_not_found(&mut store);
    }

    #[test]
    fn test_update_events() {
        let mut store = BTreeNoteEventStore::new();
        test_util::update_events(&mut store);
    }

    #[test]
    fn test_delete_event() {
        let mut store = BTreeNoteEventStore::new();
        test_util::delete_event(&mut store);
    }

    #[test]
    fn test_delete_event_not_found() {
        let mut store = BTreeNoteEventStore::new();
        test_util::delete_event_not_found(&mut store);
    }

    #[test]
    fn test_delete_events() {
        let mut store = BTreeNoteEventStore::new();
        test_util::delete_events(&mut store);
    }

    #[test]
    fn test_get_event() {
        let mut store = BTreeNoteEventStore::new();
        test_util::get_event(&mut store);
    }

    #[test]
    fn test_get_events_by_range() {
        let mut store = BTreeNoteEventStore::new();
        test_util::get_events_by_range(&mut store);
    }
}
