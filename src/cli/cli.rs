pub struct Cli;

impl Cli {
    pub fn new() -> ReceivedArgs {
        let command = build_cli();
        let matches = command.clone().get_matches();

        let log_file = matches.get_one::<String>("log").unwrap().to_owned();
        let import_file = matches.get_one::<String>("name").unwrap().to_owned();
        let config_file = matches.get_one::<String>("config").unwrap().to_owned();
        let template_pedido = matches.get_one::<bool>("pedido").unwrap().to_owned();
        ReceivedArgs {
            config_file,
            log_file,
            import_file,
            template_pedido,
        }
    }
}
pub struct ReceivedArgs {
    pub log_file: String,
    pub config_file: String,
    pub import_file: String,
    pub template_pedido: bool,
}

use clap::{arg, command, value_parser, Command};

fn build_cli() -> Command {
    command!() // requires `cargo` feature
        .arg(arg!([name] "CSV file to import").required(true))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(String))
            .default_value("conf.json"),
        )
        .arg(
            arg!(
                -l --log <FILE> "Sets a log file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(String))
            .default_value("log.csv"),
        )
        .arg(
            arg!(
                -p --pedido "Usa template para pedido"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(bool))
            .default_value("false"),
        )
}
