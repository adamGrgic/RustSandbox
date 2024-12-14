// see https://rust-cli.github.io/book/tutorial/errors.html for current prog

use clap::Parser;

# [derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running CLI Application!");

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

        for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // let content = std::fs::read_to_string("test.txt")?;

    println!("file content: {}", content);
    Ok(())
    // for reference
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // lets args = Cli {
    //    pattern,
    //    path: std::path::PathBuf::from(path)
    // }

    // println!("pattern: {:?}, path: {:?}", pattern, path)
    //

    //    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)



}
