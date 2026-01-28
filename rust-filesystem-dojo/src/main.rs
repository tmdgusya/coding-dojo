use fstools::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "read" => {
            if args.len() < 3 {
                eprintln!("Usage: fstools read <file>");
                return;
            }
            match read_file_to_string(&args[2]) {
                Ok(contents) => println!("{}", contents),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "lines" => {
            if args.len() < 3 {
                eprintln!("Usage: fstools lines <file>");
                return;
            }
            match read_lines(&args[2]) {
                Ok(lines) => {
                    for (i, line) in lines.iter().enumerate() {
                        println!("{:4}: {}", i + 1, line);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "grep" => {
            if args.len() < 4 {
                eprintln!("Usage: fstools grep <pattern> <file>");
                return;
            }
            match grep_lines(&args[3], &args[2]) {
                Ok(matches) => {
                    for (line_num, line) in matches {
                        println!("{:4}: {}", line_num, line);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "ls" => {
            let dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            match list_directory(dir) {
                Ok(entries) => {
                    for entry in entries {
                        println!("{}", entry.display());
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "info" => {
            if args.len() < 3 {
                eprintln!("Usage: fstools info <path>");
                return;
            }
            match FileInfo::from_path(&args[2]) {
                Ok(info) => {
                    println!("Path: {}", info.path.display());
                    println!("Size: {} bytes", info.size);
                    println!("Is File: {}", info.is_file);
                    println!("Is Directory: {}", info.is_dir);
                    println!("Is Readonly: {}", info.is_readonly);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "size" => {
            let dir = args.get(2).map(|s| s.as_str()).unwrap_or(".");
            match calculate_dir_size(dir) {
                Ok(size) => {
                    let (size_val, unit) = if size >= 1024 * 1024 * 1024 {
                        (size as f64 / (1024.0 * 1024.0 * 1024.0), "GB")
                    } else if size >= 1024 * 1024 {
                        (size as f64 / (1024.0 * 1024.0), "MB")
                    } else if size >= 1024 {
                        (size as f64 / 1024.0, "KB")
                    } else {
                        (size as f64, "bytes")
                    };
                    println!("{:.2} {}", size_val, unit);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: fstools copy <src> <dst>");
                return;
            }
            match copy_file(&args[2], &args[3]) {
                Ok(bytes) => println!("Copied {} bytes", bytes),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "move" => {
            if args.len() < 4 {
                eprintln!("Usage: fstools move <src> <dst>");
                return;
            }
            match move_file(&args[2], &args[3]) {
                Ok(()) => println!("Moved {} -> {}", args[2], args[3]),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "help" | "-h" | "--help" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!(
        r#"
fstools - FileSystem Toolkit

USAGE:
    fstools <command> [arguments]

COMMANDS:
    read <file>             Read and print file contents
    lines <file>            Read and print file with line numbers
    grep <pattern> <file>   Search for pattern in file
    ls [dir]                List directory contents
    info <path>             Show file/directory information
    size [dir]              Calculate directory size
    copy <src> <dst>        Copy file
    move <src> <dst>        Move file
    help                    Show this help message
"#
    );
}
