#![feature(test)]
use std::fs::File;
use std::io::{Cursor, Read};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

extern crate test;
use lopdf::Document;

#[bench]
fn bench_load(b: &mut test::test::Bencher) {
    let mut buffer = Vec::new();
    File::open("assets/example.pdf")
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    b.iter(|| {
        let stop = Arc::new(AtomicBool::new(false));
        Document::load_from(Cursor::new(&buffer), stop).unwrap();
    })
}

#[bench]
fn bench_load_incremental_pdf(b: &mut test::test::Bencher) {
    let mut buffer = Vec::new();
    File::open("assets/Incremental.pdf")
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    b.iter(|| {
        let stop = Arc::new(AtomicBool::new(false));
        Document::load_from(Cursor::new(&buffer), stop).unwrap();
    })
}
