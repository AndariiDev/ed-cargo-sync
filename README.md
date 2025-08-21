# cargo_sync

**cargo_sync** is a simple command-line tool for *Elite: Dangerous* pilots to evenly split cargo delivery loads across trips. It helps balance your cargo runs by calculating how much steel, titanium, or other commodities to carry each trip, syncing deliveries efficiently.

---

## Features

- Calculates proportional cargo loads for two commodities.
- Handles user input with validation.
- Plans cargo delivery amounts based on ship capacity.
- Designed for easy expansion (multiple ships, persistence, interactive CLI, and more).

---

## Installation

Build from source using Rust and Cargo:

git clone <repo-url>
cd ed-cargo-sync
cargo build --release

Run the tool with:

cargo run --release

---

## Usage

The tool prompts you to input:

- Steel amount needed (tons)
- Titanium amount needed (tons)
- Ship cargo capacity

It then outputs how much steel and titanium to deliver **per trip** to keep the deliveries balanced.

Example:

Enter steel amount needed: 10000
Enter titanium amount needed: 5000
Enter ship cargo capacity: 1072
Deliver 713 steel and 356 titanium this trip.

---

## Future Plans

- **Persistent Cargo and Ship Profiles:** Save ship configurations (name and capacity) in a TOML file for repeated use without retyping.
- **Multiple Ship Support:** Define and select from multiple ships stored in the config file.
- **Interactive CLI with Commands:** Add subcommands like `add-ship`, `list-ships`, and `plan-trip` for seamless user experience.
- **Extended Commodity Support:** Support for more than two commodities with proportional distribution.
- **Trip Tracking:** Track remaining cargo needs and automate multi-trip planning.
- **Enhanced Input Validation & UX Improvements**
- **Optional Terminal UI (TUI):** More visual and interactive terminal experience.
- **GUI/Web App:** Longer term, consider graphical or web versions.

---

## 

Disclaimer:
This project is an early-stage, personal learning experiment in Rust and CLI design. It’s currently minimal and focused on core functionality.
Please don’t expect polished features, external service integrations, or a GUI anytime soon—it’s a work in progress!
Contributions and suggestions are very welcome as I continue learning and improving.

---

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open issues or submit pull requests.

---

## License

[MIT License](LICENSE)
