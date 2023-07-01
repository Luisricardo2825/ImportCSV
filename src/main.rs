use std::{collections::HashMap, error::Error, process};

use console::style;
use import_csv::request::request::PromisseSankhya;

use import_csv::schemas::builder_config::EnvConfig;
use import_csv::schemas::save_record_ret_schema::SaveRecordResponse;
use import_csv::schemas::save_record_schema::{
    DataRow, DataSet, Entity, Fieldset, RequestBody, SaveRecord,
};

use import_csv::schemas::spreedsheet_schema::Spreedsheet;
use import_csv::utils::resolve_param::build_cli;
use import_csv::utils::string_utils::get_external_json;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let configs = get_config_file();

    let config_json = get_external_json(&configs.config_file);

    let env_config: EnvConfig = serde_json::from_str(&config_json).unwrap();

    let results = read_spreedsheet(configs.import_file, &env_config.entity);
    if let Err(err) = results {
        println!("error running read_spreedsheet: {}", err);
        process::exit(1);
    }

    let jsons = results.unwrap();
    println!("Valid lines to import: {}", &jsons.len());

    let promisse = PromisseSankhya::new(env_config.clone()).await;
    println!(
        "Instanced PromisseSankhya with config:({},{},{})",
        env_config.entity, env_config.username, env_config.url
    );
    use std::time::Instant;
    let now = Instant::now();
    let save_all_results = match promisse
        .save_all::<SaveRecord, SaveRecordResponse>(jsons)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let mut size = save_all_results.len();

    let file_path = configs.log_file;
    println!("Creating log file...");
    let mut wtr = csv::Writer::from_path(file_path).expect("Could'nt create log");
    println!("Writing log file...");
    for (line, res) in &save_all_results {
        if res.status_message.is_some() {
            size = size - 1;
            wtr.write_record(&[
                line.to_string(),
                res.status_message.as_ref().unwrap().clone(),
            ])
            .expect("Error writing log");
        }
    }
    println!("Log file finished..");
    let elapsed = now.elapsed();
    println!("Parallel elapsed: {:.2?}", elapsed);

    println!(
        "Included: {}, error/warning in: {}",
        size,
        save_all_results.len() - size
    );
}

fn read_spreedsheet(csv_path: String, entity: &String) -> Result<Vec<SaveRecord>, Box<dyn Error>> {
    println!("{} {csv_path}",style("[0] Received path:").green().bold());
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(csv_path)
        .expect("Cannot find file {csv_path}");
    println!("{}", style("[1] Processing file...").bold().cyan().dim());

    let mut headers: Vec<String> = vec![];
    {
        for ele in rdr.headers().expect("Cannot get headers") {
            headers.push(ele.to_string())
        }
    }

    let records = rdr.deserialize::<Spreedsheet>();
    println!("{}", style("[2] Deserializing rows...").bold().cyan().dim());
    let mut tuples: Vec<(String, Value)> = vec![];
    let mut bodies = vec![];
    println!("{}",style("[3] Creating tuples...").bold().cyan().dim());

    for (i, result) in records.enumerate() {
        let a = format!("cannot get line:{}", i);
        let row = result.expect(a.as_str());
        let object = serde_json::json!(row.local_fields);

        for (key, value) in object.as_object().unwrap() {
            tuples.push((key.to_string(), value.to_owned()));
        }
        let json_body = generate_body(&tuples, entity.to_string());
        bodies.push(json_body);
        tuples.clear();
    }
    println!("[5] Tuples created");

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

fn get_config_file() -> ReceivedArgs {
    let command = build_cli();
    let matches = command.clone().get_matches();

    let log_file = matches.get_one::<String>("log").unwrap().to_owned();
    let import_file = matches.get_one::<String>("name").unwrap().to_owned();
    let config_file = matches.get_one::<String>("config").unwrap().to_owned();

    ReceivedArgs {
        config_file,
        log_file,
        import_file,
    }
}

struct ReceivedArgs {
    pub log_file: String,
    pub config_file: String,
    pub import_file: String,
}
