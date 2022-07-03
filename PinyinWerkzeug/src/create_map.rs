use std::fs::File;
use std::path::Path;
use std::io::{
    BufWriter,
    Write
};

use crate::zi::ZI;

pub mod zi;

fn main() {
	let codegen_exist = Path::new("./src/codegen.rs").exists();

    if codegen_exist {
        println!("{:?}", "codegen.rs exists! You can use convert by convert command.");
    } else {
        build_dictonary();
        println!("{:?}", "codegen.rs is created.");
        println!("Now, you can use convert function to convert mandarin to pinyin.")
    }
}


fn build_dictonary()->std::io::Result<()> {

    let mut builder = phf_codegen::Map::new();
    let mut map_codegen = BufWriter::new(
        File::create("./src/codegen.rs").unwrap());

    for (key, value) in &ZI {
        builder.entry(key, value);
    }

    writeln!(
        &mut map_codegen,
        "pub static ZIDICT: phf::Map<&str, &str> = \n{};\n",
         builder.build()).unwrap();

    Ok(())
}