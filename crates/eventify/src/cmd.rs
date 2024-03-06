use crate::subcommands::run;

#[derive(Clone, Debug, clap::Parser)]
#[command(name = "eventify")]
#[command(about = "Ledger event pipeline")]
#[command(color = clap::ColorChoice::Always)]
pub(crate) struct Cmd {
    #[command(subcommand)]
    pub(crate) subcmd: SubCommand,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub(crate) enum SubCommand {
    Run(Box<run::Cmd>),
}
