use lette::cmd;

fn main() -> anyhow::Result<()> {
    let cli = cmd::new();
    cli.handle()?;

    Ok(())
}
