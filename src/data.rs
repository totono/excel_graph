use std::env;
use std::error::Error;
use std::ffi::OsString;

use serde::Deserialize;


#[derive(Debug,Deserialize)]
pub struct Record {
    #[serde(rename(deserialize = "取得日時"))]
    pub time: String,
    #[serde(rename(deserialize = "空き率(%)"))]
    pub percent: f32,
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected 1 argument, but not none")),
        Some(file_path) => Ok(file_path),
    }
}

pub fn get_data() -> Result< Vec<Record>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        data.push(record);
    }
    Ok(data)
}


