use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("rCLI")
        .version(env!("CARGO_PKG_VERSION"))
        .author("haessal, haessal@mizutamauki.net")
        .about("My command line inferface tools")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(ArgAction::SetTrue)
                .help("Enable debug mode"),
        )
        .get_matches();
    if matches.get_flag("debug") {
        println!("Debug mode enabled");
    }

    println!("Hello, world!");
}
