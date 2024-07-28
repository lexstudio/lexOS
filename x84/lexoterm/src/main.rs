use crate::commands::execute_command;
use std::io::{self, Write};
use rand::Rng;  // Import the Rng trait to use random number generation methods.
use std::process::Command;
use std::thread::sleep;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

mod commands; // This line includes the commands module you've just created.
fn login() {
    print!("\x1B[2J\x1B[1;1H");// ANSI escape codes for clearing screen
// sleep the thread for 1 second
    sleep(std::time::Duration::from_secs(1));
    let output = Command::new("figlet")
        .arg("Login")
        .output()
        .expect("Failed to execute figlet");
    let figlet_text = String::from_utf8_lossy(&output.stdout);
    println!("{}", figlet_text);
    println!("Welcome to LexOS 0.1 BETA!");
    println!("minkernel 0.0.2");
    println!("please login to continue");
    let mut username = String::new();
    let mut password = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Enter your password: ");
    io::stdin().read_line(&mut password).unwrap();
    let username = username.trim();
    let password = password.trim();
    if username == "lex" && password == "sun12" {
        println!("Login successful!");
        sleep(std::time::Duration::from_millis(600));
        sys();
    } else {
        println!("Login failed!");
        login();
    }
}
 // Import the function to execute commands.
pub fn sys() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing screen

    let output = Command::new("figlet")
        .arg("LexOS")
        .output()
        .expect("Failed to execute figlet");
    
    let figlet_text = String::from_utf8_lossy(&output.stdout);
    println!("{}", figlet_text);
    println!("---version 0.0.1---");
    println!("");
    println!("--made by lex-studio--");
    println!("");

    // if up arrow key is pressed, the previous command is displayed
    // if down arrow key is pressed, the next command is displayed
    
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nHello you found an easter egg!");
        // No change to any running flag, just an informative message.
    }).expect("Hello you found an easter egg!");

    while running.load(Ordering::SeqCst) {
        print!(" > ");
        io::stdout().flush().unwrap();  // Ensure the prompt is displayed immediately.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // Trim newline characters.
                execute_command(input); // Execute the command using the function from commands.rs
            },
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}
pub fn sys2() {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing screen

    let output = Command::new("figlet")
        .arg("LexOS")
        .output()
        .expect("Failed to execute figlet");
    
    let figlet_text = String::from_utf8_lossy(&output.stdout);
    println!("{}", figlet_text);
    println!("---version 0.0.1---");
    println!("");
    println!("--made by lex-studio--");
    println!("");

    // if up arrow key is pressed, the previous command is displayed
    // if down arrow key is pressed, the next command is displayed
    
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();


    while running.load(Ordering::SeqCst) {
        print!(" > ");
        io::stdout().flush().unwrap();  // Ensure the prompt is displayed immediately.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // Trim newline characters.
                execute_command(input); // Execute the command using the function from commands.rs
            },
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}
fn main() {
    println!("\x1B[2J\x1B[1;1H"); // ANSI escape codes for clearing screen
    println!("BIOS: Power on self test...");
    sleep(std::time::Duration::from_millis(500));

    println!("BIOS: Checking hardware integrity...");
    sleep(std::time::Duration::from_millis(100));

    println!("BIOS: Detecting storage devices...");
    sleep(std::time::Duration::from_millis(300));

    println!("BIOS: Hardware check passed. Loading bootloader...");
    sleep(std::time::Duration::from_millis(100));
    println!("Bootloader: Initializing...");
    sleep(std::time::Duration::from_millis(200));  
    println!("Bootloader: Checking file system...");
    sleep(std::time::Duration::from_millis(400));
    println!("Bootloader: Loading kernel into memory...");
    sleep(std::time::Duration::from_millis(200));
    println!("Kernel: Starting system services...");
    sleep(std::time::Duration::from_millis(400));
   
    println!("Kernel: Mounting file systems...");
    sleep(std::time::Duration::from_millis(300));
    println!("lexOS: Loading user interface...");
    sleep(std::time::Duration::from_millis(300));
    println!("lexOS: Starting network services...");
    sleep(std::time::Duration::from_millis(100));
    println!("lexOS: Boot successful. Welcome to lexOS!");
    sleep(std::time::Duration::from_millis(300));
       login();
}
