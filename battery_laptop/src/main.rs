use battery::Manager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = Manager::new()?;
    let batteries = manager.batteries()?;

    for (idx, maybe_battery) in batteries.enumerate() {
        let battery = maybe_battery?;
        println!("Battery {idx}:");
        println!("  State: {:?}", battery.state());
        println!("  Charge: {:.2}%", battery.state_of_charge().value * 100.0);
        println!("  Energy: {:.2} Wh", battery.energy().value);
        println!("  Energy full: {:.2} Wh", battery.energy_full().value);
        println!("  Time to full: {:?}", battery.time_to_full());
        println!("  Time to empty: {:?}", battery.time_to_empty());
    }

    Ok(())
}
