use std::fs::File;
use std::io::{self,BufRead};

use std::collections::HashMap;

use bson::Document;
use bson::doc;
use bson::Bson;

#[derive(Debug)]
enum Wave{
    Single((Vec<char>,String)),
    Multi((Vec<char>,Vec<String>,String))
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
                if line.starts_with("$date")
                    ||line.starts_with("$version")
                    ||line.starts_with("$timescale") {
                    lines.next();
                    lines.next();
                }else if line.starts_with("$var") {
                    let line_v:Vec<&str>=line.split(' ').collect();
                    
                    match line_v[2]{
                        "1"=>{
                            if line_v[4]=="mismatch" {
                                mismatch=line_v[3].to_string();
                            }
                            order.push(line_v[3].to_string());
                            waves.insert(
                                line_v[3].to_string(),
                                Wave::Single((Vec::new(),line_v[4].to_string())),
                            );
                        },
                        
                        _multi=>{
                            order.push(line_v[3].to_string());
                            waves.insert(
                                line_v[3].to_string(),
                                Wave::Multi((Vec::new(),Vec::new(),line_v[4].to_string())),
                            );
                        },
                    }
                    
                }else if line.starts_with("$") {
                    ()
                }else{
                    match line.get(..1) {
                        Some("#") => {
                            clock=line.get(1..)
                                .unwrap()
                                .parse::<usize>()
                                .unwrap();
                            
                            //complement with .
                            
                            for (_mark,wave) in &mut waves{
                                match wave{
                                    Wave::Single((w,_))=>{
                                        loop{
                                            if w.len()+1<clock {
                                                w.push('.');
                                            }else{
                                                break;
                                            }
                                        }
                                    },
                                    Wave::Multi((w,_,_))=>{
                                        loop{
                                            if w.len()+1<clock {
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
                            if let Some(Wave::Single((w,_)))=waves.get_mut(wire){
                                w.push('x');
                            }
                        },
                        Some("1") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::Single((w,_)))=waves.get_mut(wire){
                                w.push('1');
                            }
                        },
                        
                        Some("0") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::Single((w,_)))=waves.get_mut(wire){
                                w.push('0');
                            }
                        },
                        
                        Some("z") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::Single((w,_)))=waves.get_mut(wire){
                                w.push('z');
                            }
                        },
                        Some(_x) => {
                            let line_v:Vec<&str>=line.split(' ').collect();
                            if let Some(Wave::Multi((w,data,_)))=waves.get_mut(line_v[1]){
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
        Wave::Single((w,_name))=>{
            length=w.len();
            while i<length {
                if w[i]=='1' {
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
    
    if i==length {
        return None;
    } 
    
    if i>=20 {
        i=i-20;
    }else{
        i=0;
    }
    
    let end;
    
    if length>i+20 {
        end=i+20;
    }else{
        end=length;
    }
    
    

    let mut signal:Vec<Bson>=Vec::new();
    
    let mut debug:Vec<Bson>=Vec::new();
    debug.push(bson::Bson::String("Debug".to_string()));
    
    let mut reference:Vec<Bson>=Vec::new();
    reference.push(bson::Bson::String("Reference".to_string()));
    
    let mut yours:Vec<Bson>=Vec::new();
    yours.push(bson::Bson::String("Yours".to_string()));
    
    let mut mismatch=doc!{};

    for key in order.iter(){
        if let Some(value)=waves.get(key){
            match value{
                Wave::Single((w,name))=>{
                    if(name.starts_with("yours_")){
                        let name=(&name[6..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        yours.push(bson::Bson::Document(wave));
                    }else if(name.starts_with("ref_")){
                        let name=(&name[4..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        reference.push(bson::Bson::Document(wave));
                    }else if(name=="mismatch"){
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        mismatch=wave;
                    }else{
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        debug.push(bson::Bson::Document(wave));
                    }
                },
                Wave::Multi((w,words,name))=>{
                    let mut data=words[0].clone();
                    let mut words_iter=words.iter();
                    words_iter.next();
                    for iter in words_iter{
                        data=data+" "+iter;
                    }
                    if(name.starts_with("yours_")){
                        let name=(&name[6..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        yours.push(bson::Bson::Document(wave));
                    }else if(name.starts_with("ref_")){
                        let name=(&name[4..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        reference.push(bson::Bson::Document(wave));
                    }else if(name=="mismatch"){
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        mismatch=wave;
                    }else{
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        debug.push(bson::Bson::Document(wave));
                    }
                },
            }
        }
    }
    
    if(debug.len()>1){
        signal.push(bson::Bson::Array(debug));
        signal.push(bson::Bson::Document(doc!{}));
    }
    
    signal.push(bson::Bson::Array(yours));
    signal.push(bson::Bson::Document(doc!{}));
    
    signal.push(bson::Bson::Array(reference));
    
    signal.push(bson::Bson::Document(mismatch));
    
    let result=doc!{
        "head":doc!{"tock":i+1},
        "signal":signal,
    };
    
    return Some(result);
    
}
    
    
