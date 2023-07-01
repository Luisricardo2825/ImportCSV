use std::process;

use console::{style, Emoji};
use import_csv::api::request::request::PromisseSankhya;
use import_csv::cli::cli::{Cli, ReceivedArgs};

use import_csv::core::spreedsheet::SpreedsheetWorker;
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
    let results = SpreedsheetWorker::new(&configs.import_file, &env_config.entity);
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

    let promisse = PromisseSankhya::new(env_config.clone()).await;
    println!(
        "{} {}({},{},{})",
        Emoji("ℹ️", "*️⃣ "),
        style("Instanced PromisseSankhya with config:")
            .cyan()
            .bold()
            .dim(),
        env_config.entity,
        env_config.username,
        env_config.url
    );
    use std::time::Instant;
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

fn generate_log(
    configs: &ReceivedArgs,
    save_all_results: &Vec<(i32, SaveRecordResponse)>,
) -> usize {
    let pb = SpinnerBuilder::new(style("Creating log file...").green().bold().to_string());
    let mut size = save_all_results.len();

    let file_path = &configs.log_file;
    let mut wtr = csv::Writer::from_path(file_path).expect("Could'nt create log");

    pb.set_message(style("Writing log file...").cyan().bold().to_string());
    for (line, res) in save_all_results {
        if res.status_message.is_some() {
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
        }
    }

    pb.finish(format!(
        "{} {file_path}",
        style("Log created at:").green().dim().bold().to_string()
    ));
    size
}
