use clap::Parser;
use std::fs;
use std::io::prelude::*;

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

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let fi = fs::read_to_string(&args.input_file).expect("could not read input file");
    let mut fo =
        fs::File::create(&args.output_file).expect("error encountered while creating the file!");

    println!("in {}", args.input_file);
    println!("out {}", args.output_file);

    for line in fi.lines() {
        let mut out_str = String::from("");

        // remove comments

        // remove spaces
        out_str.push_str(line.replace(" ", "").as_str());

        // write
        fo.write(out_str.as_bytes())?;
    }

    Ok(())
}
