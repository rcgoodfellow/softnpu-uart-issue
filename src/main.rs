use libfalcon::{cli::run, error::Error, Runner, unit::gb};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut d = Runner::new("softnpu_uart");

    // nodes
    let scrimlet = d.node("scrimlet", "helios-2.0", 4, gb(4));
    let gimlet = d.node("gimlet", "helios-2.0", 4, gb(4));

    // links
    d.softnpu_link(scrimlet, gimlet, None, None);

    // mounts
    d.mount("./cargo-bay", "/opt/cargo-bay", scrimlet)?;

    run(&mut d).await?;

    Ok(())
}
