use crate::subcommands::{config, db, run, stream};

#[derive(Clone, Debug, clap::Parser)]
#[command(name = "eventify")]
#[command(about = "\
    Stream/Propagate events From L1/L2s \
    Index txs/blocks/events into a Postgres.
")]
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
