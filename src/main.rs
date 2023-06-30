use std::env;
use std::ffi::OsString;
use std::{collections::HashMap, error::Error, io, process};

use import_csv::request::request::PromisseSankhya;

use import_csv::schemas::login_ret_schema::AccessData;
use import_csv::schemas::save_record_ret_schema::SaveRecordResponse;
use import_csv::schemas::save_record_schema::{
    DataRow, DataSet, Entity, Fieldset, RequestBody, SaveRecord,
};
use import_csv::schemas::spreedsheet_schema::Spreedsheet;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let results = example();
    if let Err(err) = results {
        println!("error running example: {}", err);
        process::exit(1);
    }

    let jsons = results.unwrap();

    let promisse = PromisseSankhya::new(
        "http://sankhyaceara.nuvemdatacom.com.br:9199".to_string(),
        AccessData {
            password: "u#AdnwB4".to_string(),
            username: "sup".to_string(),
        },
    )
    .await;
    use std::time::Instant;
    let now = Instant::now();
    let a = match promisse
        .save_all::<SaveRecord, SaveRecordResponse>(jsons)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let mut size = a.len();

    let file_path = get_first_arg().unwrap();
    let mut wtr = csv::Writer::from_path(file_path).expect("Could'nt create log");
    for (line, res) in &a {
        if res.status_message.is_some() {
            size = size - 1;
            wtr.write_record(&[
                line.to_string(),
                res.status_message.as_ref().unwrap().clone(),
            ])
            .expect("Error writing log");
        }
    }
    let elapsed = now.elapsed();
    println!("Parallel elapsed: {:.2?}", elapsed);

    println!("Included: {}, error/warning in: {}", size, a.len() - size)
}

fn example() -> Result<Vec<SaveRecord>, Box<dyn Error>> {
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

    let records = rdr.deserialize::<Spreedsheet>();
    let mut tuples: Vec<(String, Value)> = vec![];
    let mut bodies = vec![];
    for (i, result) in records.enumerate() {
        let a = format!("cannot get line:{}", i);
        let row = result.expect(a.as_str());
        let object = serde_json::json!(row.local_fields);

        for (key, value) in object.as_object().unwrap() {
            tuples.push((key.to_string(), value.to_owned()));
        }
        bodies.push(generate_body(&tuples, "Parceiro".to_string()));
        tuples.clear();
    }

    Ok(bodies)
}

fn generate_body(tuples: &Vec<(String, Value)>, entity: String) -> SaveRecord {
    let mut local_fields = HashMap::<String, HashMap<String, HashMap<String, String>>>::new();
    let mut h = HashMap::<String, HashMap<String, String>>::new();
    let mut fields: Vec<String> = vec![];
    for ele in tuples {
        // println!("{}", &ele.0);
        let mut v = HashMap::<String, String>::new();
        let name = ele.0.to_string();
        let value = &ele.1;
        fields.push(name.clone());

        let can_be_f64 = match value.as_f64() {
            Some(_res) => true,
            None => false,
        };

        if can_be_f64 {
            v.insert("$".to_string(), format!("{}", value.as_f64().unwrap()));
        } else {
            v.insert("$".to_string(), value.as_str().unwrap().to_string());
        }

        h.insert(name, v);
    }
    local_fields.insert("localFields".to_string(), h);
    SaveRecord {
        service_name: "CRUDServiceProvider.saveRecord".to_string(),
        request_body: RequestBody {
            data_set: DataSet {
                root_entity: entity,
                include_presentation_fields: "S".to_string(),
                data_row: DataRow { local_fields },
                entity: Entity {
                    fieldset: Fieldset {
                        list: fields.join(",").to_string(),
                    },
                },
            },
        },
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Ok(OsString::from("log.csv")),
        Some(file_path) => Ok(file_path),
    }
}
