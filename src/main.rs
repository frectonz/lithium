use anyhow::Result;

fn main() -> Result<()> {
    let manager = battery::Manager::new()?;

    for maybe_battery in manager.batteries()? {
        let battery = maybe_battery?;
        dbg!(battery);
    }

    Ok(())
}
