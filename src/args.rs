use clap::Clap;

pub fn parse() -> Arguments {
    Arguments::parse()
}

#[derive(Debug, Clap)]
#[clap(
    version = "1.0",
    author = "Antoine POPINEAU <antoine.popineau@appscho.com"
)]
pub struct Arguments {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Clap)]
pub enum Subcommand {
    #[clap(name = "config", about = "Configure htb")]
    Config(Config),
    #[clap(
    name = "machines",
    about = "Interact with machines from your lab",
    aliases = &["machine", "box", "vm"]
  )]
    Machines(Machines),
    #[clap(name = "chat", about = "Interact with your chat conversations")]
    Chat(Chat),
}

#[derive(Debug, Clap)]
pub enum Config {
    #[clap(name = "token", about = "Set your Hack The Box API key")]
    Token {
        #[clap(name = "TOKEN", help = "Your API key from your account page")]
        token: String,
    },
}

#[derive(Debug, Clap)]
pub enum Machines {
    #[clap(
        name = "list",
        about = "List all machines from your lab, with optional filters"
    )]
    List(MachineListArgs),
    #[clap(name = "show", about = "Show information about a specific machine")]
    Show {
        #[clap(name = "NAME", help = "Name of the machine to be displayed")]
        name: String,
    },
    #[clap(name = "own", about = "Submit a flag for a specific machine")]
    Own {
        #[clap(name = "NAME", help = "Name of the machine to be owned")]
        name: String,
        #[clap(
            name = "FLAG",
            long = "flag",
            short = "f",
            help = "Flag to be submitted"
        )]
        flag: String,
        #[clap(
      name = "DIFFICULTY",
      long = "difficulty",
      short = "d",
      help = "Level of perceived difficulty to get the flag",
      parse(try_from_str = parse_difficulty)
    )]
        difficulty: u8,
    },
    #[clap(name = "todo", about = "Toggle a machine from/into your to-do list")]
    Todo {
        #[clap(name = "name")]
        name: String,
    },
    #[clap(name = "reset", about = "Request for a machine to be reset")]
    Reset {
        #[clap(name = "NAME", help = "Name of the machine to be reset")]
        name: String,
    },
    #[clap(name = "start", about = "Spawn a stopped machine")]
    Start {
        #[clap(name = "NAME", help = "Name of the machine to be spawned")]
        name: String,
    },
    #[clap(name = "stop", about = "Terminate a machine")]
    Stop {
        #[clap(name = "stop", help = "Name of the machine to be terminated")]
        name: String,
    },
}

#[derive(Debug, Clap)]
pub enum Chat {
    #[clap(name = "list", about = "List all conversations")]
    List,
    #[clap(name = "show", about = "List latest messages with a set of users")]
    Show {
        #[clap(name = "ID", help = "ID of the target conversation")]
        id: u64,
    },
    #[clap(name = "send", about = "Send a message to a conversation")]
    Send {
        #[clap(name = "ID", help = "ID of the target conversation", required = true)]
        id: u64,
        #[clap(name = "MESSAGE", help = "Message to be sent", required = true)]
        message: String,
    },
    #[clap(name = "open")]
    Open {
        #[clap(name = "ID", help = "ID of the target conversation", required = true)]
        id: u64,
    },
}

#[derive(Debug, Clap)]
pub struct MachineListArgs {
    #[clap(long, help = "Only show spawned machines")]
    pub spawned: bool,
    #[clap(long, group = "group_active", help = "Only show active machines")]
    pub active: bool,
    #[clap(long, group = "group_active", help = "Only show retired machines")]
    pub retired: bool,
    #[clap(
        long,
        group = "group_owned",
        help = "Only show machines you completely onwned"
    )]
    pub owned: bool,
    #[clap(
        long,
        group = "group_owned",
        help = "Only show machines you did not complete"
    )]
    pub unowned: bool,
    #[clap(long, help = "Show machines from your to-do list")]
    pub todo: bool,
    #[clap(long, help = "Show machines assigned to you")]
    pub assigned: bool,
    #[clap(long, name = "NAME")]
    pub name: Option<String>,
}

fn parse_difficulty(value: &str) -> Result<u8, &'static str> {
    match value.parse::<u8>() {
        Ok(value) => {
            if value > 0 && value <= 10 {
                Ok(value)
            } else {
                Err("should be an integer between 1 and 10")
            }
        }

        Err(_) => Err("should be an integer between 1 and 10"),
    }
}
