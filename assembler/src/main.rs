#[allow(dead_code)]

use std::cmp::Ordering;
use std::default;
use std::env;
use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::process;
use std::io;

pub struct Config {
    pub file_name: String
}


#[derive(Debug)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    None
}

pub struct HackParser{
    current_line_count: usize,
    current_line: String,
    current_command: Option<CommandType>,
    reader: BufReader<File>
}

impl HackParser{
    fn build(
        mut args: impl Iterator<Item = String>
    ) -> Result<Self, Box<dyn Error>> {
        args.next(); // Skip Path
        let file_name = args.next().unwrap_or_default();
        if file_name.is_empty(){
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "No filename provided.")));
        }
        let file = File::open::<String>(file_name);
        let file = match file {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
        };

        let reader = BufReader::new(file);
        
        Ok(HackParser {
            reader,
            current_line_count: 0,
            current_line: String::new(),
            current_command: None
        })
    }

    fn has_more_commands(&self) -> bool{
        !self.reader.buffer().is_empty()
    }


    fn advance(&mut self) -> io::Result<()> {
        self.current_line.clear();
        if self.reader.read_line(&mut self.current_line)? > 0 {
            self.current_line_count += 1;
            // decode command types
            if self.current_line.starts_with('@'){
                println!("found A command: {}", self.current_line);
                self.current_command = Some(CommandType::ACommand);
            }else{
                println!("found C command: {}", self.current_line);
                self.current_command = Some(CommandType::ACommand);
            }
        }

        Ok(())
    }

    fn command_type(&self) -> &Option<CommandType>{
        return &self.current_command;
    }

    fn symbol(&self) -> &str {
        match self.current_command {
            Some(ref command) => match command {
                CommandType::ACommand => self.current_line.strip_prefix('@').unwrap_or(""),
                CommandType::CCommand | CommandType::LCommand | CommandType::None => &self.current_line,
            },
            None => &self.current_line,
        }
    }
    
} 



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
