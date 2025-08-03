use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let data = Arc::new(vec!["hello", "world"]);
    let (tx, mut rx) = mpsc::channel(8);

    let producer = tokio::spawn({
        let data = Arc::clone(&data);
        async move {
            for item in data.iter() {
                tx.send(item.to_uppercase()).await.unwrap();
            }
        }
    });

    let mut results = Vec::new();
    while let Some(result) = rx.recv().await {
        results.push(result);
    }

    producer.await.unwrap();
    println!("Results: {:?}", results);
}
