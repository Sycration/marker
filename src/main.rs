extern crate pulldown_cmark;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;

use pulldown_cmark::{Parser, Options, html};
use clap::{App, Arg};

fn main() {
    let matches = App::new("marker")
        .version("1.0")
        .author("Sycration V. Xyrozalda <sycration@gmail.com>")
        .about("CLI markdown to HTML app")
        .arg(Arg::with_name("infile")
            .short("i".parse().unwrap())
            .long("infile")
            .about("Markdown file to be parsed")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("outfile")
            .short("o".parse().unwrap())
            .long("infile")
            .about("HTML file that will be written")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("browser")
            .short("b".parse().unwrap())
            .required(false)
            .long("browser")
            .about("Whether or not the browser will automatically be opened"))
        .arg(Arg::with_name("overwrite")
            .long("force_overwrite")
            .required(false)
            .about("overwrite the file"));

    let opts = matches.get_matches();

    let infile; //Declare variable 'cause I'm an idiot
    let outfile; //Declare variable 'cause I'm an idiot

    //don't accept a nonexistant file
    match std::path::Path::new(&opts.value_of("infile").unwrap()).exists() {
        true => infile = std::path::Path::new(opts.value_of("infile").unwrap()),
        _ => {
            println!("{:?} is not an extant file!", opts.value_of("infile"));
            std::process::abort();
        }
    }

    let mut file_use = File::open(infile).expect(" ");
    let mut markdown_input = String::new();
    file_use.read_to_string(&mut markdown_input);

    // Set up options and parser.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&*markdown_input, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    //overwrite or not
    outfile = std::path::Path::new(opts.value_of("outfile").unwrap());
    if outfile.exists() {
        if opts.is_present("overwrite") {
            std::fs::remove_file(outfile);
        }
        else {
            println!("Please use the '--force_overwrite' flag to overwrite the file");
            std::process::abort();
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(outfile);
    let mut file = file.unwrap();
    file.write((&mut html_output).as_ref());

    let uri = format!("file://{}", { format!("{:?}", outfile.canonicalize().unwrap()).replace("\"", "") });


    if opts.is_present("browser") {
        webbrowser::open(&uri);
    }
}