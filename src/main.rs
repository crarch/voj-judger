


use actix::*;





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
    let _master=Master::new().start();
    // 
    // let worker=Worker::new(master).start();

    // start console loop
    tokio::signal::ctrl_c().await.unwrap();
    
    
    

}


use actors::Master;
