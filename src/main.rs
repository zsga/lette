use lette::{cmd, conf, log};

fn main() -> anyhow::Result<()> {
    let cnf = conf::new()?;
    cnf.db.save_env()?;

    let _guard = log::setup();

    let cli = cmd::new();

    cli.handle(&cnf)?;

    Ok(())
}
