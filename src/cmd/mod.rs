mod serve;

use clap::{Parser, Subcommand};

use crate::conf::Conf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    subcmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start HTTP server
    Serve(serve::ServeCmd),
}

pub fn new() -> Cli {
    Cli::parse()
}

impl Cli {
    pub fn handle(&self, conf: &Conf) -> anyhow::Result<()> {
        match &self.subcmd {
            Some(Commands::Serve(subcmd)) => {
                serve::handle(subcmd, conf)?;
            }
            None => {}
        }

        Ok(())
    }
}
