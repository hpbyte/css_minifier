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

    let mut is_comment = false;
    let mut is_space = false;

    for line in fi.lines() {
        let in_line_chars = line.chars().collect::<Vec<char>>();
        let mut out_line_chars: Vec<char> = vec![];

        if in_line_chars.len() == 0 {
            continue;
        }

        let mut o = 0;

        // remove comments
        for i in 0..in_line_chars.len() - 1 {
            // comment ends
            if is_comment && in_line_chars[i] == '*' && in_line_chars[i + 1] == '/' {
                is_comment = false;
                continue;
            }
            if is_comment {
                continue;
            }
            // comment starts
            if !is_comment && in_line_chars[i] == '/' && in_line_chars[i + 1] == '*' {
                is_space = false;
                is_comment = true;
                continue;
            }

            // remove spaces
            if in_line_chars[i] != ' ' && in_line_chars[i] != '\t' {
                is_space = false;
            }
            if in_line_chars[i] == '\r'
                || in_line_chars[i] == '\n'
                || (is_space && (in_line_chars[i] == ' ' || in_line_chars[i] == '\t'))
            {
                is_space = true;
                continue;
            }
            if in_line_chars[i] == '{' {
                loop {
                    if (o > 1) && (in_line_chars[o - 1] == ' ' || in_line_chars[o - 1] == '\t') {
                        break;
                    }
                    o -= 1;
                }
            }
            if in_line_chars[i] == '>' {
                loop {
                    if (o > 1) && (in_line_chars[o - 1] == ' ' || in_line_chars[o - 1] == '\t') {
                        break;
                    }
                    o -= 1;
                }
            }
            if in_line_chars[i] == ':'
                || in_line_chars[i] == ';'
                || in_line_chars[i] == ','
                || in_line_chars[i] == '>'
            {
                is_space = true;
            }

            // copy
            out_line_chars.push(in_line_chars[i]);
            o += 1;
        }

        // write
        fo.write(String::from_iter(out_line_chars).as_bytes())?;
    }

    Ok(())
}
