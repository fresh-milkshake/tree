use std::env;
use std::path::Path;
use std::fs;
use std::process;

fn sanitize_filename(filename: &str) -> String {
    filename.to_string()
}

fn tree_with_depth(path: &Path, depth: i32, prefix: &str) {
    if depth < 0 {
        return;
    }

    let items = match fs::read_dir(path) {
        Ok(items) => items.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, _>>(),
        Err(_) => return,
    };

    match items {
        Ok(items) => {
            for (i, item) in items.iter().enumerate() {
                let is_last = i == items.len() - 1;
                let file_name = item.file_name().unwrap().to_string_lossy();
                let marker = if is_last { "└── " } else { "├── " };

                println!("{}{}{}", prefix, marker, sanitize_filename(&file_name));

                if depth > 0 {
                    let next_prefix = if is_last { "    " } else { "│   " };
                    tree_with_depth(&item, depth - 1, &(prefix.to_string() + next_prefix));
                }
            }
        },
        Err(_) => {}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <path> <depth>");
        process::exit(1);
    }

    let path = Path::new(&args[1]);
    let depth: i32 = args[2].parse().unwrap();

    if !path.exists() {
        println!("Error: Path does not exist.");
        process::exit(1);
    }

    if !path.is_dir() {
        println!("Error: Path is not a directory.");
        process::exit(1);
    }

    tree_with_depth(path, depth, "");
}
