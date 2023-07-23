mod bit_buffer;
mod dict;
mod lzw;

use std::{
    fs::File,
    io::Error,
    time::{Duration, Instant},
};

#[macro_export]
macro_rules! bench {
    {$code:block} => {{
        let _ins = std::time::Instant::now();
        $code
        _ins.elapsed()
    }}
}

fn print_help() {
    println!("usage: lzw [c]ompress|[d]ecompress <in_file> <out_file>")
}

pub fn print_duration(duration: Duration) {
    if duration.as_secs() > 0 {
        println!("Time: {:.1}s", duration.as_secs_f32());
    } else if duration.as_millis() > 0 {
        println!("Time: {}ms", duration.as_millis());
    } else {
        println!("Time: {}µs", duration.as_micros());
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4 {
        print_help();
        return Ok(());
    }

    let in_file_name = &args[2];
    let out_file_name = &args[3];

    let in_file = File::open(&in_file_name)?;
    let out_file = File::create(&out_file_name)?;

    if args[1].starts_with("c") {
        println!("Compressing '{}' in '{}'...", in_file_name, out_file_name);

        let start = Instant::now();
        lzw::compress(in_file, out_file)?;
        let duration = start.elapsed();

        println!(
            "'{}' was compressed successfully in '{}'",
            in_file_name, out_file_name
        );
        print_duration(duration);
    } else if args[1].starts_with("d") {
        println!("Decompressing '{}' in '{}'...", in_file_name, out_file_name);

        let start = Instant::now();
        lzw::decompress(in_file, out_file)?;
        let duration = start.elapsed();

        println!(
            "'{}' was decompressed successfully in '{}'",
            in_file_name, out_file_name
        );
        print_duration(duration);
    } else {
        print_help();
    }

    Ok(())
}
