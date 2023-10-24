pub enum Command {
    GET(String),
    SET(String, String),
    DELETE(String),
}

pub fn parse_command(line: String) -> Result<Command, String> {
    let mut it = line.split(" ");

    match it.next().expect("could not parse value") {
        "GET" => {
            Ok(Command::GET(it.next().expect("could not parse key").to_string()))
        }
        "SET" => {
            Ok(Command::SET(
                it.next().expect("could not parse key").to_string(),
                it.next().expect("could not parse value").to_string()))
        }
        "DELETE" => {
            Ok(Command::DELETE(it.next().expect("could not parse key").to_string()))
        }
        _ => Err("Unknown command".to_string())
    }
}
