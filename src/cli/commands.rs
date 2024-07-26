use clap::{Command, Arg};

fn build_cli() -> Command {
    Command::new("L2 Order Book CLI") 
        .version("1.0")
        .about("Command line interface for the L2 Order Book project") 
        .arg(
            Arg::new("instrument")
                .short('i')
                .long("instrument")
                .value_name("INSTRUMENT")
                .help("Specifies the trading instrument")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("depth-limit")
                .short('d')
                .long("depth-limit")
                .value_name("DEPTH_LIMIT")
                .help("Specifies depth limit for the order book")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("provider")
                .short('p')
                .long("provide")
                .value_name("PROVIDER")
                .help("Specifies provider name to connect to. Only 'Deribit' at the moment supported.")
                .required(false)
                .value_parser(clap::value_parser!(String)),
        )
}

pub fn get_cli_args() -> clap::ArgMatches {
    build_cli().get_matches()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instrument_argument() {
        let cmd = build_cli().try_get_matches_from(vec![
            "test", "--instrument", "BTC-USD"
        ]);
        assert!(cmd.is_ok());
        let matches = cmd.unwrap();
        assert_eq!(matches.get_one::<String>("instrument").unwrap(), "BTC-USD");
    }

    #[test]
    fn test_short_instrument_argument() {
        let cmd = build_cli().try_get_matches_from(vec![
            "test", "-i", "ETH-USD"
        ]);
        assert!(cmd.is_ok());
        let matches = cmd.unwrap();
        assert_eq!(matches.get_one::<String>("instrument").unwrap(), "ETH-USD");
    }
}