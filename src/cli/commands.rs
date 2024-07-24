use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("Order Book CLI") // todo: rename
        .version("1.0")
        .about("Command line interface for the Order Book project") //todo: rename 
        .arg(
            Arg::new("instrument")
                .short('i')
                .long("instrument")
                .value_name("INSTRUMENT")
                .help("Specifies the trading instrument")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
}

pub fn get_matches() -> clap::ArgMatches {
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
    fn test_missing_instrument_argument() {
        let cmd = build_cli().try_get_matches_from(vec![
            "test"
        ]);
        assert!(cmd.is_err());
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