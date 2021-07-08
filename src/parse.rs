use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use bson::Document;
use bson::doc;

#[derive(Debug)]
enum Wave{
    single((Vec<char>,String)),
    multi((Vec<char>,Vec<String>,String))
}

pub fn parse(input_vcd:&str)->Option<Document>{
    let input=File::open(input_vcd).unwrap();
    let mut lines=io::BufReader::new(input)
        .lines();
    
    let mut waves=HashMap::new();
    let mut clock:usize=0;
    
    let mut mismatch="".to_string();
    
    let mut order:Vec<String>=Vec::new();
    loop{
        if let Some(line)=lines.next(){
            if let Ok(line)=line{
                if(
                    line.starts_with("$date")
                    ||line.starts_with("$version")
                    ||line.starts_with("$timescale")
                ){
                    lines.next();
                    lines.next();
                }else if(line.starts_with("$var")){
                    let line_v:Vec<&str>=line.split(' ').collect();
                    
                    match line_v[2]{
                        "1"=>{
                            if(line_v[4]=="mismatch"){
                                mismatch=line_v[3].to_string();
                            }
                            order.push(line_v[3].to_string());
                            waves.insert(
                                line_v[3].to_string(),
                                Wave::single((Vec::new(),line_v[4].to_string())),
                            );
                        },
                        
                        multi=>{
                            order.push(line_v[3].to_string());
                            waves.insert(
                                line_v[3].to_string(),
                                Wave::multi((Vec::new(),Vec::new(),line_v[4].to_string())),
                            );
                        },
                    }
                    
                }else if(line.starts_with("$")){
                    ()
                }else{
                    match line.get(..1) {
                        Some("#") => {
                            clock=line.get(1..)
                                .unwrap()
                                .parse::<usize>()
                                .unwrap();
                            
                            //complement with .
                            
                            for (mark,wave) in &mut waves{
                                match wave{
                                    Wave::single((w,_))=>{
                                        loop{
                                            if(w.len()+1<clock){
                                                w.push('.');
                                            }else{
                                                break;
                                            }
                                        }
                                    },
                                    Wave::multi((w,_,_))=>{
                                        loop{
                                            if(w.len()+1<clock){
                                                w.push('.');
                                            }else{
                                                break;
                                            }
                                        }
                                    },
                                }
                            }
                        }
                        Some("x") => {
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::single((w,_)))=waves.get_mut(wire){
                                w.push('x');
                            }
                        },
                        Some("1") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::single((w,_)))=waves.get_mut(wire){
                                w.push('1');
                            }
                        },
                        
                        Some("0") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::single((w,_)))=waves.get_mut(wire){
                                w.push('0');
                            }
                        },
                        Some(x) => {
                            let line_v:Vec<&str>=line.split(' ').collect();
                            if let Some(Wave::multi((w,data,_)))=waves.get_mut(line_v[1]){
                                w.push('=');
                                data.push(line_v[0][1..].to_string());
                            }
                        },
                        _ => (),
                    }              
                }
            }
        }else{
            break;
        }
        
        
    }
    
    let mut i=0;
    let length;
    let value=waves.get(&mismatch).unwrap();
    match value{
        Wave::single((w,name))=>{
            length=w.len();
            while(i<length){
                if(w[i]=='1'){
                    break;
                }else{
                    i=i+1;
                }
            }
        },
        _=>{
            length=0
        },
    }
    
    if(i==length){
        return None;
    } 
    
    if(i>=20){
        i=i-20;
    }else{
        i=0;
    }
    
    let end;
    
    if(length>i+20){
        end=i+20;
    }else{
        end=length;
    }

    let mut signal:Vec<Document>=Vec::new();

    for key in order.iter(){
        if let Some(value)=waves.get(key){
            match value{
                Wave::single((w,name))=>{
                    let wave=doc!(
                        "name":name,
                        "wave":w[i..end].into_iter().collect::<String>()
                    );
                    signal.push(wave);
                },
                Wave::multi((w,words,name))=>{
                    let mut data=words[0].clone();
                    let mut words_iter=words.iter();
                    words_iter.next();
                    for iter in words_iter{
                        data=data+" "+iter;
                    }
                    let wave=doc!(
                        "name":name,
                        "wave":w[i..end].into_iter().collect::<String>(),
                        "data":data
                    );
                    signal.push(wave);
                },
            }
        }
    }
    
    let result=doc!{
        "head":doc!{"tock":1},
        "signal":signal,
    };
    
    return Some(result);
    
}
    
    
