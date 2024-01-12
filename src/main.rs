mod interpreter;
use std::fs;
use std::path::Path;
use clap::Parser;
#[derive(Parser)] // requires `derive` feature
#[command(author, about, long_about = None)]
struct Args {
    #[arg(short = 'd', value_name = "DIMENSION", default_value_t = 8usize, help = "Dimension of memory")]
    dimension: usize,

    #[arg(short = 'v', help = "View memory dump")]
    view_memory: bool,

    #[arg()]
    filename: String,
}

fn main() {

    //get file content
    let args = Args::parse();
    let path = Path::new(&args.filename);
    let content: Vec<u8> = match fs::read_to_string(path) {
        Ok(t) => t.into_bytes(),
        Err(e) => {
            eprintln!("{} ",e);
            std::process::exit(1);
        }
    };
    let dimension = args.dimension;
    let (ptr, mem) = interpreter::interpret(content, dimension);
    if args.view_memory {
        println!("\n---\nFinal dump:");
        println!("pointer = {}", ptr);
        for i in 0..dimension {
            println!("{:?}", &mem[i*dimension..i*dimension+dimension]);
        }
    }
}