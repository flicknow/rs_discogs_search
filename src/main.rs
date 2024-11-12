use rs_discogs_search::doc;
use rs_discogs_search::indexer::Indexer;
use rs_discogs_search::stream::Stream;
use serde_json::ser::to_string;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

async fn index_xml<T: 'static + doc::Doc>(mut stream: Stream<T>) {
    let indexer: Indexer<T> = Indexer::new(String::from("http://localhost:9200"));
    while let Some(doc) = stream.next().await {
        indexer.index(doc).await;
    }
}

async fn index_releases(mut stream: Stream<doc::release::Release>) {
    let indexer = Indexer::new(String::from("http://localhost:9200"));
    while let Some(release) = stream.next().await {
        let mut is_jazz = false;
        for genre in &release.genres.genre {
            if genre == "Jazz" {
                is_jazz = true;
                break;
            }
        }
        if !is_jazz {
            continue;
        };

        let master_id = &release.master_id;
        if (master_id.id > 0) && (!master_id.is_main_release) {
            continue;
        }

        indexer.index(release).await;
    }
}

async fn index_masters(mut stream: Stream<doc::master::Master>) {
    let indexer = Indexer::new(String::from("http://localhost:9200"));
    while let Some(master) = stream.next().await {
        let mut is_jazz = false;
        for genre in &master.genres.genre {
            if genre == "Jazz" {
                is_jazz = true;
                break;
            }
        }
        if !is_jazz {
            continue;
        };

        indexer.index(master).await;
    }
}

async fn count_xml<T: 'static + doc::Doc>(mut stream: Stream<T>) {
    let mut last = now();
    let step = 10000;
    let mut count = 0;

    let mut seen: Option<T> = None;
    while let Some(item) = stream.next().await {
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

#[tokio::main]
async fn main() -> Result<(), quick_xml::Error> {
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
        "masters" => index_masters(Stream::new(url)).await,
        "releases" => index_releases(Stream::new(url)).await,
        //"artists" => count_xml::<doc::artist::Artist>(Stream::new(url)).await,
        //"labels" => count_xml::<doc::label::Label>(Stream::new(url)).await,
        _ => (),
    };

    return Ok(());
}
