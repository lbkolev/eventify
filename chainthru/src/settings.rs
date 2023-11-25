pub(crate) mod config;
pub(crate) mod db;
pub(crate) mod run;

use clap::Parser;

#[derive(Clone, Debug, Parser)]
#[command(name = "Chainthru")]
#[command(about = "Index Ethereum into a Postgres database & serve it through an API.")]
pub(crate) struct Settings {
    #[command(subcommand)]
    pub(crate) cmd: SubCommand,

    #[arg(
        long = "log.level",
        env = "RUST_LOG",
        help = "The log level to use",
        default_value = "warn",
        value_parser = clap::value_parser!(log::Level),
    )]
    pub(crate) log_level: log::Level,
}

// Define an enum for subcommands
#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum SubCommand {
    Run(run::Cmd),
    Db(db::Cmd),
    Config(config::Cmd),
}
