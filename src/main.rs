use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("lines-are-rusty")
        .version("0.1")
        .about("Converts lines files from .rm to SVG.")
        .author("Axel Huebl <axel.huebl@plasma.ninja>")
        .arg(
            Arg::with_name("file")
                .help("The file to read from")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .help("The file to save the PDF to")
                .required(true)
                .index(2),
        )
        .get_matches();
    let filename = matches
        .value_of("file")
        .expect("Expected required filename.");
    let output_filename = matches
        .value_of("output")
        .expect("Expected required filename.");

    // Load the file into a Vec<u8>
    let mut f = File::open(filename).unwrap();
    let mut line_file = Vec::<u8>::new();
    f.read_to_end(&mut line_file).unwrap();

    // let max_size_file = 1024 * 1024; // 1mb, or 1024 kilobytes
    // assert!(max_size_file >= line_file.len());

    let page = if &line_file[0..33] == "reMarkable .lines file, version=3".as_bytes() {
        let content = &line_file[43..];
        lines_are_rusty::v3::read_page(&content)
    } else if &line_file[0..33] == "reMarkable .lines file, version=5".as_bytes() {
        println!("{:?}", &line_file[33..43]);
        let content = &line_file[43..];
        lines_are_rusty::v5::read_page(&content).expect("Failed to parse document")
    } else {
        println!("Unsupported line file format:");
        println!("{}", std::str::from_utf8(&line_file[0..33]).unwrap());
        return;
    };

    println!("done.");
    lines_are_rusty::render_svg(&format!("{}.svg", output_filename), &page);
    // lines_are_rusty::render_pdf(&format!("{}.pdf", output_filename), &[page]);
}
