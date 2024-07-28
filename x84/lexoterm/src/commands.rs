use std::fs::{self, File};
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process;
use std::net::TcpStream;
use std::time::UNIX_EPOCH;
use std::fs::DirEntry;
use std::process::Command;
use std::env::current_dir;
use crate::sys2;

// Dir color will be blue, file color will be white
const DIR_COLOR: &str = "\x1b[38;2;69;133;136m";
const FILE_COLOR: &str = "\x1b[38;2;255;255;255m";
const RESET_COLOR: &str = "\x1b[0m";
const RUST_FILE_COLOR: &str = "\x1b[38;2;255;0;0m";
pub fn execute_command(input: &str) {
    match input {
        "ex" => {
            println!("Exiting...");
            process::exit(0);
        }
        "kernel" => min_kernel(),
        "help" => show_help(),
        "l" => list_files(),
        "c" => clear_screen(),
        "wai" => where_am_i(),
        "dt" => date_time(),
        "drive" => drive_info(),
        cmd if cmd.starts_with("copy") => copy_item(cmd),
        cmd if cmd.starts_with("cd") => change_directory(cmd),
        cmd if cmd.starts_with("mkf") => create_file(cmd),
        cmd if cmd.starts_with("rm") => remove_file(cmd),
        cmd if cmd.starts_with("ip") => show_ip(),
        cmd if cmd.starts_with("rn") => rename_file(cmd),
        cmd if cmd.starts_with("mkd") => make_directory(cmd),
        cmd if cmd.starts_with("vim") => {
            let parts: Vec<&str> = cmd.split_whitespace().collect();
            let file_path = if parts.len() > 1 { Some(parts[1]) } else { None };
            open_vi(file_path); // Pass the file path to open_vi
        }
        _ => println!("???"),
        

    }
}
// Keep the rest of your functions here

// Note: Ensure all other warnings (like unreachable patterns) are resolved by reordering the match arms as shown.

// Existing functions: show_help, list_files, create_file
// Text editor like vim or nano
fn open_vi(file_path: Option<&str>) {
    let mut command = Command::new("vim");

    if let Some(path) = file_path {
        command.arg(path); // Add the file path as an argument
    }

    match command.spawn() {
        Ok(mut child) => {
            child.wait().expect("Failed to wait on child");
        }
        Err(err) => {
            match Command::new("sh")
                .arg("-c")
                .arg("whereis vi | cut -d' ' -f2 | xargs vi")
                .spawn()
            {
                Ok(mut child) => {
                    child.wait().expect("Failed to wait on child");
                }
                Err(err) => {
                    println!("Error: could not find or execute vi: {}", err);
                }
            }
        }
    }
}
fn min_kernel() {
    let output = Command::new("figlet")
        .arg("minkernel")
        .output()
        .expect("Failed to execute command");
    let figlet_text = String::from_utf8_lossy(&output.stdout);
    print!("{}", figlet_text);
    
    println!("");
    println!("---kernel version 0.0.2---");
}



fn show_help() {
    println!("Available commands:");
    println!("hello - Say hello");
    println!("exit - Exit the terminal");
    println!("help - Show this help message");
    println!("l - List files in the current directory");
    println!("mkf <filename> - Create a new file with the given name");
    println!("rm <filename> - remove file/folder");
    println!("ip - show ip information");
    println!("rn <filename> <newfilename> - rename file");
    println!("mkd <filename> - create a new folder");
    println!("cd <filename> - change directory");
    println!("copy <filename> <newfilename> - make a copy");
    println!("dt - show date and time");
    println!("kernel - show kernel version");
    println!("wai - show current directory");
    println!("drive - show drive information");

    println!("");
}
fn create_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        println!("file created");
        println!("");
        return;
    }

    let file_name = parts[1];
    match File::create(Path::new(file_name)) {
        Ok(_) => println!("{} created successfully.", file_name),
        Err(e) => println!("Error creating file {}: {}", file_name, e),
    }
}



fn drive_info() {
    println!("Drive information: ");
    println!("Keyboard drive: 0.0.1");
    println!("Internal keyboar(laptop) drive: 0.0.1");
    println!("USB drive: 0.0.1 BETA");
    println!("More drivers and updates coming soon!");
    println!("For more information visit: https://lex-studio.net/lexOS/drivers");
}

fn date_time() {
    let now = chrono::Local::now();
    println!("{}", now.format("%Y-%m-%d %H:%M:%S"));
}
fn show_ip() {
    match TcpStream::connect("google.com:80") {
        Ok(stream) => {
            let local_addr = stream.local_addr().unwrap();
            println!("Local IP address: {}", local_addr.ip());
        }
        Err(e) => println!("Unable to connect: {}", e),
    }
}

fn rename_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: rn <oldname> <newname>");
        return;
    }

    let old_name = parts[1];
    let new_name = parts[2];
    match fs::rename(old_name, new_name) {
        Ok(_) => println!("{} renamed to {}.", old_name, new_name),
        Err(e) => println!("Error renaming file: {}", e),
    }
}

fn make_directory(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: mk <directory>");
        return;
    }

    let dir_name = parts[1];
    match fs::create_dir(dir_name) {
        Ok(_) => println!("Directory {} created successfully.", dir_name),
        Err(e) => println!("Error creating directory {}: {}", dir_name, e),
    }
}
fn remove_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: rm <path>");
        return;
    }
    let path = parts[1];
    // Try to remove both directories and files
    if Path::new(path).is_dir() {
        match fs::remove_dir_all(path) {
            Ok(_) => println!("{} removed successfully.", path),
            Err(e) => println!("Error removing {}: {}", path, e),
        }
    } else {
        match fs::remove_file(path) {
            Ok(_) => println!("{} removed successfully.", path),
            Err(e) => println!("Error removing {}: {}", path, e),
        }
    }
}

fn clear_screen() {
    #[cfg(target_os = "windows")]
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();

    #[cfg(not(target_os = "windows"))]
    Command::new("clear").status().unwrap();
}





fn change_directory(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.len() < 2 {
        println!("Usage: cd <directory>");
        return;
    }

    let dir = parts[1];

    // Handle the special case of `cd ..`
    if dir == ".." {
        let current_dir = std::env::current_dir().unwrap();
        let restricted_path = Path::new("/home/user/lexOS");

        if current_dir == restricted_path {
            println!("You are already in root directory");
        } else {
            if let Some(parent) = current_dir.parent() {
                if std::env::set_current_dir(parent).is_ok() {
                    // Optionally print the new directory
                    // println!("{}", parent.display()); 
                } else {
                    println!("Failed");
                }
            } else {
                println!("Failed");
            }
        }
    } else { // Handle other directories
        let target_path = Path::new(dir);
        let allowed_prefix = Path::new("/home/user/lexOS");
        let is_absolute = target_path.is_absolute();
        
        if is_absolute && !target_path.starts_with(allowed_prefix) {
            println!("directory doesnt exist");
        } else {
            if std::env::set_current_dir(dir).is_ok() {
                // Optionally print the new directory
                // println!("{}", dir); 
            } else {
                println!("Failed to change directory to {}", dir);
            }
        }
    }
}

// Recursive directory copy function
fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            if let Err(e) = fs::copy(&entry.path(), &dest_path) {
                return Err(Error::new(ErrorKind::Other, format!("Failed to copy file: {:?}", e)));
            }
        }
    }
    Ok(())
}

// Adjusted `copy_item` function.
fn copy_item(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: copy <source> <destination>");
        return;
    }
    let source_path = Path::new(parts[1]);
    let destination_path = Path::new(parts[2]);

    // Determine the final destination path
    let final_destination = if destination_path.is_dir() {
        destination_path.join(source_path.file_name().expect("Failed to get file name"))
    } else {
        destination_path.to_path_buf()
    };

    if source_path.is_dir() {
        match copy_dir_all(source_path, &final_destination) {
            Ok(_) => println!("Directory copied from {:?} to {:?}", source_path, final_destination),
            Err(e) => println!("Failed to copy directory: {}", e),
        }
    } else {
        match fs::copy(source_path, &final_destination) {
            Ok(_) => println!("File copied from {:?} to {:?}", source_path, final_destination),
            Err(e) => println!("Failed to copy file: {}", e),
        }
    }
}
fn get_file_size(entry: &DirEntry) -> io::Result<f64> {
    let metadata = entry.metadata()?;
    // Convert the file size from bytes to megabytes with higher precision
    Ok(metadata.len() as f64 / 1_048_576.0)
}

fn get_modification_date(entry: &DirEntry) -> io::Result<String> {
    let metadata = entry.metadata()?;
    let modified_time = metadata.modified().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.duration_since(UNIX_EPOCH).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?.as_secs();
    let datetime = chrono::NaiveDateTime::from_timestamp(modified_time as i64, 0);
    Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string())
}

fn list_files() {
    let current_dir = std::env::current_dir().unwrap();
    let entries = fs::read_dir(current_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let (file_type, color) = if entry.file_type().unwrap().is_dir() {
            ("Dir ", DIR_COLOR)
        }
        else {
            ("File", FILE_COLOR)
        };
        let file_size = get_file_size(&entry).unwrap(); // Returns size in MB
        let modification_date = get_modification_date(&entry).unwrap();
        let file_name = entry.file_name().into_string().unwrap_or_else(|_| "<invalid UTF-8>".to_string());
        // Apply color to the file name output
        println!("{}{:10} {:10.2} MB {} {}{}", color, file_type, file_size, modification_date, file_name, RESET_COLOR);
    }
}
fn move_item(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: move <source> <destination>");
        return;
    }

    let source = Path::new(parts[1]);
    let destination = Path::new(parts[2]);

    if let Err(e) = fs::rename(source, destination) {
        println!("Failed to move '{}': {}", source.display(), e);
    } else {
        println!("Moved '{}' to '{}'", source.display(), destination.display());
    }
}


fn where_am_i() {
    match std::env::current_dir() {
        Ok(path) => {
            let lexos_root = Path::new("/home/user/");

            if let Ok(stripped_path) = path.strip_prefix(lexos_root) {
                println!("{}", stripped_path.display()); // Display the path after lexOS
            } else {
                // Handle cases where the current directory isn't under lexOS
                println!("{}", path.display()); // Or display a different message
            }
        }
        Err(e) => println!("Failed to get current directory: {}", e),
    }
}

