use std::{process::Command, thread};

fn main() {
    let ip_range: String = "192.168.32.*".to_owned();
    let num_of_threads: i32 = 10;
    let mut start: i32 = 0;
    let mut threads: Vec<_> = Vec::new();
    
    for _ in 0..num_of_threads {
        let end = start + 255/num_of_threads;
        let ip_range = ip_range.clone();
        let t = thread::spawn(move || {
            ping(ip_range, start, end);
        });
        start = end;
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }
}

fn ping(ip_range: String, start: i32, end: i32) {
    for i in start..end {
        let ip = ip_range.replace("*", &format!("{}", i).as_str());
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
        
        let text = format!("{:?}", output.stdout);
        if text.contains("time") && output.status.success() {
            println!("{}is up \x1b[5;32m•\x1b[0m", format!("{0:15}", ip));
        } else {
            println!("{}is down \x1b[5;31m•\x1b[0m", format!("{0:15}", ip));
        }
    }
}