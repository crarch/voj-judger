use actix::prelude::*;




use futures_util::stream::SplitSink;




use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use futures_util::SinkExt;
pub struct WsClient {
    pub framed:SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>,
}
    
struct Ping;

impl Actor for WsClient {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        self.framed.send(Message::Text("hihi".to_string()));
            
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
    }
}
        

use tokio_tungstenite::tungstenite::Message;



impl StreamHandler<Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>> for WsClient {
    fn handle(
        &mut self,
        message:Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>, 
        _ctx: &mut Context<Self>,
    ) {
        if let Ok(_message)=message{
            println!("ok");
            // match message{
            // 
            // 
            // 
            // }
        }

        // println!("{:?}",message);
    }
}


