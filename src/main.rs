use std::time::Duration;
use std::{io, thread};
use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,ClientBuilder
};
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};

mod env;

mod actors;

#[actix::main]
async fn main() {

    // let addr = WsClient::create(|ctx| {
    //     WsClient::add_stream(stream, ctx);
    //     WsClient{
    //         framed:Some(SinkWrite::new(sink, ctx))
    //     }
    // 
    // });
    // 
    let master=Master::new().start();
    // 
    // let worker=Worker::new(master).start();

    // start console loop
    tokio::signal::ctrl_c().await.unwrap();
    
    
    

}
use actors::WsClient;
use actors::Worker;
use actors::Master;
