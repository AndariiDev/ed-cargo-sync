use std::io::{self, Write};

fn main() {
    let steel_needed = read_input_u16("Enter steel amount needed: ");
    let titanium_needed = read_input_u16("Enter titanium amount needed: ");
    let ship_capacity = read_input_u16("Enter ship cargo capacity: ");

    let (steel_per_trip, titanium_per_trip) =
        calculate_cargo_split(steel_needed, titanium_needed, ship_capacity);

    println!(
        "Deliver {} steel and {} titanium this trip.",
        steel_per_trip, titanium_per_trip
    );
}

fn calculate_cargo_split(
    steel_needed: u16,
    titanium_needed: u16,
    ship_capacity: u16,
) -> (u16, u16) {
    let total_needed = steel_needed as f32 + titanium_needed as f32;

    if total_needed == 0.0 {
        return (0, 0);
    }

    let steel_ratio = (steel_needed as f32 * 1000.0) / total_needed;
    let titanium_ratio = (titanium_needed as f32 * 1000.0) / total_needed;

    let steel_per_trip = (ship_capacity as f32 * steel_ratio) / 1000.0;
    let titanium_per_trip = (ship_capacity as f32 * titanium_ratio) / 1000.0;

    (steel_per_trip as u16, titanium_per_trip as u16)
}

fn read_input_u16(prompt: &str) -> u16 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u16>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input, please enter a valid positive whole number.");
            }
        }
    }
}
