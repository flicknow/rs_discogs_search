use rs_discogs_search::artist::Artist;
use rs_discogs_search::label::Label;
use rs_discogs_search::master::Master;
use rs_discogs_search::release::Release;
use rs_discogs_search::Stream;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

fn count_xml<T: DeserializeOwned + Serialize>(mut stream: Stream<T>) {
    let mut last = now();
    let step = 10000;
    let mut count = 0;
    while let Some(item) = stream.next() {
        count += 1;

        if (count % step) == 0 {
            println!("{}", serde_json::to_string(&item).unwrap());

            let current = now();
            println!("> {} events in {}ms", step, current - last);

            last = current;
        }
    }
    println!("> saw {} records", count);
}

fn main() -> Result<(), quick_xml::Error> {
    let dump_type = "artists";
    let dump_date = "20240701";
    let url = format!(
        "https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_{}_{}.xml.gz",
        dump_date, dump_type
    );
    let stream: Stream<Artist> = Stream::new(&url);
    count_xml(stream);

    Ok(())
}
