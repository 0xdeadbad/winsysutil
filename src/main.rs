use clap::Parser;

static _VERSION: &str = "0.1.0";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
    println!("winsysutil {}", _VERSION);

    println!("[WIP]");
    todo!()
}
