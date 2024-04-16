use benchmark_rs::note_event_store::{
    base::NoteEventStore, event::NoteEvent, vec::VecNoteEventStore,
};

#[cfg(test)]
mod benches {
    use super::*;
    use benchmark_rs::note_event_store::btree::BTreeNoteEventStore;
    use rand::Rng;
    use test::Bencher;

    fn bench_add_1000_random_events(b: &mut Bencher, store: &mut impl NoteEventStore) {
        let mut rng = rand::thread_rng();

        b.iter(|| {
            (0..1000).for_each(|i| {
                let start_ticks = rng.gen_range(0..1000);
                store.add_event(NoteEvent {
                    id: i.to_string(),
                    start_ticks,
                    end_ticks: start_ticks + rng.gen_range(0..100),
                    note_number: 60,
                    velocity: 100,
                })
            })
        });
    }

    #[bench]
    fn bench_add_1000_random_events_to_vec_store(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
        bench_add_1000_random_events(b, &mut store);
    }

    #[bench]
    fn bench_add_1000_random_events_to_btree_store(b: &mut Bencher) {
        let mut store = BTreeNoteEventStore::new();
        bench_add_1000_random_events(b, &mut store)
    }

    fn bench_add_1000_ascend_events(b: &mut Bencher, store: &mut impl NoteEventStore) {
        b.iter(|| {
            (0..1000).for_each(|i| {
                store.add_event(NoteEvent {
                    id: i.to_string(),
                    start_ticks: i,
                    end_ticks: i + 10,
                    note_number: 60,
                    velocity: 100,
                })
            })
        });
    }

    #[bench]
    fn bench_add_1000_ascend_events_to_vec_store(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
        bench_add_1000_ascend_events(b, &mut store);
    }

    #[bench]
    fn bench_add_1000_ascend_events_to_btree_store(b: &mut Bencher) {
        let mut store = BTreeNoteEventStore::new();
        bench_add_1000_ascend_events(b, &mut store);
    }

    fn bench_add_1000_descend_events(b: &mut Bencher, store: &mut impl NoteEventStore) {
        b.iter(|| {
            (0..1000).rev().for_each(|i| {
                store.add_event(NoteEvent {
                    id: i.to_string(),
                    start_ticks: i,
                    end_ticks: i + 10,
                    note_number: 60,
                    velocity: 100,
                })
            })
        });
    }

    #[bench]
    fn bench_add_1000_descend_events_to_vec_store(b: &mut Bencher) {
        let mut store = VecNoteEventStore::new();
        bench_add_1000_descend_events(b, &mut store);
    }

    #[bench]
    fn bench_add_1000_descend_events_to_btree_store(b: &mut Bencher) {
        let mut store = BTreeNoteEventStore::new();
        bench_add_1000_descend_events(b, &mut store);
    }
}
