use crate::subcommands::{config, db, run};

#[derive(Clone, Debug, clap::Parser)]
#[command(name = "eventify")]
#[command(about = "Index Ethereum into a Postgres database & serve it through an API.")]
#[command(color = clap::ColorChoice::Always)]
pub(crate) struct Settings {
    #[command(subcommand)]
    pub(crate) cmd: SubCommand,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum SubCommand {
    Run(run::Cmd),
    Db(db::Cmd),
    Config(config::Cmd),
}
