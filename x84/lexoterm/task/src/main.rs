use sysinfo::{ProcessExt, System, SystemExt};
use std::{thread, time};

fn main() {
    let mut sys = System::new_all();
    let top_n = 10; // Number of processes to display

    loop {
        sys.refresh_all();

        let mut processes: Vec<_> = sys.processes().values().collect();
        // Sort processes by CPU usage in descending order
        processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

        println!("{:<5} {:<8} {:<5} {:<8} {:<8} {:<6} {:<6} {:<15}", "PID", "USER", "PR", "NI", "VIRT", "RES", "%CPU", "COMMAND");

        // Only iterate over the top N processes
        for process in processes.iter().take(top_n) {
            let user = process.as_user().unwrap_or("unknown");
            let pid = process.pid();
            let cpu_usage = process.cpu_usage();
            let memory = process.memory(); // In KB
            let name = process.name();
            println!("{:<5} {:<8} {:<5} {:<8} {:<8} {:<6.2} {:<6.2} {:<15}", 
                pid, user, "20", "0", "0", memory as f32 / 1024.0, cpu_usage, name);
        }

        // Sleep for a second before the next update
        thread::sleep(time::Duration::from_secs(1));

        // Clear the terminal screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
}
