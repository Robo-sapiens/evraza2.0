use std::collections::HashMap;

use futures::stream::FuturesUnordered;
use futures::{StreamExt, TryStreamExt};
use rdkafka::message::OwnedMessage;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::BorrowedMessage,
    ClientConfig, Message,
};
use serde::Deserialize;
use serde::Serialize;

// TODO 1. websocket
// 2. clickhouse
// 3. logger
// 4. separate files
#[tokio::main]
async fn main() {
    let num_workers = 3;

    (0..num_workers)
        .map(|_| tokio::spawn(stream_process()))
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await
}

async fn stream_process() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "Robo_sapiens")
        .set("sasl.username", "9433_reader")
        .set("sasl.password", "eUIpgWu0PWTJaTrjhjQD3.hoyhntiK")
        .set("sasl.mechanisms", "SCRAM-SHA-512")
        .set("security.protocol", "SASL_SSL")
        .set(
            "ssl.ca.location",
            "/usr/local/share/ca-certificates/Yandex/YandexCA.crt",
        )
        .set(
            "bootstrap.servers",
            "rc1a-b5e65f36lm3an1d5.mdb.yandexcloud.net:9091",
        )
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["zsmk-9433-dev-01"])
        .expect("Can't subscribe to specified topic");

    //    let mut exgausers_data = Vec::new();

    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        async move {
            save_borrowed_msg(&borrowed_message).await;
            let owned_msg = borrowed_message.detach();
            save_owned_msg(&owned_msg).await;
            let value = borrowed_message.payload().unwrap();
            let em: ExgausterMsg =
                serde_json::from_slice(value).expect("failed to deserialize JSON to ExgausterMsg");
            println!("EM: {:?}", em);
            //            println!("Size of em map: {}", &em.sm_exgausters.len());
            //save_exgauser(borrowed_message, exgausers_data);
            Ok(())
        }
    });
    stream_processor.await.expect("stream processing failed");
}

// TODO Pin?
//type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
//async fn save_exgauser(
//    borrowed_message: BorrowedMessage<'_>,
//    mut exgausers_data: Vec<ExgausterMsg>,
//) -> Result<()> {
//    save_borrowed_msg(&borrowed_message).await;
//    let owned_msg = borrowed_message.detach();
//    save_owned_msg(&owned_msg).await;
//    let value = borrowed_message.payload().unwrap();
//    let em: ExgausterMsg =
//        serde_json::from_slice(value).expect("failed to deserialize JSON to ExgausterMsg");
//    //            println!("EM: {:?}", em);
//    //            println!("Size of em map: {}", &em.sm_exgausters.len());
//    exgausers_data.push(em);
//    Ok(())
//}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExgausterMsg {
    moment: String,
    #[serde(flatten)]
    sm_exgausters: HashMap<String, f64>,
}

async fn save_owned_msg(msg: &OwnedMessage) {
    println!("Msg topic {}", msg.topic());
}

async fn save_borrowed_msg(msg: &BorrowedMessage<'_>) {
    println!("Message received: {}", msg.offset());
}
