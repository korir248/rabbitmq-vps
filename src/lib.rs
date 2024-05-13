// use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection};
use lapin::{
    options::{BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Connection, ConnectionProperties, Consumer, Result,
};

pub struct AmqpHandler {
    // pub pool: Pool<AsyncPgConnection>,
    connection: Connection,
}

impl AmqpHandler {
    pub async fn new(addr: String) -> Result<Self> {
        let connection: Connection =
            Connection::connect(&addr, ConnectionProperties::default()).await?;
        println!("CONNECTED");

        let channel = connection.create_channel().await?;

        channel
            .queue_declare(
                "video_processing",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(Self { connection })
    }

    pub async fn create_consumer(&self) -> Result<Consumer> {
        let channel = self.connection.create_channel().await?;

        let consumer = channel
            .basic_consume(
                "video_processing",
                "video_processing_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(consumer)
    }

    // pub async fn convert_video(video_url: String) {}
}
