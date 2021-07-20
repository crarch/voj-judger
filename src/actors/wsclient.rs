use actix::prelude::*;




use futures_util::stream::SplitSink;




use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;
pub struct WsClient ;//{
    // pub framed:SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>,
// }
    

impl Actor for WsClient {
    type Context = Context<Self>;

    // fn started(&mut self, ctx: &mut Context<Self>) {
    // 
    //     let ws_url="ws".to_string()+&get_env("API_URL")[4..]+"/websocket";
    //     let judger_key=get_env("JUDGER_KEY");
    // 
    //     let header=httparse::Header{
    //         name:"Authorization",
    //         value:&judger_key.as_bytes(),
    //     };
    // 
    //     let mut headers = vec!(header);
    //     let request=httparse::Request{
    //         method:Some("GET"),
    //         path:Some(&ws_url),
    //         version:Some(1),
    //         headers:&mut headers,
    //     };
    // 
    //     let fut=connect_async(request);
    // 
    //     let (ws_stream,_)=futures::executor::block_on(async {
    //         fut.await.unwrap()
    //     });
    // 
    //     let (tx,mut rx)=ws_stream.split();
    // }
    // 
    // fn stopped(&mut self, _: &mut Context<Self>) {
    // }
}
        

use tokio_tungstenite::tungstenite::Message;



// impl StreamHandler<Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>> for WsClient {
// 
//     fn handle(
//         &mut self,
//         message:Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>, 
//         ctx: &mut Context<Self>,
//     ) {
//             if let Ok(message)=message{
//                 println!("{}",&message);
//                 match(message){
//                     Message::Text(job)=>{
//                     },
//                     Message::Ping(x)=>{
//                         self.send(Message::Pong(Vec::new()));
//                     },
//                     Message::Pong(x)=>{
//                         self.send(Message::Ping(Vec::new()));
//                     },
//                     _=>(),
//                 }
//             }
// 
//     }
// }
// 
// 
// impl WsClient{
//     fn send(&mut self,msg:Message){
// 
//         let fut = self.framed.send(msg);
//         futures::executor::block_on(async {
//             let _ = fut.await;
//             println!("ok");
//         });
// 
//     }
// }
use actix::StreamHandler;
use futures_util::stream::once;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use crate::get_env;