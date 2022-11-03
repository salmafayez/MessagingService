use std::time::Duration;
use rdkafka::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord};
use crate::oracle_database::{Customer};
use crate::template_engine::create_message;

pub fn produce(brokers: &str, topic_name: &str, customers : &Vec<Customer>) {

    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");


    for customer in customers  {
        let message = create_message(&customer.name, &customer.phone);
        producer.send(
            BaseRecord::to(topic_name)
                .payload(&format!("{}", message))
                .key(&format!("Key {}", customer.name))
        ).expect("error sending");
    }
}