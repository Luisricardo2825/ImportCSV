use std::collections::HashMap;

use serde_json::Value;

use crate::schemas::api::{
    pedido::ped_vend_req_body::PedidoReqBody,
    save_record::save_record_schema::{
        DataRow, DataSet, Entity, Fieldset, RequestBody, SaveRecord,
    },
};

pub fn generate_body_generic(tuples: &Vec<(String, Value)>, entity: String) -> SaveRecord {
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

pub fn generate_body_pedido(tuples: &Vec<(String, Value)>) -> PedidoReqBody {
    let ite: Vec<&(String, Value)> = tuples
        .into_iter()
        .filter(|x: &&(String, Value)| x.0.contains("@"))
        .collect();
    let cab: Vec<&(String, Value)> = tuples
        .into_iter()
        .filter(|x: &&(String, Value)| !x.0.contains("@") || (x.0.contains("@") && x.0.len() == 1))
        .collect();

    let mut itens = vec![];
    let mut item_fields = HashMap::<String, HashMap<String, String>>::new();
    let mut count = 0;
    let size = ite.len();
    for ele in ite {
        count += 1;
        let mut v = HashMap::<String, String>::new();
        let name = ele.0.to_string();
        let value = &ele.1;
        let can_be_f64 = match value.as_f64() {
            Some(_res) => true,
            None => false,
        };

        if can_be_f64 {
            v.insert("$".to_string(), format!("{}", value.as_f64().unwrap()));
        } else {
            let val_string = value.as_str().unwrap().to_string();
            if !val_string.is_empty() {
                v.insert("$".to_string(), val_string);
            }
        }
        if !(&name[1..]).to_string().is_empty() {
            item_fields.insert((&name[1..]).to_string(), v);
        }

        if name.eq("@") || count == size {
            if item_fields.keys().len() > 0 {
                itens.push(item_fields.clone());
            }
            item_fields.clear();
        }
    }

    let mut cab_hash = HashMap::<String, HashMap<String, String>>::new();
    for ele in cab {
        let mut v = HashMap::<String, String>::new();
        let name = ele.0.to_string();
        let value = &ele.1;
        let can_be_f64 = match value.as_f64() {
            Some(_res) => true,
            None => false,
        };
        if name.eq("@") {
            continue;
        }
        if can_be_f64 {
            v.insert("$".to_string(), format!("{}", value.as_f64().unwrap()));
        } else {
            let val_string = value.as_str().unwrap().to_string();
            if !val_string.is_empty() {
                v.insert("$".to_string(), val_string);
            }
        }
        if !(name).to_string().is_empty() {
            cab_hash.insert((name).to_string(), v);
        }
    }

    // println!("{}", serde_json::to_string(&cab_hash).unwrap());
    PedidoReqBody {
        service_name: "CACSP.incluirNota".to_string(),
        request_body: crate::schemas::api::pedido::ped_vend_req_body::RequestBody {
            nota: crate::schemas::api::pedido::ped_vend_req_body::Nota {
                cabecalho: cab_hash,
                itens: crate::schemas::api::pedido::ped_vend_req_body::Itens {
                    informarpreco: "False".to_string(),
                    item: itens,
                },
            },
        },
    }
}
