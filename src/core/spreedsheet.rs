use console::style;
use serde_json::Value;

use std::error::Error;

use crate::{
    core::generate_body::generate_body,
    schemas::{
        api::save_record::save_record_schema::SaveRecord,
        spreedsheet::spreedsheet_schema::Spreedsheet,
    },
    ui::spinner::SpinnerBuilder,
};

pub struct SpreedsheetWorker {
    pub csv_path: String,
    pub entity: String,
}

impl SpreedsheetWorker {
    pub fn new<S: AsRef<str>>(csv_path: S, entity: S) -> Result<Vec<SaveRecord>, Box<dyn Error>> {
        let result = Self {
            csv_path: csv_path.as_ref().to_string(),
            entity: entity.as_ref().to_string(),
        };
        Self::read_spreedsheet(&result)
    }
    fn read_file(&self) -> csv::Reader<std::fs::File> {
        csv::ReaderBuilder::new()
            .delimiter(b';')
            .flexible(true)
            .from_path(&self.csv_path)
            .expect("Cannot find file {csv_path}")
    }
    fn read_spreedsheet(&self) -> Result<Vec<SaveRecord>, Box<dyn Error>> {
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
        let mut bodies = vec![];
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
            let json_body = generate_body(&tuples, self.entity.clone());
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
