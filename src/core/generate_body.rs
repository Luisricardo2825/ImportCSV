use std::collections::HashMap;

use serde_json::Value;

use crate::schemas::api::save_record::save_record_schema::{
    DataRow, DataSet, Entity, Fieldset, RequestBody, SaveRecord,
};

pub fn generate_body(tuples: &Vec<(String, Value)>, entity: String) -> SaveRecord {
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
