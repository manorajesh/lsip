use std::process::Command;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

fn main() {
    let ip_range = String::from("192.168.32.*");
    let output = Arc::new(Mutex::new(Vec::new()));
    let total_ips = 255;

    (0..255).into_par_iter().for_each(|i| {
        let ping_lines = ping(ip_range.replace("*", &i.to_string()));
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

fn ping(ip: String) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("ping")
            .args(&["/n", "1", &ip])
            .output()
            .expect("failed to execute process")            
    } else {
        Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&ip)
            .output()
            .expect("failed to execute process")
    };
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.contains("TTL") {
        let ip = format!("{0:15}", ip);
        if let Some(time) = stdout.split("time").nth(1) {
            let time = time.split("ms").next().unwrap().replace("=", "");
            let line = format!("{ip}is up \x1b[5;32m•\x1b[0m \x1b[38;5;8m({time}ms)\x1b[0m", time = time);
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
    while i < 100 {
        progress.push_str(" ");
        i += 1;
    }
    print!("\r[{}] {}%", progress, percent);
}