use std::collections::HashMap;
use std::net::SocketAddr;

use clickhouse::{Client, Row};
use futures::stream::FuturesUnordered;
use futures::{SinkExt, StreamExt, TryStreamExt};
use rdkafka::message::OwnedMessage;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::BorrowedMessage,
    ClientConfig, Message,
};
use serde::Deserialize;
use serde::Serialize;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};

// TODO 1. websocket
// 2. clickhouse
// 3. logger
// 4. separate files

// TODO Pin?
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize, Debug, Clone, Row)]
struct ExgausterMsg {
    moment: String,
    #[serde(flatten)]
    sm_exgausters: HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    let num_workers = 3;

    (0..num_workers)
        .map(|_| tokio::spawn(stream_process()))
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await;

    let addr = "";
    let listener = TcpListener::bind(&addr).await.expect("can't listen");

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connect streams should have a peer address");

        tokio::spawn(accept_connection(peer, stream));
    }
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

    let client = create_client();
    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let cl = client.clone();
        async move {
            save_borrowed_msg(&borrowed_message).await;
            let owned_msg = borrowed_message.detach();
            save_owned_msg(&owned_msg).await;
            let value = borrowed_message.payload().unwrap();
            let em: ExgausterMsg =
                serde_json::from_slice(value).expect("failed to deserialize JSON to ExgausterMsg");
            //        println!("EM: {:?}", &em);

            let mut insert = cl.insert("table_name").unwrap();
            insert.write(&em).await.expect("can't insert into table");

            insert.end().await.unwrap();
            //            println!("Size of em map: {}", &em.sm_exgausters.len());
            Ok(())
        }
    });
    stream_processor.await.expect("stream processing failed");
}

async fn save_owned_msg(msg: &OwnedMessage) {
    println!("Msg topic {}", msg.topic());
}

async fn save_borrowed_msg(msg: &BorrowedMessage<'_>) {
    println!("Message received: {}", msg.offset());
}

fn create_client() -> Client {
    Client::default()
        .with_url("")
        .with_user("")
        .with_database("")
}

async fn get_exgauster_with_date(client: Client, date: String) -> Result<Vec<ExgausterMsg>> {
    let mut cursor = client
        .query("SELECT ?fields FROM table_name WHERE ?")
        .bind(date)
        .fetch::<ExgausterMsg>()?;

    let mut exgausters = Vec::new();
    while let Some(row) = cursor.next().await? {
        exgausters.push(row);
    }
    Ok(exgausters)
}

async fn get_all(client: Client) -> Result<Vec<ExgausterMsg>> {
    let mut cursor = client
        .query("SELECT ?fields FROM table_name")
        .fetch::<ExgausterMsg>()?;

    let mut exgausters = Vec::new();
    while let Some(row) = cursor.next().await? {
        exgausters.push(row);
    }
    Ok(exgausters)
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("ERROR PROCESSING CONNECTION: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> tungstenite::Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("failed to accept");

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}
