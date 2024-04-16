#![feature(test)]

extern crate test;

use benchmark_rs::note_event_store::{
    base::NoteEventStore,
    event::{NoteEvent, NoteEventUpdate},
    vec::VecNoteEventStore,
};

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_events(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();

        b.iter(|| {
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
            ])
        });
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

    #[bench]
    fn bench_update_events(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
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

        b.iter(|| {
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
            ])
        });
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

    #[bench]
    fn bench_delete_events(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
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

        b.iter(|| store.delete_events(vec!["0", "1"]));
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
