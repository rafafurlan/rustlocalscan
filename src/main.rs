use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::env;
use std::sync::mpsc;
use std::collections::HashSet;
use std::process::Command;

#[tokio::main]
async fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <First 3 Parts of IP> <START_PORT> <END_PORT> <MAX_THREADS>", args[0]);
        eprintln!("Example: 192.168.0 1 65535 50");
        return;
    }

    // Shared list to store found hosts
    let hosts = Arc::new(Mutex::new(HashSet::new()));
    let ip_base = &args[1];
    let port_start: u16 = args[2].parse().expect("Invalid start port");
    let port_end: u16 = args[3].parse().expect("Invalid end port");
    let max_threads: usize = args[4].parse().expect("Invalid number of max threads");

    let (tx, rx) = mpsc::channel();
    let active_threads = Arc::new(Mutex::new(0));

    // Scan IP range (1-254)
    for i in 1..=254 {
        let complete_ip = format!("{}.{}", ip_base, i);
        let tx = tx.clone();
        let active_threads = Arc::clone(&active_threads);
        let hosts = Arc::clone(&hosts);

        {
            // Limit the number of active threads
            let mut num_threads = active_threads.lock().unwrap();
            while *num_threads >= max_threads {
                drop(num_threads);
                thread::sleep(Duration::from_millis(100));
                num_threads = active_threads.lock().unwrap();
            }
            *num_threads += 1;
        }

        // Spawn a new thread to perform a ping, arping, and nmap scan
        thread::spawn(move || {
            if ping_scan(&complete_ip) || arping_scan(&complete_ip) || nmap_scan(&complete_ip) {
                let mut hosts = hosts.lock().unwrap();
                hosts.insert(complete_ip.clone());
            }
            tx.send(()).unwrap();
            let mut num_threads = active_threads.lock().unwrap();
            *num_threads -= 1;
        });
    }

    // Wait for all scans to complete
    for _ in 1..=254 {
        rx.recv().unwrap();
    }

    // Perform port scan on found hosts
    let hosts = hosts.lock().unwrap();
    for host in &*hosts {
        println!("Host found: {}", host);
        println!("Open Ports:");

        let tx = tx.clone();
        let active_threads = Arc::clone(&active_threads);

        for port in port_start..=port_end {
            let host = host.clone();
            let tx = tx.clone();
            let active_threads = Arc::clone(&active_threads);

            {
                // Limit the number of active threads
                let mut num_threads = active_threads.lock().unwrap();
                while *num_threads >= max_threads {
                    drop(num_threads);
                    thread::sleep(Duration::from_millis(100));
                    num_threads = active_threads.lock().unwrap();
                }
                *num_threads += 1;
            }

            // Spawn a new thread to perform a port scan
            thread::spawn(move || {
                if scan_port(&host, port) {
                    println!("Port {} open", port);
                }
                tx.send(()).unwrap();
                let mut num_threads = active_threads.lock().unwrap();
                *num_threads -= 1;
            });
        }

        // Wait for all port scans to complete for this host
        for _ in port_start..=port_end {
            rx.recv().unwrap();
        }
        println!();
    }
}

// Function to scan a specific port on a given IP
fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    match address.parse::<SocketAddr>() {
        Ok(addr) => TcpStream::connect_timeout(&addr, Duration::from_millis(500)).is_ok(),
        Err(_) => false,
    }
}

// Function to perform a ping scan on a given IP
fn ping_scan(ip: &str) -> bool {
    let ping_output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output();

    if let Ok(output) = ping_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.contains("64 bytes");
        }
    }
    false
}

// Function to perform an arping scan on a given IP
fn arping_scan(ip: &str) -> bool {
    let arping_output = Command::new("arping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output();

    if let Ok(output) = arping_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.contains("60 bytes");
        }
    }
    false
}

// Function to perform an nmap scan on a given IP
fn nmap_scan(ip: &str) -> bool {
    let nmap_output = Command::new("nmap")
        .arg("-sn")
        .arg(ip)
        .output();

    if let Ok(output) = nmap_output {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.contains("1 host up");
        }
    }
    false
}

