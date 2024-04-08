use crate::{conf::Conf, router};
use clap::Args;

#[derive(Args, Debug)]
pub struct ServeCmd {
    /// Server port to listen on
    #[arg(short, long, value_name = "PORT", default_value = "5000")]
    port: Option<u16>,
}

pub fn handle(cmd: &ServeCmd, conf: &Conf) -> anyhow::Result<()> {
    let port = cmd.port.unwrap_or(conf.app.port);
    start_server(conf, port)?;
    Ok(())
}

fn start_server(conf: &Conf, port: u16) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move { router::serve(conf, port).await })?;

    std::process::exit(0)
}
