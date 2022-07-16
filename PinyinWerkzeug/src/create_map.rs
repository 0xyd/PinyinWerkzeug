use std::fs::File;
use std::path::Path;
use std::io::{
	BufReader, 
	BufRead,
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
        build_tmp();
        build_dictonary();
        println!("{:?}", "codegen.rs is created.");
        println!("Now, you can use convert function to convert mandarin to pinyin.")
    }
}

fn build_dictonary()->std::io::Result<()> {

    let tmp = File::open("./src/tmp_codegen.rs")?;
    let reader = BufReader::new(tmp);
    let mut flag = false;
    let mut map_codegen = BufWriter::new(
        File::create("./src/codegen.rs").unwrap());

    for line in reader.lines() {

        let t = line?;

        println!("t:{:?}\n", t);

        if flag {

            if t.contains("],") {
                println!("{:?}", "t.contains(\"],\")");
                write!(map_codegen, "],\n").ok();
                flag = false;
                continue;
            }
            
            let tt = t.split("\",");
            let mut count = 0;
            for ttt in tt {
                
                if count > 0 {
                    let pinyinlen = ttt.len();
                    let pinyin = &ttt[..pinyinlen-2];
                    write!(map_codegen, "\"{}\"),\n", pinyin).ok();
                } else {
                    write!(map_codegen, "{}\",", ttt).ok();    
                }
                
                count += 1;
            }

        } else {

            if t.contains("entries: &[") {
                flag = true;
            }
            write!(map_codegen, "{}\n", t).ok();
        }
    }

    Ok(())
}


fn build_tmp()->std::io::Result<()> {

    let mut tmp_file = File::create("./src/tmp_codegen.rs").unwrap();
    let mut builder = phf_codegen::Map::new();
    let mut tmp_codegen = BufWriter::new(tmp_file);

    for (key, value) in &ZI {
        builder.entry(key, &value);
    }

    writeln!(
        &mut tmp_codegen,
        "pub static ZIDICT: phf::Map<&str, &str> = \n{};\n",
         builder.build())?;

    // let input = File::open("./src/tmp_codegen.rs")?;
    // let reader = BufReader::new(input);
    // let mut flag = false;
    // let mut map_codegen = BufWriter::new(
    //     File::create("./src/codegen.rs").unwrap());

    // for line in reader.lines() {

    //     let t = line?;

    //     println!("t:{:?}\n", t);

    //     if flag {

    //         if t.contains("],") {
    //             println!("{:?}", "t.contains(\"],\")");
    //             write!(map_codegen, "],\n").ok();
    //             flag = false;
    //             continue;
    //         }
            
    //         let tt = t.split("\",");
    //         let mut count = 0;
    //         for ttt in tt {
                
    //             if count > 0 {
    //                 let pinyinlen = ttt.len();
    //                 let pinyin = &ttt[..pinyinlen-2];
    //                 write!(map_codegen, "\"{}\"),\n", pinyin).ok();
    //             } else {
    //                 write!(map_codegen, "{}\",", ttt).ok();    
    //             }
                
    //             count += 1;
    //         }

    //     } else {

    //         if t.contains("entries: &[") {
    //             flag = true;
    //         }
    //         write!(map_codegen, "{}\n", t).ok();
    //     }
    // }

    Ok(())
}