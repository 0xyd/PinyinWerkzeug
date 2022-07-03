use std::path::Path;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {

    /// Test
    #[clap(short = 'i', value_parser, value_name = "INPUT_TEXT")]
    input_txt: String,
}

fn main() {
    
    if Path::new("./src/codegen.rs").is_file() {
        println!("{:?}", "codegen exists!! Let's continue the conversion.");
        // use crate::codegen::ZIDICT;
        // pub mod ZIDICT;
    } else {
        panic!("codegen.rs doesn't exist. Please run create_map first.");
    }
}