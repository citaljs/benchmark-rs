use super::event::{NoteEvent, NoteEventUpdate};

pub trait NoteEventStore {
    fn add_event(&mut self, event: NoteEvent);
    fn add_events(&mut self, events: Vec<NoteEvent>);
    fn update_event(&mut self, event: NoteEventUpdate);
    fn update_events(&mut self, events: Vec<NoteEventUpdate>);
    fn delete_event(&mut self, id: &str);
    fn delete_events(&mut self, ids: Vec<&str>);
    fn get_event(&self, id: &str) -> Option<&NoteEvent>;
    fn get_events_by_range(&self, start_ticks: u64, end_ticks: u64) -> Vec<&NoteEvent>;
}
