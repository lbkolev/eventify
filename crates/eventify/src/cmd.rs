use crate::subcommands::{config, db, run};

#[derive(Clone, Debug, clap::Parser)]
#[command(name = "eventify")]
#[command(about = "\
    Stream/Propagate txs/blocks/events From L1/L2s | \
    Index txs/blocks/events into a Postgres.
")]
#[command(color = clap::ColorChoice::Always)]
pub(crate) struct Cmd {
    #[command(subcommand)]
    pub(crate) subcmd: SubCommand,

    #[arg(
        long = "only-migrations",
        env = "EVENTIFY_DB_MIGRATIONS",
        help = "Run only the database migrations and exit immediately after.",
        action
    )]
    pub(crate) only_migrations: bool,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum SubCommand {
    Run(Box<run::Cmd>),
    Db(db::Cmd),
    Config(config::Cmd),
}
