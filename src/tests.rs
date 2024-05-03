use std::{env, fs::File};

use crate::onion_link::{OnionLink, Storage};


#[test]
fn add_entry() {
    let mut storage = Storage::default();

    let entry = OnionLink::new("Dread", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let _ = storage.add_entry(entry);

    let _ = dbg!(storage);
}

#[test]
fn search_entry() {
    let mut storage = Storage::default();

    let e1 = OnionLink::new("Dread", "http://dreadytofatroptsdj6io7l3xptbet6onoyno2yv7jicoxknyazubrad.onion");
    let e2 = OnionLink::new("Torch", "http://stuff.onion");
    let e3 = OnionLink::new("Store", "http://afad.onion");

    let _ = storage.add_entry(e1);
    let _ = storage.add_entry(e2);
    let _ = storage.add_entry(e3);

    let _ = dbg!(&storage);

    let res = storage.search_entry("o");
    let _ = dbg!(res);
}
