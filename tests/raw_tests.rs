use std::{error::Error, fs::File, io::Read, path::Path};
extern crate chromaprint;
use chromaprint::{Chromaprint, CHROMAPRINT_ALGORITHM_TEST2};

pub fn load_audio_file<T: AsRef<Path>>(path: T) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer.chunks(2).map(|chunk| i16::from_ne_bytes([chunk[0], chunk[1]])).collect())
}

pub fn load_stereo_audio_file<T: AsRef<Path>>(path: T) -> Result<Vec<i16>, Box<dyn Error>> {
    Ok(load_audio_file(&path)?.chunks(2).map(|chunk| (chunk[0] + chunk[1]) / (chunk.len() as i16)).collect())
}

#[test]
fn fingerprint() {
    let raw = load_stereo_audio_file("tests/data/test_stereo_44100.raw").unwrap();
    let mut c = Chromaprint::new();
    println!("version: {}", chromaprint::Chromaprint::version());
    assert!(c.start(44100, 1));
    assert!(c.feed(&raw));
    println!("feeded");
    assert!(c.finish());
    assert_eq!("AQAAC0kkZUqYREkUnFAXHk8uuMZl6EfO4zu-4ABKFGESWIIMEQE", c.fingerprint().unwrap());
}
