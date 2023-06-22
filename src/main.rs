use std::{collections::HashMap, error::Error, hash::Hash, io, process};

use import_csv::{request::request::Promisse, DataRow, Entity, Fieldset, RequestBody, Root};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // if let Err(err) = example() {
    //     println!("error running example: {}", err);
    //     process::exit(1);
    // }
    let jsons = vec![generate_body(
        vec![(String::new(), String::new())],
        String::new(),
    )];
    let promisse = Promisse::new("https://jsonplaceholder.typicode.com/todos".to_string());
    let a = promisse.all::<Root, Root>(jsons).await;

    println!("{:?}", a.unwrap())
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(io::stdin());

    let mut headers: Vec<String> = vec![];
    {
        for ele in rdr.headers().expect("Cannot get headers") {
            headers.push(ele.to_string())
        }
    }

    let records = rdr.records();
    println!("{:?}", headers);
    for (i, result) in records.enumerate() {
        let a = format!("cannot get line:{}", i);
        let row = result.expect(a.as_str());
        println!("{:?}", row);
    }
    Ok(())
}

fn generate_body(tuples: Vec<(String, String)>, entity: String) -> Root {
    let mut h = HashMap::<String, HashMap<String, serde_json::Value>>::new();
    let mut fields: Vec<String> = vec![];
    for ele in tuples {
        let mut v = HashMap::<String, serde_json::Value>::new();
        let value = ele.0.to_string();
        let name = ele.1.to_string();
        fields.push(name.clone());
        v.insert("$".to_string(), serde_json::to_value(value).unwrap());
        h.insert(name, v);
    }
    Root {
        service_name: "CRUDServiceProvider.saveRecord".to_string(),
        request_body: RequestBody {
            data_set: import_csv::DataSet {
                root_entity: entity,
                include_presentation_fields: "S".to_string(),
                data_row: DataRow { local_fields: h },
                entity: Entity {
                    fieldset: Fieldset {
                        list: fields.join(",").to_string(),
                    },
                },
            },
        },
    }
}
