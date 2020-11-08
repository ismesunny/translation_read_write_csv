use csv;
use reqwest;
use seahorse::color;
use serde::Serialize;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::error::Error;
use std::process::exit;

#[derive(Debug, Serialize, Clone)]
struct Record {
    msgid: String,
    msgid_plural: String,
}
#[derive(Debug, Serialize)]
struct RecordWrite {
    msgid: String,
    msgid_plural: String,
    flags: String,
    references: String,
    #[serde(rename = "extractedComments")]
    extracted_comments: String,
    comments: String,
    #[serde(rename = "msgstr[0]")]
    msgstr0: String,
    #[serde(rename = "msgstr[1]")]
    msgstr1: String,
    // #[serde(rename = "msgstr[2]")]
    // msgstr2: String,
    // #[serde(rename = "msgstr[3]")]
    // msgstr3: String,
}

fn readcsv() -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    //read_data
    let mut rdr = csv::Reader::from_path("test.csv").unwrap();
    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result.unwrap();
        records.push(Record {
            msgid: record["msgid"].to_string(),
            msgid_plural: record["msgid_plural"].to_string(),
        });
    }
    records
}
fn writecsv(msg_str: Vec<String>, msg_p_str: Vec<String>) -> Result<(), Box<dyn Error>> {
    //read
    let mut rdr = csv::Reader::from_path("test.csv")?;
    let mut w_msgid = vec![];
    let mut w_msgid_plural = vec![];
    let mut w_flags = vec![];
    let mut w_references = vec![];
    let mut w_extracted_comments = vec![];
    let mut w_comments = vec![];
    let mut w_msgstr = vec![];

    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result?;
        w_msgid.push(record["msgid"].clone());
        w_msgid_plural.push(record["msgid_plural"].clone());
        w_flags.push(record["flags"].clone());
        w_references.push(record["references"].clone());
        w_extracted_comments.push(record["extractedComments"].clone());
        w_comments.push(record["comments"].clone());
        w_msgstr.push(record["msgstr[0]"].clone());
        // w_msgstr.push(!record["msgstr[0]"].clone().is_empty());
    }
    let mut wtr = csv::Writer::from_path("output.csv")?;

    for (((((((a, b), c), d), e), f), g), h) in msg_str
        .iter()
        .zip(w_msgid.clone())
        .zip(w_msgid_plural.clone())
        .zip(w_flags.clone())
        .zip(w_references.clone())
        .zip(w_extracted_comments.clone())
        .zip(w_comments.clone())
        .zip(msg_p_str.clone())
    {
        wtr.serialize(RecordWrite {
            msgid: b.to_string(),
            msgid_plural: c.to_string(),
            flags: d.to_string(),
            references: e.to_string(),
            extracted_comments: f.to_string(),
            comments: g.to_string(),
            msgstr0: a.to_string(),
            msgstr1: h.to_string(), //msgstr2
                                    // msgstr3
        })?;
    }
    wtr.flush()?;
    Ok(())
}
fn main() {
    let records = readcsv();
    let mut data_msgid: Vec<String> = Vec::new();
    let mut data_msgid_p: Vec<String> = Vec::new();
    let source = String::from("en"); //source language
    let target = String::from("km"); //target language

    for i in records.iter() {
        data_msgid.push(i.msgid.to_string());
    }
    for j in records.iter() {
        data_msgid_p.push(j.msgid_plural.to_string());
    }

    let mut store_msg = vec![];
    let mut store_msg_p = vec![];

    //loop translate msgid_plural
    for i in data_msgid_p.iter() {
        std::thread::sleep(std::time::Duration::from_secs(10)); //set milli second for loop translate

        let url = translation(i.to_string(), source.clone(), target.clone());
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

                store_msg_p.push(result);
            }
            None => eprintln!("{}", color::red("Error...")),
        }
    }
    //loop translate msgid
    for j in data_msgid.iter() {
        std::thread::sleep(std::time::Duration::from_secs(10)); //set milli second for loop translate

        let url = translation(j.to_string(), source.clone(), target.clone());
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

                store_msg.push(result);
            }
            None => eprintln!("{}", color::red("Error...")),
        }
    }

    println!("last {:?}", store_msg);
    println!("last 22 {:?}", store_msg_p);
    writecsv(store_msg.clone(), store_msg_p.clone()).unwrap();
}

fn translation(v: String, source: String, target: String) -> String {
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
