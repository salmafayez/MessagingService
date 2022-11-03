use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, warn};
use oracle::Connection;
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use crate::kafka_producer::produce;
use crate::oracle_database::{getConnection, getCustomerPage, getCustomersCount, getNumberOfPages};

pub fn consume(brokers: &str, topics: &[&str]) {
    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "consumers")
        .set("enable.auto.commit", "true")
        .create()
        .expect("invalid consumer config");
    consumer
        .subscribe(topics)
        .expect("topic subscribe failed");

    loop {
        let optionalMessage = consumer.poll(Duration::from_secs(1));
        match optionalMessage {
            None =>{},
            Some(message)=> {
                match message {
                    Err(e) => warn!("Kafka error: {}", e),
                    Ok(m) => {
                        match m.payload_view::<str>() {
                            None => "",
                            Some(Ok(s)) => {
                                let connection: Arc<Mutex<Connection>> = Arc::new(Mutex::new(getConnection()));
                                let customersCount = getCustomersCount();
                                let numberOfPages = getNumberOfPages(&customersCount);

                                let mut thread_handles = vec![];

                                for i in (1..numberOfPages+1) {
                                    let connect = Arc::clone(&connection);
                                    thread_handles.push(
                                        thread::spawn( move || {
                                            println!("inside thread {}", i);
                                            let c = connect.lock().unwrap();
                                            let customers = getCustomerPage(&i, &c);
                                            produce(&"localhost:9092", &"offers", &customers);
                                        })
                                    );
                                }
                                for handle in thread_handles {
                                    handle.join().expect("No handler");
                                }
                                println!("{}", s);
                                s
                            },
                            Some(Err(e)) => {
                                warn!("Error while deserializing message payload: {:?}", e);
                                ""
                            }
                        };
                    }
                };
            }
        }
    }
}