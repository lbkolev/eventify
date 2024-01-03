use crate::subcommands::{config, db, run, stream};

#[derive(Clone, Debug, clap::Parser)]
#[command(name = "eventify")]
#[command(about = "Index Ethereum into a Postgres database & serve it through an API.")]
#[command(color = clap::ColorChoice::Always)]
pub(crate) struct Cmd {
    #[command(subcommand)]
    pub(crate) subcmd: SubCommand,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum SubCommand {
    Run(run::Cmd),
    Stream(stream::Cmd),
    Db(db::Cmd),
    Config(config::Cmd),
}
