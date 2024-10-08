# Rust Local Scan

This project is a network scanner written in Rust that performs a ping sweep to identify active hosts in a local network, followed by a port scan on those hosts to identify open ports. The scanner utilizes multithreading to increase efficiency and supports three different methods for detecting active hosts: `ping`, `arping`, and `nmap`.

## Features
- Scans IP range (e.g., `192.168.0.1` to `192.168.0.254`) to find active hosts.
- Uses `ping`, `arping`, and `nmap` for host detection to ensure better coverage.
- Performs a port scan on active hosts to identify open ports.
- Supports multithreading with user-defined thread limits to optimize scan speed.

## Usage
To run the scanner, you need to specify the IP range (first 3 parts), the port range, and the maximum number of threads allowed.

### Command Line Arguments
```
Usage: sudo <program_name> <First 3 Parts of IP> <START_PORT> <END_PORT> <MAX_THREADS>
Example: sudo cargo run -- 192.168.0 1 65535 50
```
- **First 3 Parts of IP**: The first 3 octets of the IP address range to scan (e.g., `192.168.0`).
- **START_PORT**: The starting port number for the port scan (e.g., `1`).
- **END_PORT**: The ending port number for the port scan (e.g., `65535`).
- **MAX_THREADS**: The maximum number of threads to use for scanning (e.g., `50`).

### Example
```
sudo cargo run -- 192.168.0 1 1024 50
```
This command will scan the IP addresses from `192.168.0.1` to `192.168.0.254`, scanning ports `1` to `1024` on each active host, and will use a maximum of `50` threads.

## Requirements
- **Rust**: You need to have Rust installed to run this project.
- **Tools**: The following command line tools must be installed and accessible:
  - `ping`
  - `arping`
  - `nmap`

## Installation
1. Clone the repository:
   ```
   git clone <repository_url>
   ```
2. Navigate to the project directory:
   ```
   cd <project_directory>
   ```
3. Build the project using Cargo:
   ```
   cargo build --release
   ```

## How It Works
1. **Host Discovery**: The program scans the IP range specified by the user. Each IP is checked using `ping`, `arping`, and `nmap` to ensure active hosts are detected.
2. **Port Scanning**: Once an active host is identified, a port scan is performed on the specified port range to find open ports.
3. **Thread Management**: The program uses a user-defined number of threads to efficiently manage multiple scans simultaneously without overwhelming system resources.

## Notes
- The program currently targets Linux environments, as it relies on command line tools (`ping`, `arping`, `nmap`) with arguments that are specific to Linux. To make it compatible with Windows, you may need to adjust the command arguments.
- Ensure you have sufficient permissions to run `arping` and `nmap`. Running the program with `sudo` is recommended.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributions
Contributions are welcome! Feel free to open issues or submit pull requests to improve the code or add new features.

## Disclaimer
Use this tool responsibly. Only scan networks and devices you own or have permission to scan.

## Author
Developed by Rafael Furlan.

