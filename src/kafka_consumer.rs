// use core::slice::SlicePattern;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use log::{info, warn};
use oracle::Connection;
use oracle::pool::{PoolBuilder, PoolType};
use rdkafka::{ClientConfig, ClientContext, Message, TopicPartitionList};
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{BaseConsumer, CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer};
use rdkafka::error::KafkaResult;
use threadpool::ThreadPool;
use crate::kafka_producer::produce;
use crate::oracle_database::{getConnection, getCustomerPage, getCustomersCount, getNumberOfPages};
// use std::collections::vec_deque::ring_slices::RingSlices;


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

    let connection_pool = PoolBuilder::new("poc", "poc", "//localhost:1521/orclpdb1")
        .pool_type(PoolType::Heterogeneous)
        .max_connections(20)
        .build().expect("error in creating the pool");

    let thread_pool = ThreadPool::new(20);


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
                                let customersCount = getCustomersCount();
                                let numberOfPages = getNumberOfPages(&customersCount);

                                for i in (0..numberOfPages) {
                                    let pool = connection_pool.clone();
                                    thread_pool.execute( move || {
                                        println!("thread number {:?}", thread::current().id());
                                        let c = pool.get().unwrap();
                                        let customers = getCustomerPage(&i, &c);
                                        produce(&"localhost:9092", &"messages", &customers);
                                    });
                                }
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