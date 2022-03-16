use clap::{Arg, Command};

#[tokio::main]
pub async fn main() {
    const COMMAND: &str = "command";
    const COMMAND_INITIALIZE: &str = "initialize";
    const COMMAND_CREATE_NFT: &str = "create-nft";
    const PARAMS_STATS: &str = "stats";

    let app = Command::new("rust-opensea")
        .version("0.1.0")
        .author("akiho <aki030402@mail.com>")
        .about("OpenSea CLI")
        .arg(
            Arg::new(COMMAND)
                .help("exec command name")
                .short("c".parse().unwrap())
                .long(COMMAND)
                .possible_values(&[COMMAND_INITIALIZE, COMMAND_CREATE_NFT])
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new(PARAMS_STATS)
                .help("opensea status")
                .short("s".parse().unwrap())
                .long(PARAMS_STATS)
                .multiple_values(true)
                .required(false)
                .takes_value(true),
        );

    let matches = app.get_matches();

    let command = matches.value_of(COMMAND).unwrap();
    let stats_list: Vec<_> = matches
        .values_of(PARAMS_STATS)
        .unwrap_or_default()
        .collect();

    println!("Value for command: {}", command);
    println!("Value for kv: {:?}", stats_list);
}
