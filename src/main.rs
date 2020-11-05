use csv;
use csv::WriterBuilder;
use reqwest;
use seahorse::color;
use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::process::exit;

#[derive(Debug, Serialize, Clone)]
struct Record {
    msgid: String,
}
#[derive(Debug, Serialize)]
struct RecordWrite {
    msgstr: String,
}
fn readcsv() -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    //read_data
    let mut rdr = csv::Reader::from_path("data.csv").unwrap();
    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result.unwrap();
        records.push(Record {
            msgid: record["msgid"].to_string(),
        });
    }
    records
}
fn writecsv(msg_str: Vec<String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().append(true).open("data.csv")?;
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);
    //let mut wtr = csv::Writer::from_path(file_path).unwrap();

    //let mut wtr = csv::Writer::from_path("data.")?;
    for msgstr_text in msg_str.iter() {
        println!("Write Word {}", msgstr_text);
        wtr.serialize(RecordWrite {
            msgstr: msgstr_text.to_string(),
        })?;
    }
    wtr.flush()?;
    Ok(())
}
fn main() {
    let records = readcsv();
    let mut data: Vec<String> = Vec::new();
    for i in records.iter() {
        data.push(i.msgid.to_string());
    }

    let mut store = vec![];
    for i in data.iter() {
        std::thread::sleep_ms(15000); //set milli second for loop translate
        let source = String::from("en"); //source language
        let target = String::from("km"); //target language
        let url = generate_url(i.to_string(), source, target);
        let v = reqwest::blocking::get(&url)
            .and_then(|resp| resp.text())
            .and_then(|body| Ok(serde_json::from_str::<Vec<Value>>(&body)))
            .unwrap_or_else(|_| {
                eprintln!(
                    "{}",
                    color::red("network error! please connect to your network...")
                );
                exit(1);
            })
            .unwrap_or_else(|_| {
                eprintln!("{}", color::red("translation parse error..."));
                exit(1);
            });
        match v.first() {
            Some(item) => {
                let result = item
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s[0].as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" ");

                store.push(result);
            }
            None => eprintln!("{}", color::red("Error...")),
        }
    }
    //println!("last {:?}", store);
    writecsv(store.clone()).unwrap();
}

fn generate_url(v: String, source: String, target: String) -> String {
    let base_url = "https://translate.googleapis.com/translate_a/single";
    format!(
        "{}{}{}{}{}",
        base_url,
        "?client=gtx&ie=UTF-8&oe=UTF-8&dt=t",
        format!("{}{}", "&sl=", source),
        format!("{}{}", "&tl=", target),
        format!("&q={}", v).to_string()
    )
}
