use rssli::cli;

fn main() -> anyhow::Result<()> {
    log::debug!("Starting RSSLI");
    cli::cli_main()?;
    log::debug!("RSSLI finished");
    Ok(())
}
