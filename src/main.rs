use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("use: css_minifier -i PATH_TO_INPUT_FILE -o OUTPUT_FILE_PATH");
        process::exit(-1);
    }

    let mut fi = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't read input");
            process::exit(-2);
        }
    };

    let mut fo = match OpenOptions::new().write(true).create(true).open(&args[2]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't write output");
            process::exit(-3);
        }
    };

    let mut i_len = 0;
    let mut o_len = 0;
    let mut num_chars;
    let mut no_spaces = false;
    let mut no_comments = false;
    let mut buffer = [0; 2048];
    while let Ok(n) = fi.read(&mut buffer) {
        if n == 0 {
            break;
        }
        num_chars = n;

        let mut o = 0;
        for mut i in 0..num_chars {
            // remove comments
            if no_comments && buffer[i] == b'*' && buffer[i + 1] == b'/' {
                no_comments = false;
                i += 1;
                continue;
            }
            if no_comments {
                continue;
            }
            if buffer[i] == b'/' && buffer[i + 1] == b'*' {
                no_spaces = false;
                no_comments = true;
                i += 1;
                continue;
            }

            // remove spaces
            if buffer[i] != b' ' && buffer[i] != b'\t' {
                no_spaces = false;
            }
            if buffer[i] == b'\r'
                || buffer[i] == b'\n'
                || (no_spaces && (buffer[i] == b' ' || buffer[i] == b'\t'))
            {
                no_spaces = true;
                continue;
            }
            if buffer[i] == b'{' {
                while o > 1 && (buffer[o - 1] == b' ' || buffer[o - 1] == b'\t') {
                    o -= 1;
                }
            }
            if buffer[i] == b'>' {
                while o > 1 && (buffer[o - 1] == b' ' || buffer[o - 1] == b'\t') {
                    o -= 1;
                }
            }
            if buffer[i] == b':' || buffer[i] == b';' || buffer[i] == b',' || buffer[i] == b'>' {
                no_spaces = true;
            }

            // copy
            buffer[o] = buffer[i];
            o += 1;
        }
        fo.write(&buffer[0..o]).unwrap();
        i_len += num_chars;
        o_len += o;
    }

    println!("From {} chars to {} chars", i_len, o_len);
}
