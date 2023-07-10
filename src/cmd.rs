pub enum Command {
    Get,
    Set,
    Invalid,
}

impl Command {
    pub fn get_command(str: &String) -> Command {
        match str.as_bytes() {
            b"get" => Command::Get,
            b"set" => Command::Set,
            _ => Command::Invalid,
        }
    }
}
