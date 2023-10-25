use std::collections::HashMap;

use console::style;
use csv::Error;
use serde_json::Value;

use crate::{
    schemas::{
        api::{
            pedido::ped_vend_req_body::PedidoReqBody, save_record::save_record_schema::SaveRecord,
        },
        spreedsheet::spreedsheet_schema::Spreedsheet,
    },
    ui::spinner::SpinnerBuilder,
};

use super::generate_body::{generate_body_generic, generate_body_pedido};
#[derive(PartialEq)]
pub enum Types {
    GENERIC,
    PEDIDO,
}
pub struct SpreedsheetWorker {
    pub csv_path: String,
    pub entity: String,
    pub types: Types,
}

impl SpreedsheetWorker {
    pub fn new<S: AsRef<str>>(csv_path: S, entity: S, sheet_type: Types) -> Self {
        let is_generic = Types::GENERIC == sheet_type;
        Self {
            csv_path: csv_path.as_ref().to_string(),
            entity: entity.as_ref().to_string(),
            types: match is_generic {
                true => Types::GENERIC,
                false => Types::PEDIDO,
            },
        }
    }
    fn read_file(&self) -> csv::Reader<std::fs::File> {
        csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_path(&self.csv_path)
            .expect("Cannot find file {csv_path}")
    }
    pub fn read_spreedsheet(&self) -> Result<Vec<SaveRecord>, Box<Error>> {
        let initial_msg = format!(
            "{} {}",
            style("[1/5] Received path:").green().bold(),
            &self.csv_path
        );
        let pb = SpinnerBuilder::new(initial_msg);
        let mut rdr = Self::read_file(&self);

        pb.set_message(
            style("[2/5] Processing file...")
                .bold()
                .cyan()
                .dim()
                .to_string(),
        );
        let mut headers: Vec<String> = vec![];
        {
            for ele in rdr.headers().expect("Cannot get headers") {
                headers.push(ele.to_string())
            }
        }

        let records = rdr.deserialize::<Spreedsheet>();
        pb.set_message(
            style("[3/5] Deserializing rows...")
                .bold()
                .cyan()
                .dim()
                .to_string(),
        );
        let mut tuples: Vec<(String, Value)> = vec![];
        let mut bodies: Vec<SaveRecord> = vec![];
        pb.set_message(
            style("[4/5] Creating tuples...")
                .bold()
                .cyan()
                .dim()
                .to_string(),
        );

        for (i, result) in records.enumerate() {
            let a = format!("cannot get line:{}", i);
            let row = result.expect(a.as_str());
            let object = serde_json::json!(row.local_fields);

            for (key, value) in object.as_object().unwrap() {
                tuples.push((key.to_string(), value.to_owned()));
            }

            let json_body = generate_body_generic(&tuples, self.entity.clone());
            bodies.push(json_body);

            tuples.clear();
        }

        pb.finish(
            style("[5/5] Tuples created")
                .bold()
                .green()
                .dim()
                .to_string(),
        );

        Ok(bodies)
    }
    pub fn read_spreedsheet_ped(&self) -> Result<Vec<PedidoReqBody>, Box<Error>> {
        let initial_msg = format!(
            "{} {}",
            style("[1/5] Received path:").green().bold(),
            &self.csv_path
        );
        let pb = SpinnerBuilder::new(initial_msg);
        let mut rdr = Self::read_file(&self);

        pb.set_message(
            style("[2/5] Processing file...")
                .bold()
                .cyan()
                .dim()
                .to_string(),
        );
        let mut headers: Vec<String> = vec![];
        {
            for ele in rdr.headers().expect("Cannot get headers") {
                headers.push(ele.to_string())
            }
        }

        let records = rdr.deserialize::<Spreedsheet>();
        pb.set_message(
            style("[3/5] Deserializing rows...")
                .bold()
                .cyan()
                .dim()
                .to_string(),
        );
        let mut bodies: Vec<PedidoReqBody> = vec![];
        pb.set_message(
            style("[4/5] Creating tuples...")
                .bold()
                .cyan()
                .dim()
                .to_string(),
        );
        let rec: Vec<Result<Spreedsheet, Error>> = records.into_iter().collect();
        let mut hash_tuples = HashMap::<String, Vec<(String, Value)>>::new();
        for (i, result) in rec.into_iter().enumerate() {
            let mut tuples: Vec<(String, Value)> = vec![];
            let a = format!("cannot get line:{}", i);
            let row = result.expect(a.as_str());
            let object = serde_json::json!(row.local_fields);
            let current_id = row
                .local_fields
                .get("@")
                .expect("Identificador \"@\" não encontraod0")
                .as_f64()
                .expect("O identificador deve ser um numero")
                .to_string();

            for (key, value) in object
                .as_object()
                .expect("Erro ao converte a planilha para JSON")
            {
                tuples.push((key.to_string(), value.to_owned()));
                if hash_tuples.contains_key(&current_id) {
                    let new_arr = hash_tuples.get_mut(&current_id).unwrap();
                    new_arr.push((key.to_string(), value.to_owned()));
                } else {
                    hash_tuples.insert(current_id.clone(), tuples.clone());
                }
            }

            // let same_id = id.eq_ignore_ascii_case(&current_id);

            tuples.clear();
        }
        // println!("{}", serde_json::to_string(&hash_tuples).unwrap());
        for (_, tuples) in hash_tuples {
            let json_body = generate_body_pedido(&tuples);
            bodies.push(json_body);
        }
        pb.finish(
            style("[5/5] Tuples created")
                .bold()
                .green()
                .dim()
                .to_string(),
        );
        Ok(bodies)
    }
    pub fn count(&self) -> CountResult {
        let mut rdr = Self::read_file(&self);
        let mut headers_count: i64 = 0;
        let mut rows_count: i64 = 0;
        for _ in rdr.headers().expect("Cannot get headers") {
            headers_count += 1;
        }

        let records = rdr.records();

        for _ in records.enumerate() {
            rows_count += 1;
        }

        CountResult {
            headers_count,
            rows_count,
        }
    }
}

pub struct CountResult {
    pub headers_count: i64,
    pub rows_count: i64,
}
