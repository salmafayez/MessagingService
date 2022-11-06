extern crate core;

use kafka_consumer::consume;

mod oracle_database;
mod kafka_producer;
mod kafka_consumer;
mod template_engine;

fn main(){
    consume(&"localhost:9092", &["campaigns"]);
}
