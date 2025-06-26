# TEMPers

A simple Rust utility to read TEMPer v1.4 USB thermometers. This personal project was created to learn Rust while solving a practical need for temperature monitoring in my homelab room.

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/tempers.git
   cd tempers
   ```

2. Install udev rules to allow non-root access:
   ```bash
   sudo cp 70-temper.rules /etc/udev/rules.d/
   sudo udevadm control --reload-rules
   sudo udevadm trigger
   ```

3. Unplug and replug your TEMPer device

4. Build with Cargo:
   ```bash
   cargo build --release
   ```

## Usage

Run the compiled binary:
```bash
./target/release/tempers
```

Sample output:
```
24.35  # Temperature is given in Celsius degrees
```

## Notes

- This is a personal learning project
- Currently only supports TEMPer v1.4 (0c45:7401)
- Requires Linux (udev) and Rust 1.70+
