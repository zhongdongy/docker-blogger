use colored::Colorize;
use notify::event::{CreateKind, ModifyKind, RemoveKind, RenameMode};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

use crate::label_red;
use crate::label_yellow;

fn pathbuf_to_string(pathbufs: &Vec<PathBuf>) -> String {
    let paths: Vec<String> = pathbufs
        .iter()
        .map(|pb| String::from(pb.to_str().unwrap()))
        .collect();

    paths.join(", ")
}

pub fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                EventKind::Modify(mk) => match mk {
                    ModifyKind::Data(dc) => {
                        println!(
                            "{} File `{:?}` should be regenerated (content change)",
                            label_yellow!("[WATCHER]"),
                            dc
                        );
                    }
                    ModifyKind::Name(n) => {
                        if n == RenameMode::To {
                            println!(
                                "{} File `{}` should be regenerated",
                                label_yellow!("[WATCHER]"),
                                pathbuf_to_string(&event.paths)
                            );
                        }
                    }
                    _ => (),
                },
                EventKind::Remove(s) => {
                    // Doesn't needs to be regenerated.
                    let target = match s {
                        RemoveKind::File => "Files",
                        RemoveKind::Folder => "Directories",
                        RemoveKind::Other => "Others",
                        RemoveKind::Any => "Uknown",
                    };
                    println!(
                        "{} {} `{}` removed",
                        label_yellow!("[WATCHER]"),
                        target,
                        pathbuf_to_string(&event.paths)
                    );
                }
                EventKind::Create(c) => {
                    if c == CreateKind::Any {
                        println!(
                            "{} New file `{}` should be generated",
                            label_yellow!("[WATCHER]"),
                            pathbuf_to_string(&event.paths)
                        );
                    }
                }
                _ => (),
            },
            Err(e) => println!(
                "{}{} Watcher error: {:?}",
                label_red!("[ERROR]"),
                label_yellow!("[WATCHER]"),
                e
            ),
        }
    }

    Ok(())
}
