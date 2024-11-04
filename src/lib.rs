use flate2::read::GzDecoder;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use reqwest::blocking::get;
use std::io::BufReader;

pub mod artist;
pub mod label;
pub mod master;
pub mod release;

use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub struct Stream<T> {
    stream: BufReader<GzDecoder<reqwest::blocking::Response>>,
    phantom: PhantomData<T>,
}

impl<T> Stream<T> {
    pub fn new(url: &str) -> Stream<T> {
        let response = get(url).unwrap();
        let decoder = GzDecoder::new(response);
        let mut reader = Reader::from_reader(BufReader::new(decoder));

        let mut buf = Vec::new();
        let _start = match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => start,
            Ok(event) => panic!("unexpected event {:?}", event),
            Err(err) => panic!("error {}", err),
        };

        return Stream::<T> {
            stream: reader.into_inner(),
            phantom: PhantomData,
        };
    }
}

impl<T: DeserializeOwned> Iterator for Stream<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let item: T = match quick_xml::de::from_reader(&mut self.stream) {
            Ok(item) => item,
            _ => return None,
        };
        return Some(item);
    }
}
