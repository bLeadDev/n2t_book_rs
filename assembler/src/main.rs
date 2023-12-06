#[allow(unused)]
#[allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::Write;

use assembler::HackParser;


fn main() {

    let hp = HackParser::build(env::args());
    let mut hp = match hp {
        Ok(hp) => hp,
        Err(e) => {
            println!("Error building parser. Error: {e}");
            return;
        }
    };

    let _ = hp.advance();

    let out = File::create("out_test.hack");
    let mut out = out.unwrap();
    let o = vec![1u8, 55, 32, 33, 88, 54, 32];

    let _ = out.write_all(&o);

    while hp.has_more_commands() {
        //println!("{:#?}", hp.command_type());
        let _ = hp.advance();
        let sym = hp.symbol();
        println!("Xxx was: {}", sym);
    }

    println!("Hello, world!");
}
