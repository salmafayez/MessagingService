use kafka_consumer::consume_and_print;

#[macro_use]
extern crate rbatis;
extern crate rbdc_oracle;

mod oracle_database;
mod kafka_producer;
mod kafka_consumer;
mod template_engine;

#[tokio::main]
async fn main(){
    consume_and_print(&"localhost:9092", &["rust"]).await
}
