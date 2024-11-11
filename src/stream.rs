use flate2::read::GzDecoder;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::DeError;
use reqwest::blocking::get;
use serde::de::DeserializeOwned;
use std::io::BufReader;
use std::io::Read;

pub struct Stream {
    start: String,
    stream: BufReader<GzDecoder<reqwest::blocking::Response>>,
}

impl Stream {
    pub fn new(url: &str) -> Stream {
        let response = get(url).unwrap();
        let decoder = GzDecoder::new(response);
        let mut reader = Reader::from_reader(BufReader::new(decoder));

        let mut buf = Vec::new();
        let start = match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => start,
            Ok(event) => panic!("unexpected start tag {:?}", event),
            Err(err) => panic!("error reading xml start tag {}", err),
        };

        let name = std::str::from_utf8(start.name().into_inner())
            .unwrap()
            .to_string();
        return Stream {
            start: name,
            stream: reader.into_inner(),
        };
    }

    pub fn next<T: DeserializeOwned>(&mut self) -> Option<T> {
        let item: T = match quick_xml::de::from_reader(&mut self.stream) {
            Ok(item) => item,
            Err(err) => match err {
                DeError::InvalidXml(quick_xml::Error::EndEventMismatch { expected, found })
                    if expected == "" && found == self.start =>
                {
                    let mut buf = Vec::new();
                    match self.stream.read(&mut buf) {
                        Ok(0) => return None,
                        Ok(_) => panic!("unexpected end: {}", String::from_utf8(buf).unwrap()),
                        Err(err) => panic!("io error {:?}", err),
                    };
                }
                _ => panic!("error reading xml {:?}", err),
            },
        };
        return Some(item);
    }
}
