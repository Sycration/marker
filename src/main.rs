extern crate pulldown_cmark;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use webbrowser;
use clap::{Clap, App, Arg};

use pulldown_cmark::{Parser, Options, html};

fn main() {
    let matches = App::new("marker")
        .version("1.0")
        .author("Sycration V. Xyrozalda <sycration@gmail.com>")
        .about("CLI markdown, use the '-b/--browser true' flag to automatically open the file in a browser")
        .arg("-i, --infile=<infile>")
        .arg("-o, --outfile=<outfile>")
        .arg("-b, --browser=[browser]");

    let opts = matches.get_matches();
    let infilez = opts.value_of("infile").unwrap();
    let outfilez = opts.value_of("outfile").unwrap();

    let infile; //Declare variable 'cause I'm retarded
    let outfile; //Declare variable 'cause I'm retarded

    match std::path::Path::new(&infilez).exists() {
        true => infile = std::path::Path::new(&infilez),
        _ => {
            println!("{} is not an extant file!", infilez);
            std::process::abort();
        }
    }

    let mut file_use = File::open(infile).expect(" ");
    let mut markdown_input = String::new();
    file_use.read_to_string(&mut markdown_input);

// Set up options and parser. Strikethroughs are not part of the CommonMark standard
// and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&*markdown_input, options);

// Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

// Check that the output is what we expected.
    //println!("{}", html_output);

    outfile = std::path::Path::new(&outfilez);
    if outfile.exists() {
        std::fs::remove_file(outfile);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(false)
            .open(outfile);
        let mut file = file.unwrap();
        file.write((&mut html_output).as_ref());
    } else {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(outfile);
        let mut file = file.unwrap();
        file.write((&mut html_output).as_ref());
    }
    let uri = format!("file://{}", { format!("{:?}", outfile.canonicalize().unwrap()).replace("\"", "") });

    //if opts.value_of("browser").unwrap() {
    //    webbrowser::open(&uri);
    //}
    if opts.is_present("browser") {
        match opts.value_of("browser").unwrap() {
            "true" | "True" => {
                webbrowser::open(&uri);
                std::process::abort();
            }
            _ => { std::process::abort(); }
        }
    }
}