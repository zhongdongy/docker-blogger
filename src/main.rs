//! Command Line Interface for Eastwind Blogger (Rust)
//!
//! Running the program without options has no effects.
//!
//! # Options
//!
//! - `-s | --server` Start a Actix-Web server
//! - `-w | --watch` Watching for blog contents changes.
//! - `-d | --dir` Blog contents directory.

////////////////////////////////////////////////////////////////////////////////

use std::env;
use std::thread;

use colored::Colorize;
use eastwind_blogger::label_green;
use eastwind_blogger::label_red;
use eastwind_blogger::label_yellow;
use eastwind_blogger::run_server;
use eastwind_blogger::utils::bin::parse_options;
use eastwind_blogger::watch;

fn main() {
    let ops: Vec<String> = env::args().collect();
    let config = parse_options(&ops);
    config.check().unwrap();

    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    if config.watch {
        println!(
            "{} About to watch for file changes in `{}`",
            label_yellow!("WATCHER"),
            config.directory
        );
        if let Err(e) = watch(String::from(config.directory)) {
            println!(
                "{} Unable to watch for file system changes: {:?}",
                label_red!("[ERROR]"),
                e
            )
        }
    }

    if config.server {
        println!("{} Ready to start blogger server", label_green!("SERVER"));
        let th = thread::spawn(|| run_server());
        let _ = th.join().unwrap();
    }
}
