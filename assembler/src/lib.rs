#[allow(unused_imports)]

use std::env;

use std::error::Error;
use std::fs::File;
use std::io::BufRead;

use std::io::BufReader;

#[allow(unused_imports)]
use std::io::Write;
use std::io;


#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
    None
}

#[allow(dead_code)]
pub struct HackParser{
    current_line_count: usize,
    current_line: String,
    current_command: Option<CommandType>,
    reader: BufReader<File>
}

#[allow(dead_code)]
impl HackParser{
    pub fn build(
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

    pub fn has_more_commands(&self) -> bool{
        !self.reader.buffer().is_empty()
    }


    pub fn advance(&mut self) -> io::Result<()> {
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

    #[allow(dead_code)]
    pub fn command_type(&self) -> &Option<CommandType>{
        return &self.current_command;
    }

    pub fn symbol(&self) -> &str {
        match self.current_command {
            Some(ref command) => match command {
                CommandType::ACommand => self.current_line.strip_prefix('@').unwrap_or(""),
                CommandType::CCommand | CommandType::LCommand | CommandType::None => &self.current_line,
            },
            None => &self.current_line,
        }
    }
    
}


#[cfg(test)]
mod tests {
    use std::path::MAIN_SEPARATOR;

    use super::*;

    #[test]
    fn test_creating_from_file_a_com_1(){
        // GIVEN a string vec with arguments with the filename as second argument, file is valid, has code
        let args = vec!["path/to/here".to_string(), "test_files/test_mixed_com.n2t".to_string()];
        // WHEN building the parser
        let hp = HackParser::build(args.into_iter());
        // THEN it should have the command @32
        let mut hp = hp.unwrap();
        let _ = hp.advance();
        assert!(hp.command_type().as_ref() == Some(&CommandType::ACommand));
    }

    #[test]
    fn test_reading_file(){
        // GIVEN a string vec with arguments with the filename as second argument, file is valid
        let args = vec!["path/to/here".to_string(), "test_files/test_mixed_com.n2t".to_string()];
        // WHEN building the parser
        let hp = HackParser::build(args.into_iter());
        // THEN it should not panic here
        let mut hp = hp.unwrap();
        let _ = hp.advance();
    }
    
    #[test]
    fn test_reading_file_invalid_filename(){
        // GIVEN a string vec with arguments with the filename as second argument, file is invalid
        let args = vec!["path/to/here".to_string(), "test_files/NOT_HERE.n2t".to_string()];
        // WHEN building the parser
        let hp = HackParser::build(args.into_iter());
        // THEN it should panic here    
        let _ = match hp {
            Ok(_) => panic!("Expected error, was OK!"),
            Err(_) => () 
            // TODO: implement error detection for the right kind
        };
    }

    #[test]
    fn test_reading_file_no_filename(){
        // GIVEN a string vec with arguments with the filename as second argument, file name is empty
        let args = vec!["path/to/here".to_string(), "".to_string()];
        // WHEN building the parser
        let hp = HackParser::build(args.into_iter());
        // THEN it should panic here    
        let _ = match hp {
            Ok(_) => panic!("Expected error, was OK!"),
            Err(_) => () 
            // TODO: implement error detection for the right kind
        };
    }
}