use std::env;
use std::process;

mod state;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No arguments provided. Use --help for usage information.");
        process::exit(1);
    }
    let arg1 = &args[1];
    match arg1.as_str() {
        "--list" | "-l" => {
            if args.len() > 2 {
                eprintln!("Too many arguments for --list. Use --help for usage information.");
                process::exit(3);
            }
            list_bookmarks();
        }
        "--remove" | "-r" | "--delete" | "-d" => {
            if args.len() < 3 {
                eprintln!("No bookmark provided for {}. Use --help for usage information.", arg1);
                process::exit(4);
            }
            if args.len() > 3 {
                eprintln!("Too many arguments for {}. Use --help for usage information.", arg1);
                process::exit(5);
            }
            remove_bookmark(args[2].to_string());
        }
        "--find" | "-f" => {
            if args.len() < 3 {
                eprintln!("No bookmark provided for --find. Use --help for usage information.");
                process::exit(6);
            }
            if args.len() > 3 {
                eprintln!("Too many arguments for --find. Use --help for usage information.");
                process::exit(5);
            }
            find_bookmark(args[2].to_string());
        }
        "--help" | "-h" => {
            println!("Usage: bkmk [OPTION] [BOOKMARK]");
            println!("A simple command line bookmark manager.");
            println!();
            println!("Options:");
            println!("  -l, --list             List all bookmarks.");
            println!("  -r, --remove BOOKMARK  Remove the specified bookmark.");
            println!("  -d, --delete BOOKMARK  Alias for --remove.");
            println!("  -f, --find BOOKMARK    Find the directory for the specified bookmark.");
            println!("  -h, --help             Show this help message.");
            println!();
            println!("If no option is provided, a new bookmark will be created with the name BOOKMARK.");
        }
        "🥚" | "🪺" | "🐰" | "🐇" | "🐣" => {
            for _ in 0..5 {
                for i in 0i32..=10 {
                    let j = (i - 5).abs();
                    let s = " ".repeat(5-j as usize) + "🐰" + &" ".repeat(j as usize);
                    print!("\r{}\x07", s);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }
            print!("\r😁\x07\n");
            process::exit(0);
        }
        key if key.starts_with("--") => {
            eprintln!("Unknown option: {key}. Use --help for usage information.");
            process::exit(2);
        }
        key => {
            create_bookmark(key.to_string());
        }
    }
}

/// List the bookmarks saved in the state file.
fn list_bookmarks() {
    let state_file = state::load_state_file();
    if state_file.is_empty() {
        println!("No bookmarks.");
        return;
    }
    println!("Current bookmarks:");
    for (key, value) in &state_file {
        println!("{} -> {}", key, value);
    }
}

/// Add a bookmark with name `new_bookmark`.
fn create_bookmark(new_bookmark: String) {
    println!("Creating bookmark {new_bookmark}.");
    let directory = env::current_dir()
        .unwrap_or_else(|e| {
            eprintln!("Could not find current directory: {e}");
            process::exit(1);
        })
        .into_os_string()
        .into_string()
        .unwrap_or_else(|e| {
            eprintln!("Could not find convert current directory to string: {e:?}");
            process::exit(1);
        });
    let mut state_map = state::load_state_file();
    state_map.insert(new_bookmark, directory);
    state::write_state_file(state_map);
}

/// Remove the bookmark with name `bookmark` if it exists.
fn remove_bookmark(bookmark: String) {
    println!("Removing bookmark {bookmark}.");
    let mut state_map = state::load_state_file();
    state_map.remove(&bookmark);
    state::write_state_file(state_map);
}

/// Look up the bookmark with name `bookmark`.
fn find_bookmark(bookmark: String) {
    let state_map = state::load_state_file();
    let result = state_map.get(&bookmark);
    match result {
        Some(destination) => {
            println!("{}", destination);
        },
        None => {
            eprintln!("Bookmark {bookmark} not found.");
            process::exit(7);
        }
    }
}
