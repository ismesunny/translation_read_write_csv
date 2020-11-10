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
}
struct JSONPointer {
    segments: Vec<String>,
    segments_ac: Vec<String>,
}

fn build_json_pointer(s: Vec<String>) -> JSONPointer {
    JSONPointer {
        segments: s
            .iter()
            .map(|x| {
                x.replace("_", "").replace("&", "").replace("%", "xx..xx")
                // .replace("xx..xx", "%")
            })
            .collect(),
        segments_ac: s
            .iter()
            .map(|x| x.replace("_", "").replace("xx..xx", "%"))
            .collect(),
    }
}

fn readcsv() -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();

    //read_data
    let mut rdr = csv::Reader::from_path("test.csv").unwrap();
    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result.unwrap();
        // r_msgid_rp.push(record["msgid"].clone());
        records.push(Record {
            msgid: record["msgid"].to_string(),
            msgid_plural: record["msgid_plural"].to_string(),
        });
    }
    records
}
fn writecsv(msg_str: Vec<String>, msg_p_str: Vec<String>) -> Result<(), Box<dyn Error>> {
    let p_ac = build_json_pointer(msg_str);

    // println!("after {:?}", p_ac.segments);
    println!("write xxx {:?}", p_ac.segments_ac);
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
    }
    let mut wtr = csv::Writer::from_path("output.csv")?;

    for (((((((a, b), c), d), e), f), g), h) in p_ac
        .segments_ac
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

    for j in records.iter() {
        data_msgid.push(j.msgid.to_string());
    }

    for i in records.iter() {
        data_msgid_p.push(i.msgid_plural.to_string());
    }
    println!("data {:?}", data_msgid);
    let p = build_json_pointer(data_msgid.clone());

    println!("after {:?}", p.segments);

    let mut store_msg = vec![];
    let mut store_msg_p = vec![];

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
                let arr = item.as_array();
                let result = match arr {
                    Some(values) => values
                        .iter()
                        .map(|s| s[0].as_str().unwrap())
                        .collect::<Vec<&str>>()
                        .join(" "),
                    None => String::from(""),
                };
                store_msg_p.push(result);
            }
            None => eprintln!("{}", color::red("Error...")),
        }
    }

    //loop translate msgid
    for j in p.segments.iter() {
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

    // let p_last = build_json_pointer(store_msg.clone());
    // println!("last pp {:?}", p_last.segments);

    writecsv(store_msg, store_msg_p).unwrap();
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
