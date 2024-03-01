use clap::Parser;
use std::{env, fs};

const ROOT_BASE_NAME: &str = "dir_100";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of directories to create
    #[arg(short, long, default_value_t = 1)]
    dir: u32,
    /// Number of files to create
    #[arg(short, long)]
    file: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let current_path = env::current_dir()?;
    let root_path = current_path.join(ROOT_BASE_NAME);
    fs::create_dir(&root_path)?;

    // fs::File::create("hello.txt")?;
    for i in 1..=args.dir {
        let dir_name = format!("nord_{}", i);
        let dir_path = root_path.join(dir_name);
        fs::create_dir(&dir_path)?;

        for j in 1..=args.file {
            let file_name = format!("nord_{}.txt", j + (args.file * (i - 1)));
            let file_path = dir_path.join(file_name);
            fs::File::create(file_path)?;
        }
    }

    println!("DONE. {} directory(ies) and {} file(s) CREATED!", args.dir, args.file);
    println!("You can now `cd {}` to explore files", ROOT_BASE_NAME);

    Ok(())
}
