use std::io;

struct Ship {
    name: String,
    capacity: u16,
}

struct Commodity {
    name: String,
    required: u32,
}

fn main() {
    // Example cargo amount (pretend user input)
    let steel_amount = 10000;

    // Create a commodity instance
    let steel = Commodity {
        name: String::from("Steel"),
        required: steel_amount,
    };

    // print to check
    println!("Commodity: {}, Amount: {}", steel.name, steel.required);
}
