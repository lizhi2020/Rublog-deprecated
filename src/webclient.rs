use std::io::Result;

use reqwest;
pub fn get_file(target:&str) ->Result<String>{
    Ok(reqwest::blocking::get(target).unwrap().text().unwrap())
}