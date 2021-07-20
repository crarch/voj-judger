

use actix::Actor;

mod env;
mod actors;


use actors::Master;

#[actix::main]
async fn main() {

    let _master=Master::new().start();
    
    tokio::signal::ctrl_c().await.unwrap();

}
