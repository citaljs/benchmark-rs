#[derive(Clone)]
pub struct NoteEvent {
    pub id: String,
    pub start_ticks: u64,
    pub end_ticks: u64,
    pub note_number: u8,
    pub velocity: u8,
}

pub struct NoteEventUpdate {
    pub id: String,
    pub start_ticks: Option<u64>,
    pub end_ticks: Option<u64>,
    pub note_number: Option<u8>,
    pub velocity: Option<u8>,
}
