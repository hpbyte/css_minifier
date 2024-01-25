use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("use: css_minifier PATH_TO_INPUT_FILE OUTPUT_FILE_PATH");
        process::exit(-1);
    }

    let mut fi = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't read input");
            process::exit(-2);
        }
    };

    let mut fo = match File::create(&args[2]) {
        Ok(f) => f,
        Err(_) => {
            println!("can't open output file");
            process::exit(-3);
        }
    };

    let mut i_len = 0;
    let mut o_len = 0;
    let mut buffer = Vec::new();
    let mut no_spaces = false;
    let mut no_comments = false;

    fi.read_to_end(&mut buffer)?;
    let mut output = Vec::new();

    let mut i = 0;
    while i < buffer.len() {
        // remove comments
        if no_comments && buffer[i] == b'*' && buffer.get(i + 1) == Some(&b'/') {
            no_comments = false;
            i += 2;
            continue;
        }
        if no_comments {
            i += 1;
            continue;
        }
        if buffer[i] == b'/' && buffer.get(i + 1) == Some(&b'*') {
            no_spaces = false;
            no_comments = true;
            i += 2;
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
            i += 1;
            continue;
        }
        if buffer[i] == b'{' || buffer[i] == b'>' {
            while output.len() > 1
                && (output[output.len() - 1] == b' ' || output[output.len() - 1] == b'\t')
            {
                output.pop();
            }
        }
        if buffer[i] == b':' || buffer[i] == b';' || buffer[i] == b',' || buffer[i] == b'>' {
            no_spaces = true;
        }

        // copy
        output.push(buffer[i]);
        i += 1;
    }

    fo.write_all(&output)?;
    i_len = buffer.len();
    o_len = output.len();

    println!("From {} chars to {} chars", i_len, o_len);

    Ok(())
}
