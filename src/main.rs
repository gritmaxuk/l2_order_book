use l2_order_book::cli::get_matches;

fn main() {
    let matches = get_matches();

    if let Some(instrument) = matches.get_one::<String>("instrument") {
        println!("Instrument specified: {}", instrument); // todo: add logger
        // todo: logic here 
    } else {
        eprintln!("Instrument not specified!");
    }
}