use std::fmt;

struct Cli {
    command: Commands,
}

#[derive(Debug)]
enum Commands {
    Parse(Vec<String>), //needs to own collection
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Commands::Parse(_) => write!(f, "parse"),
        }
    }
}
impl Cli {
    fn new(args: &[String]) -> Result<Self, String> {
        // let command = args[0];
        match args[0].as_str() {
            "parse" => Ok(Self {
                command: Commands::Parse(args[1..].to_vec()),
            }),
            _ => Err("error creating cli".to_string()),
        }
    }
    fn run(&self) -> Result<(), String> {
        println!("running cli");
        match self.command {
            Commands::Parse() => println!("running parse"),
        }
        Ok(())
    }
}

fn main() {
    let p = Commands::Parse(Vec::new());
    println!("{p}");
    //get args from terminal
    let args: Vec<String> = std::env::args().collect();
    //if args.len>2 fail
    if args.len() < 2 {
        eprintln!("ivalid command structure: tc [command] <arguments>");
        std::process::exit(1);
    }
    //slice it: remove the first argument
    let slice_args = &args[1..]; //refrencces args but from 1..
    // the other args parsed should contain command and the rest of the arg; //parse --name=Alice
    //sliced arg[0] = parse need to be converted to command\
    let _ = Cli::run(slice_args);
}
