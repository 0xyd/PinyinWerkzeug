use std::path::Path;
use clap::Parser;
use crate::codegen::ZIDICT;

pub mod codegen;

#[derive(Parser, Debug)]
struct Args {

    #[clap(short = 'i', value_parser, value_name = "INPUT_TEXT")]
    input_txt: String,
}

fn convert_pinyin(txt: String) -> String {
    let split = txt.split("");
    let mut pinyin_txt = String::from("");
    // let mut pinyin_txt:Vec<String> = Vec::new() ;

    for s in split.filter(|&x| !x.is_empty()) {
        let pinyin = ZIDICT.get(s).cloned();
        match pinyin {
            Some(p) => {
                pinyin_txt.push_str(p);
            },
            None => {
                println!("Word doesn't match with any.");
            }
        }

    }
    pinyin_txt
}

fn main() {

    if Path::new("./src/codegen.rs").is_file() {
        println!("{:?}", "codegen exists!! Let's continue the conversion.");
    } else {
        panic!("codegen.rs doesn't exist. Please run create_map first.");
    }

    let args = Args::parse();
    let pinyin_txt = convert_pinyin(args.input_txt);

    println!("PINYIN: {:?}", pinyin_txt.trim());
}