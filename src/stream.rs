use crate::doc;
use flate2::read::GzDecoder;
use quick_xml::events::Event;
use quick_xml::DeError;
use reqwest::blocking::get;
use std::io::BufReader;
use std::io::Read;
use tokio::sync::mpsc;

pub struct Stream<T: doc::Doc> {
    channel: mpsc::Receiver<T>,
    pub handle: tokio::task::JoinHandle<()>,
}

impl<T: 'static + doc::Doc> Stream<T> {
    pub async fn next(&mut self) -> Option<T> {
        return self.channel.recv().await;
    }

    pub fn new(url: String) -> Stream<T> {
        let (tx, rx) = mpsc::channel::<T>(32);
        let handle = tokio::task::spawn_blocking(move || {
            let mut reader = Reader::new(&url);
            while let Some(doc) = reader.next::<T>() {
                tx.blocking_send(doc).unwrap();
            }
            return ();
        });

        return Stream {
            channel: rx,
            handle: handle,
        };
    }
}

pub struct Reader {
    start: String,
    stream: BufReader<GzDecoder<reqwest::blocking::Response>>,
}

impl Reader {
    pub fn new(url: &str) -> Reader {
        let response = get(url).unwrap();
        let decoder = GzDecoder::new(response);
        let mut reader = quick_xml::reader::Reader::from_reader(BufReader::new(decoder));

        let mut buf = Vec::new();
        let start = match reader.read_event_into(&mut buf) {
            Ok(Event::Start(start)) => start,
            Ok(event) => panic!("unexpected start tag {:?}", event),
            Err(err) => panic!("error reading xml start tag {}", err),
        };

        let name = std::str::from_utf8(start.name().into_inner())
            .unwrap()
            .to_string();
        return Reader {
            start: name,
            stream: reader.into_inner(),
        };
    }

    pub fn next<T: doc::Doc>(&mut self) -> Option<T> {
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
