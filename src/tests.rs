use crate::storage::{OnionLink, Storage};


#[test]
fn add_entries() {
    let mut storage = Storage::default();

    let e1 = OnionLink::new("Dread", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let e2 = OnionLink::new("culo", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let e3 = OnionLink::new("store", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let e4 = OnionLink::new("bbc", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let e5 = OnionLink::new("news", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let _ = storage.add_entry(e1);
    let _ = storage.add_entry(e2);
    let _ = storage.add_entry(e3);
    let _ = storage.add_entry(e4);
    let _ = storage.add_entry(e5);

    let _ = dbg!(storage);
}

