use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /* iterators produce series of values
     `collect` on iter -> turns into collection of all elements
     for invalid Unicode arguments:
     - std::env::args_os -> returns itr of OsString values

    (rarely annotate types in Rust)
    but Rust isn't able to infer collection type
     */
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1); // non-zero exits means program exited with error
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // do not need `unwrap_or_else` since return value is ()
    if let Err(e) = minigrep::run(config) {
        println!("App error: {}", e);
        process::exit(1);
    }
}
