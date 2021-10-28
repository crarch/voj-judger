use std::fs::File;
use std::io::{self,BufRead};

use std::collections::HashMap;

use bson::Document;
use bson::doc;
use bson::Bson;

#[derive(Debug)]
enum Wave{
    Single(Vec<char>),
    Multi((Vec<char>,Vec<String>))
}

pub fn parse(input_vcd:&str)->Option<Document>{
    let input=File::open(input_vcd).unwrap();
    let mut lines=io::BufReader::new(input)
        .lines();
    
    let mut waves=HashMap::new();
    
    let mut mapper=HashMap::new();
    
    let mut clock:usize=0;
    
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
                    
                    //mapper
                    match line_v[2]{
                        "1"=>{
                            order.push(line_v[4].to_string());
                            
                            mapper.insert(
                                line_v[4].to_string(),
                                line_v[3].to_string()
                            );
                            
                            waves.insert(
                                line_v[3].to_string(),
                                Wave::Single(Vec::new()),
                            );
                        },
                        
                        _multi=>{
                            order.push(line_v[4].to_string());
                            
                            mapper.insert(
                                line_v[4].to_string(),
                                line_v[3].to_string()
                            );
                            
                            waves.insert(
                                line_v[3].to_string(),
                                Wave::Multi((Vec::new(),Vec::new())),
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
                                    Wave::Single(w)=>{
                                        loop{
                                            if w.len()<clock {
                                                w.push('.');
                                            }else{
                                                break;
                                            }
                                        }
                                    },
                                    Wave::Multi((w,_))=>{
                                        loop{
                                            if w.len()<clock {
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
                            if let Some(Wave::Single(w))=waves.get_mut(wire){
                                w.push('x');
                            }
                        },
                        Some("1") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::Single(w))=waves.get_mut(wire){
                                w.push('1');
                            }
                        },
                        
                        Some("0") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::Single(w))=waves.get_mut(wire){
                                w.push('0');
                            }
                        },
                        
                        Some("z") => { 
                            let wire=line.get(1..).unwrap();
                            if let Some(Wave::Single(w))=waves.get_mut(wire){
                                w.push('z');
                            }
                        },
                        Some(_b) => {
                            let line_v:Vec<&str>=line.split(' ').collect();
                            if let Some(Wave::Multi((w,data)))=waves.get_mut(line_v[1]){
                                if(&line_v[0][..]=="bx"){
                                    w.push('x');
                                }else{
                                    w.push('=');
                                    data.push(binary_to_hex(&line_v[0][1..]));
                                }
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
    
    for (_mark,wave) in &mut waves{
        match wave{
            Wave::Single(w)=>{
                loop{
                    if w.len()<=clock {
                        w.push('.');
                    }else{
                        break;
                    }
                }
            },
            Wave::Multi((w,_))=>{
                loop{
                    if w.len()<=clock {
                        w.push('.');
                    }else{
                        break;
                    }
                }
            },
        }
    }

    
    let mut i=0;
    let length;
    
    let mismatch=mapper.get("mismatch").unwrap();
    
    let value=waves.get(mismatch).unwrap();
    
    match value{
        Wave::Single(w)=>{
            length=w.len();
            while i<length {
                if (w[i]!='0'&&w[i]!='.') {
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
    
    if i>=19 {
        i=i-19;
    }else{
        i=0;
    }
        
    
    let end;
    
    if length>i+20 {
        end=i+20;
    }else{
        end=length;
    }
        
    
    
    //avoid wave:.........
    for (_mark,wave) in &mut waves{
        match wave{
            Wave::Single(w)=>{
                let mut iter=i;
                while w[iter]=='.' {
                    iter=iter-1;
                }
                w[i]=w[iter];
            },
            Wave::Multi((w,_))=>{
                let mut iter=i;
                while w[iter]=='.' {
                    iter=iter-1;
                }
                w[i]=w[iter];
            },
        }
    }

    let mut signal:Vec<Bson>=Vec::new();
    
    let mut debug:Vec<Bson>=Vec::new();
    debug.push(bson::Bson::String("Debug".to_string()));
    
    let mut input:Vec<Bson>=Vec::new();
    input.push(bson::Bson::String("Input".to_string()));
    
    let mut reference:Vec<Bson>=Vec::new();
    reference.push(bson::Bson::String("Reference".to_string()));
    
    let mut yours:Vec<Bson>=Vec::new();
    yours.push(bson::Bson::String("Yours".to_string()));
    
    let mut mismatch=doc!{};
    
    let mut playground=true;

    for iter in order.iter(){
        let symbol=mapper.get(iter).unwrap();
        if let Some(value)=waves.get(symbol){
            match value{
                Wave::Single(w)=>{
                    
                    if iter.starts_with("y_") {
                        let name=(iter[2..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        yours.push(bson::Bson::Document(wave));
                    }else if iter.starts_with("r_") {
                        let name=(iter[2..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        reference.push(bson::Bson::Document(wave));
                        playground=false;
                    }else if iter.starts_with("i_") {
                        let name=(iter[2..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        input.push(bson::Bson::Document(wave));
                    }else if iter=="mismatch" {
                        let wave=doc!(
                            "name":iter.to_string(),
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        mismatch=wave;
                    }else{
                        let wave=doc!(
                            "name":iter.to_string(),
                            "wave":w[i..end].into_iter().collect::<String>()
                        );
                        debug.push(bson::Bson::Document(wave));
                    }
                },
                Wave::Multi((w,words))=>{
                    let mut data=String::new();
                    
                    if(words.len()!=0){
                        data=words[0].clone();
                        let mut words_iter=words.iter();
                        words_iter.next();
                        for iter in words_iter{
                            data=data+" "+iter;
                        }
                    }
                    
                    if iter.starts_with("y_") {
                        let name=(iter[2..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        yours.push(bson::Bson::Document(wave));
                    }else if iter.starts_with("r_") {
                        let name=(iter[2..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        reference.push(bson::Bson::Document(wave));
                    }else if iter.starts_with("i_") {
                        let name=(iter[2..]).to_string();
                        let wave=doc!(
                            "name":name,
                            "wave":w[i..end].into_iter().collect::<String>(),
                            "data":data
                        );
                        input.push(bson::Bson::Document(wave));
                    }else{
                        let name=iter.to_string();
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
    
    if debug.len()>1 {
        signal.push(bson::Bson::Array(debug));
        signal.push(bson::Bson::Document(doc!{}));
    }
    
    if input.len()>1 {
        signal.push(bson::Bson::Array(input));
        signal.push(bson::Bson::Document(doc!{}));
    }
    
    if !playground{
        
        signal.push(bson::Bson::Array(yours));
        signal.push(bson::Bson::Document(doc!{}));
        
        signal.push(bson::Bson::Array(reference));
        
        signal.push(bson::Bson::Document(mismatch));
    }
    
    let result=doc!{
        "head":doc!{"tock":i as u32},
        "signal":signal,
    };
    
    return Some(result);
    
}
    
    

fn binary_to_hex(val: &str) -> String {
    if let Ok(n) = u32::from_str_radix(val, 2){
        return format!("{:X}", n)
    }
    
    val.to_string()
}
