use clap::{arg, command, value_parser, Command};

pub fn build_cli() -> Command {
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
}
