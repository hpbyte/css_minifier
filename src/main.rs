use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Path of the input file
    #[arg(short, long)]
    input_file: String,

    // Path of the output file
    #[arg(short, long)]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    println!("in {}", args.input_file);
    println!("out {}", args.output_file);
}
