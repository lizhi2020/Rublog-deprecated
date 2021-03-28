use std::{env,fs,io};
use std::string::String;
use std::path::PathBuf;
use std::fs::File;
use lazy_static::*;
use regex::Regex;
use crate::webclient::get_file;
use crate::VERSION;
use error_chain::*;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        SystemTimeError(std::time::SystemTimeError);
        ReqwestError(reqwest::Error);
    }
}

pub fn update()->std::io::Result<()>{
    let target="https://raw.githubusercontent.com/lizhi2020/Rublog/main/templates/";
    let mut dst=env::current_exe()?;
    dst.pop();
    dst.push("templates");

    for item in ["home.html","about.html","index.html","post.html","prism-okaidia.css","style.css"].iter(){
        let mut url=String::from(target);
        url.push_str(item);
        let content = get_file(url.as_str())?;
        dst.push(item);

        fs::write(&dst, content)?;
        println!("{:?} <-> {}",dst,url);
        dst.pop();
    }
    Ok(())
}
pub fn upgrade()->io::Result<()>{
    let file=env::current_exe()?;
    let mut dst=PathBuf::from(&file);
    dst.pop();
    dst.push("temp");
    println!("rename to {}",dst.display());

    if dst.is_file() {
        fs::remove_file(&dst).unwrap();
    }
    fs::rename(&file,dst).expect("rename error");

    let response=reqwest::blocking::get("https://github.com/lizhi2020/Rublog/releases/latest").unwrap();
    //println!("{:?}",&response);
    lazy_static! {
        static ref RE: Regex=Regex::new(r".*tag/(?P<tag>.*)").unwrap();
    }

    let tag=RE.captures(response.url().path()).and_then(|cap| cap.name("tag")).unwrap();
    println!("find the last version:{}",&tag.as_str());
    if let std::cmp::Ordering::Equal = String::from(VERSION).cmp(&String::from(tag.as_str())){
        println!("you has been the latest");
        return Ok(())
    };
    let target="https://github.com/lizhi2020/Rublog/releases/download/";
    
    let mut target=String::from(target);
    target.push_str(tag.as_str());
    target.push_str("/rublog.exe");
    println!("{}",target);  
    
    let mut file=File::create(file).expect("create exe error");

    reqwest::blocking::get(target).unwrap().copy_to(&mut file).unwrap();

    Ok(())
}