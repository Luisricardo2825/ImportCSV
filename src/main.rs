use std::process;
use std::time::Instant;

use console::{style, Emoji};
use import_csv::api::request::request::PromisseSankhya;
use import_csv::cli::cli::{Cli, ReceivedArgs};

use import_csv::core::spreedsheet::{SpreedsheetWorker, Types};
use import_csv::schemas::api::save_record::save_record_ret_schema::SaveRecordResponse;
use import_csv::schemas::api::save_record::save_record_schema::SaveRecord;
use import_csv::schemas::config::builder_config::EnvConfig;
use import_csv::ui::spinner::SpinnerBuilder;
use import_csv::utils::string_utils::get_external_json;
#[tokio::main]
async fn main() {
    let configs = Cli::new();
    let config_json = get_external_json(&configs.config_file);

    let env_config: EnvConfig = serde_json::from_str(&config_json).unwrap();

    let promisse = PromisseSankhya::new(env_config.clone()).await;
    if !&configs.template_pedido {
        generic_template(configs, &promisse, &env_config.entity).await
    } else {
        pedido_template(configs, &promisse, &env_config.entity).await
    }
}

fn generate_log(
    configs: &ReceivedArgs,
    save_all_results: &Vec<(i32, SaveRecordResponse)>,
) -> usize {
    let pb = SpinnerBuilder::new(style("Creating log file...").green().bold().to_string());
    let mut size: usize = save_all_results.len();

    let file_path = &configs.log_file;

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path(file_path)
        .expect("Could'nt create log");

    pb.set_message(style("Writing log file...").cyan().bold().to_string());
    let mut first = true;
    for (line, res) in save_all_results {
        if res.status_message.is_some() {
            if first {
                wtr.write_record(&["Line", "Error"])
                    .expect("Error writing headers");
                first = false;
            }
            size = size - 1;
            let row = [
                line.to_string(),
                res.status_message.as_ref().unwrap().clone(),
            ];
            match wtr.write_record(&row) {
                Err(err) => {
                    pb.finish(format!(
                        "{}{:?} : {}",
                        style("Error writing line: ").red().bold().to_string(),
                        row,
                        err
                    ));
                    panic!("Error writing log file")
                }
                Ok(()) => {}
            };
        } else {
            let mut row = vec![];
            if let Some(body) = res.response_body.clone() {
                if let Some(entities) = body.entities {
                    for ele in entities.entity {
                        row.push(ele.1);
                    }
                }
                if let Some(pk) = body.pk {
                    row.push(serde_json::to_value(pk).unwrap());
                }
            }

            if first {
                let mut headers: Vec<String> = vec![];
                row.clone().into_iter().for_each(|field| {
                    let bulk_row = field.as_object().unwrap();
                    bulk_row.keys().for_each(|x| headers.push(x.to_owned()))
                });
                wtr.write_record(headers).expect("Error writing headers");
                first = false;
            }

            for field in row {
                for (_, value) in field.as_object().unwrap() {
                    let val = value.get("$");
                    if val.is_some() {
                        let str_s = format!("{}", val.unwrap().as_str().unwrap());
                        wtr.write_field(str_s).expect("Error writing");
                    }
                }
                wtr.write_record(None::<&[u8]>).expect("Error writing");
            }
            // match wtr.write_record(&row) {
            //     Err(err) => {
            //         pb.finish(format!(
            //             "{}{:?} : {}",
            //             style("Error writing line: ").red().bold().to_string(),
            //             row,
            //             err
            //         ));
            //         panic!("Error writing log file")
            //     }
            //     Ok(()) => {}
            // };
        }
    }

    pb.finish(format!(
        "{} {file_path}",
        style("Log created at:").green().dim().bold().to_string()
    ));
    size
}

async fn pedido_template(configs: ReceivedArgs, promisse: &PromisseSankhya, entity: &String) {
    let results =
        SpreedsheetWorker::new(&configs.import_file, entity, Types::PEDIDO).read_spreedsheet_ped();

    if let Err(err) = results {
        println!("error running read_spreedsheet: {}", err);
        process::exit(1);
    }

    let jsons: Vec<import_csv::schemas::api::pedido::ped_vend_req_body::PedidoReqBody> =
        results.unwrap();

    let now = Instant::now();
    let save_all_results = match promisse.save_ped::<import_csv::schemas::api::pedido::ped_vend_req_body::PedidoReqBody, SaveRecordResponse>(jsons).await
    {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let size = generate_log(&configs, &save_all_results);

    let elapsed = now.elapsed();
    let total_with_error = save_all_results.len() - size;
    if total_with_error > 0 {
        println!(
            "{} {} {}",
            Emoji("❌", "🔴"),
            style("Error/warning in:").red().dim().bold().blink(),
            total_with_error
        )
    }

    println!("{} {:.2?}", style("Finished in:").green().dim(), elapsed);
}

async fn generic_template(configs: ReceivedArgs, promisse: &PromisseSankhya, entity: &String) {
    let results =
        SpreedsheetWorker::new(&configs.import_file, entity, Types::GENERIC).read_spreedsheet();
    if let Err(err) = results {
        println!("error running read_spreedsheet: {}", err);
        process::exit(1);
    }

    let jsons = results.unwrap();
    println!(
        "{} {} {}",
        Emoji("ℹ️", "*️⃣ "),
        style("Valid rows to import:").cyan().bold().dim(),
        &jsons.len()
    );

    let now = Instant::now();
    let save_all_results: Vec<(i32, SaveRecordResponse)> = match promisse
        .save_all::<SaveRecord, SaveRecordResponse>(jsons)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let size = generate_log(&configs, &save_all_results);

    let elapsed = now.elapsed();
    let total_with_error = save_all_results.len() - size;
    if total_with_error > 0 {
        println!(
            "{} {} {}",
            Emoji("❌", "🔴"),
            style("Error/warning in:").red().dim().bold().blink(),
            total_with_error
        )
    }

    println!("{} {:.2?}", style("Finished in:").green().dim(), elapsed);
}
