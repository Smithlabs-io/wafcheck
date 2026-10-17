use wafcheck::cli;
use clap::Parser;
// use wafcheck::error;

fn main() {
    let args = cli::Input::parse();
    match wafcheck::run(args.command) {
            Ok(output) => println!("{}", output),
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("Exiting with error code: {}", e.exit_code());
                std::process::exit(e.exit_code());
            }
            
        }
}
