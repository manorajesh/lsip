use rayon::prelude::*;
use std::{process::{Command, exit}, sync::{Arc, Mutex}};
use clap::Parser;

#[derive(Parser)]
#[command(
    version = "0.0.2",
    author = "Mano Rajesh",
    about = "Scan the local network for active hosts."
)]

struct Args {
    /// IP range to scan, use wildcard "*" to define range [default: 192.168.1.*]
    ip_range: String,

    /// How to wait for a response [default: 1 second]
    #[arg(short = 't', long = "timeout", value_name="SECONDS")]
    timeout: Option<usize>,

    /// Total IPs to scan [default: 255]
    #[arg(short = 'n', long = "total", value_name="TOTAL")]
    total: Option<usize>,
}

fn main() {
    let args = Args::parse();
    if !args.ip_range.contains("*") {
        println!("Invalid IP range");
        exit(1);
    }

    let ip_range = String::from(args.ip_range);
    let output = Arc::new(Mutex::new(Vec::new()));
    let total_ips = args.total.unwrap_or(255);

    (0..total_ips).into_par_iter().for_each(|i| {
        let ping_lines = ping(ip_range.replace("*", &i.to_string()), args.timeout.unwrap_or(1));
        let mut blocker = output.lock().unwrap();
        blocker.push(ping_lines);
        progress_bar(blocker.len(), total_ips);
    });

    output.lock().unwrap().sort_by(|a, _| {
        if a.contains("is up") {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });

    println!();
    for line in output.lock().unwrap().iter() {
        println!("{}", line);
    }
}

fn ping(ip: String, timeout: usize) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("ping")
            .args(&["/n", "1", &ip])
            .arg(&format!("-w{}", timeout))
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&ip)
            .arg(&format!("-t{}", timeout))
            .output()
            .expect("failed to execute process")
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("TTL") || stdout.contains("ttl") {
        let ip = format!("{0:15}", ip);
        if let Some(time) = stdout.split("time").nth(1) {
            let time = time.split("ms").next().unwrap().replace("=", "");
            let line = format!(
                "{ip}is up \x1b[5;32m•\x1b[0m \x1b[38;5;8m({time}ms)\x1b[0m",
                time = time
            );
            return line;
        } else {
            let line = format!("{ip}is up \x1b[5;32m•\x1b[0m \x1b[37m(unknown ms)\x1b[0m"); // two spaces to line up
            return line;
        }
    } else {
        let ip = format!("{0:15}", ip);
        let line = format!("{ip}is down \x1b[5;31m•\x1b[0m");
        return line;
    }
}

fn progress_bar(current: usize, total: usize) {
    let mut progress = String::new();
    let mut percent = current as f32 / total as f32 * 100.0;
    percent = percent.round();
    let mut i = 0;
    while i < percent as usize {
        progress.push_str("=");
        i += 1;
    }
    progress.push_str(">");
    while i < 100 {
        progress.push_str(" ");
        i += 1;
    }
    print!("\r[{}] {}%", progress, percent);
}
