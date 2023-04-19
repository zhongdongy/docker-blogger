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
            "[{}] About to watch for file changes in `{}`",
            label_yellow!("WATCHER"),
            config.directory
        );
        if let Err(e) = watch(String::from(config.directory)) {
            println!(
                "[{}] Unable to watch for file system changes: {:?}",
                label_red!("ERROR"),
                e
            )
        }
    }

    if config.server {
        println!("[{}] Ready to start blogger server on http://127.0.0.1:{}", label_green!("SERVER"), config.port.unwrap_or(8080));
        println!("[{}] Rebuild all pages via http://127.0.0.1:{}/admin/cache-reload/", label_green!("SERVER"), config.port.unwrap_or(8080));
        let port = config.port.clone();
        let th = thread::spawn(move || run_server(port));
        match th.join() {
            Ok(res) => match res {
                Ok(_) => {
                    println!(
                        "[{}] Server Stopped",
                        label_yellow!("SERVER")
                    );
                }
                Err(e) => println!("[{}] Server stopped unexpectedly: {:?}", label_red!("SERVER"), e.to_string()),
            },
            Err(e) => eprintln!("[{}] Unable to start server: {:?}", label_red!("SERVER"), e),
        }
    }
}
