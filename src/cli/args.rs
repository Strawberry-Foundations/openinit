#[derive(Default)]
pub struct Args {
    pub args: Vec<String>,
    pub command: Command,
    pub command_str: String,
    
}

#[derive(Default)]
pub enum Command {
    #[default]
    None,
    Help,
    Reboot,
}

impl Args {
    pub fn collect() -> Self {
        let mut args = Self {
            args: vec![],
            command: Command::None,
            command_str: String::new(),
        };

        let collector: Vec<String> = std::env::args().collect();

        if collector.len() <= 1 {
            return args
        }

        let parser: Vec<String> = std::env::args().skip(1).collect();

        args.args.clone_from(&parser);
        args.command_str = parser.clone().first().unwrap().to_string();

        match args.command_str.as_str() {
            "help" => args.command = Command::Help,
            "reboot" => args.command = Command::Reboot,
            _ => args.command = Command::None,
        }

        args
    }
}