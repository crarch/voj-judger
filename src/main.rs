use std::time::Duration;
use std::{io, thread};
use actix::Actor;

mod env;
mod actors;
use actors::WsClient;
use actors::Worker;
use actors::Master;

mod judge;

#[actix::main]
async fn main() {

    let master=Master::new().start();
    
    tokio::signal::ctrl_c().await.unwrap();

}
