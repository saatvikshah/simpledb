use std::io::{self, Write};
use regex::{self, Regex};

enum MetaCommand{
    Exit
}

impl MetaCommand{
    fn is_meta_command(input_str: &str) -> bool {
        input_str.starts_with('.')
    }

    fn categorize(input_str: &str) -> Option<Self>{
        match input_str {
            ".exit" => Some(MetaCommand::Exit),
            _       => None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Statement {
    Insert,
    Select
}

#[derive(Debug)]
enum ParseError{
    RegexCreation(regex::Error),
    NoRegexMatch
}

impl From<regex::Error> for ParseError {
    fn from(err: regex::Error) -> ParseError {
        ParseError::RegexCreation(err)
    }
}

#[derive(Debug)]
struct Schema{
    id: u8,
    username: [char; 32],
    email: [char; 32]
}

fn to_fixlen_array(input: std::str::Chars) -> [char; 32] {
    // TODO: Better way? Also handle overflow case
    let mut result = ['\0'; 32];
    for (i, c) in input.enumerate() {
        result[i] = c;
    }
    result
}

fn parse_insert(input_str: &str) -> Result<Schema, ParseError> {
    let re = Regex::new(r"insert (?P<id>\d+) (?P<username>\w+) (?P<email>\w+@\w+\.\w+)")?;
    let caps = re.captures(input_str);
    match caps {
        Some(captures) => Ok(Schema{id: std::str::FromStr::from_str(&captures["id"]).unwrap(), 
                                            username: to_fixlen_array(captures["username"].chars()),
                                            email: to_fixlen_array(captures["email"].chars())}),
        None => Err(ParseError::NoRegexMatch)
    }
}

#[derive(Debug)]
struct Table {
    rows: Vec<Schema>
}

impl Statement {
    fn categorize(input_str: &str) -> Option<Self> {
        const VALUES: [Statement; 2] = [Statement::Insert, Statement::Select];
        const NAMES: [&str; 2] = ["insert", "select"];
        NAMES.iter().position(|&name| input_str.starts_with(name)).map(|index| VALUES[index])
    }
}

fn main() -> std::io::Result<()> {
    let mut user_input = String::new();
    let mut table = Table{rows : Vec::new() };
    loop {
        print!("db>");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();
        if MetaCommand::is_meta_command(&user_input) {
            match MetaCommand::categorize(&user_input) {
                Some(MetaCommand::Exit) => return Ok(()),
                None => println!("unrecognized meta command: {}", &user_input)
            }
        } else if let Some(stmt) = Statement::categorize(&user_input) {
            if stmt == Statement::Insert {
                // TODO: How to silence warning here about not propagating/returning on error?
                parse_insert(&user_input)
                    .map(|row| table.rows.push(row))
                    .map_err(|e| { println!("Encountered error when parsing insert: {:?}", e); e } );
            } else if stmt == Statement::Select {
                println!("{:?}", table)
            }
        } else {
            println!("Unrecognized input: {}", &user_input);
        }
        user_input.clear();
    }
}
