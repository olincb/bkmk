use std::env;
use std::process;

mod state;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    match arg1.as_str() {
        "--list" | "-l" => {
            list_bookmarks();
        }
        "--remove" | "-r" => {
            remove_bookmark(args[2].to_string());
        }
        "--find" | "-f" => {
            find_bookmark(args[2].to_string());
        }
        key => {
            create_bookmark(key.to_string());
        }
    }
}

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

fn remove_bookmark(bookmark: String) {
    println!("Removing bookmark {bookmark}.");
    let mut state_map = state::load_state_file();
    state_map.remove(&bookmark);
    state::write_state_file(state_map);
}

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
