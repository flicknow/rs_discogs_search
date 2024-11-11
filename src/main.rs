use rs_discogs_search::artist::Artist;
use rs_discogs_search::label::Label;
use rs_discogs_search::master::Master;
use rs_discogs_search::release::Release;
use rs_discogs_search::stream::Stream;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::ser::to_string;
use std::env;
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

fn count_xml<T: DeserializeOwned + Serialize + Debug>(mut stream: Stream) {
    let mut last = now();
    let step = 10000;
    let mut count = 0;

    let mut seen: Option<T> = None;
    while let Some(item) = stream.next::<T>() {
        count += 1;

        if (count % step) == 0 {
            //println!("{}", serde_json::to_string(&item).unwrap());

            let current = now();
            println!("> {} events in {}ms", step, current - last);

            last = current;
        }

        match seen {
            None => println!("> first saw {}", to_string(&item).unwrap()),
            _ => (),
        }
        seen = Some(item);
    }
    println!("> saw {} total records", count);
    println!("> last saw {}", to_string(&seen).unwrap());
}

fn main() -> Result<(), quick_xml::Error> {
    let args: Vec<String> = env::args().collect();
    let dump_type = match args.get(1) {
        Some(dump_type) => match dump_type.as_str() {
            "artists" | "labels" | "masters" | "releases" => dump_type,
            _ => panic!("unexpected rs_discogs_search TYPE {}", dump_type),
        },
        None => panic!("USAGE: rs_discogs_search TYPE"),
    };

    let dump_date = "20240701";
    let url = format!(
        "https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_{}_{}.xml.gz",
        dump_date, dump_type
    );
    match dump_type.as_str() {
        "artists" => count_xml::<Artist>(Stream::new(&url)),
        "labels" => count_xml::<Label>(Stream::new(&url)),
        "masters" => count_xml::<Master>(Stream::new(&url)),
        "releases" => count_xml::<Release>(Stream::new(&url)),
        _ => (),
    };

    Ok(())
}
