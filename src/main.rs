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
    let mut commodity_name = String::new();

    println!("Enter the commodity name:");

    io::stdin()
        .read_line(&mut commodity_name)
        .expect("Failed to read line");

    // Remove the newline character at the end
    let commodity_name = commodity_name.trim();

    // print to check
    println!("You entered: {}", commodity_name);

    let mut commodity_required_str = String::new();

    println!("Enter the commodity amount:");

    io::stdin()
        .read_line(&mut commodity_required_str)
        .expect("Failed to read amount");

    let commodity_required: u32 = commodity_required_str
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("You entered: {}", commodity_required);
}
