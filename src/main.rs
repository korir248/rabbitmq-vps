// use futures_lite::stream::StreamExt;

// use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "amqp://127.0.0.1:5672";
//     let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
//     let channel = conn.create_channel().await?;

//     let mut consumer = channel
//         .basic_consume(
//             "video_processing",
//             "consumer",
//             BasicConsumeOptions::default(),
//             FieldTable::default(),
//         )
//         .await
//         .expect("basic_consume");

//     println!(" [*] Waiting for messages. To exit press CTRL+C");

//         while let Some(delivery) = consumer.next().await {
//             if let Ok(delivery) = delivery {
//                 println!(" [x] Received {:?}", std::str::from_utf8(&delivery.data)?);
//                 delivery
//                     .ack(BasicAckOptions::default())
//                     .await
//                     .expect("basic_ack");
//             }
//         }

//     Ok(())
// }

use futures_lite::stream::StreamExt;

use lapin::{options::BasicAckOptions, Result};
use rabbitmq_vps::AmqpHandler;

fn main() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap();

    async_global_executor::block_on(async {
        let amqp_handler = AmqpHandler::new(addr).await.unwrap();

        let mut consumer = amqp_handler.create_consumer().await.unwrap();

        async_global_executor::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.expect("error in consumer");
                let data = String::from_utf8(delivery.data.clone()).unwrap();
                delivery.ack(BasicAckOptions::default()).await.expect("ack");
                println!("Data from publisher: {:#?}", data);
                std::thread::sleep(std::time::Duration::from_secs(2))
            }
        })
        .detach();

        loop {
            std::thread::sleep(std::time::Duration::from_micros(2))
        }
    })
}
