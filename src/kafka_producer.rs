use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use crate::oracle_database::CvmUser;
use crate::template_engine::create_message;

pub async fn produce(brokers: &str, topic_name: &str, users : &Vec<CvmUser>) {

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    for user in users  {
        let message = create_message(&user.name, &user.phone);
        producer.send(
            FutureRecord::to(topic_name)
                .payload(&format!("Message {}", message))
                .key(&format!("Key {}", user.name))
            ,
            Duration::from_secs(0),
        )
            .await.expect("Error in sending the message");
    }
}