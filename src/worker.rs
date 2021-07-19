

use websocket_lite::{Message,Opcode};
use futures::stream::StreamExt;
use futures::SinkExt;

use bson::Bson;
use serde_json::Value;

use crate::{judge,parse_job};

use websocket_lite::{AsyncNetworkStream,AsyncClient};

pub async fn start(mut client:AsyncClient<Box<dyn AsyncNetworkStream + Sync + Send + Unpin + 'static>>){
    
    while let Some(Ok(message))=client.next().await{
        match message.opcode(){
            Opcode::Text=>{
                let data=message.as_text().unwrap();

                if let Some((job_id,code,question_id,user_id,submit_time))=parse_job(data).await{
                    println!("judging {}",&job_id);
                    let result=judge(job_id,code,question_id,user_id,submit_time).await.unwrap();
                    let result:Value=Bson::from(result).into();
                    let result=result.to_string();
                    client.send(Message::text(result)).await.unwrap(); 
                }

            },

            Opcode::Ping => client.send(Message::pong(message.into_data())).await.unwrap(),
            
            Opcode::Pong => client.send(Message::ping(message.into_data())).await.unwrap(),

            _=>(),

        }
    }
}    