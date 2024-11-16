use crate::doc::Doc;
use opensearch::http::transport::Transport;
use opensearch::OpenSearch;
use tokio::sync::mpsc;

pub struct Indexer<T: Doc> {
    pub channel: mpsc::Sender<T>,
    pub handle: tokio::task::JoinHandle<()>,
}

impl<T: 'static + Doc> Indexer<T> {
    pub async fn index(&self, doc: T) {
        self.channel.send(doc).await.unwrap();
        return ();
    }

    pub fn new(url: String) -> Self {
        let (tx, mut rx) = mpsc::channel::<T>(32);
        let handle = tokio::spawn(async move {
            let worker = IndexerWorker::new(&url);
            while let Some(doc) = rx.recv().await {
                worker.index(&doc).await;
            }
            return ();
        });

        return Self {
            channel: tx,
            handle: handle,
        };
    }
}

struct IndexerWorker {
    client: OpenSearch,
}

impl IndexerWorker {
    pub fn new(url: &str) -> Self {
        let transport = Transport::single_node(url).unwrap();
        let client = OpenSearch::new(transport);
        return Self { client };
    }
    pub async fn index<T: Doc>(&self, doc: &T) {
        let res = self
            .client
            .index(doc.index_id())
            .body(doc)
            .send()
            .await
            .unwrap();

        let status_code = &res.status_code();
        if !status_code.is_success() {
            let status_code = status_code.as_u16();
            let headers = format!("{:?}", res.headers());
            let text = res.text().await.unwrap();
            panic!("{} {} {}", status_code, headers, text);
        }
        return ();
    }
}
