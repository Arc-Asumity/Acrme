// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/arg.rs
// This module handles command-line argument parsing.

use clap::{Arg, ArgAction, Command};

fn spawn_common_help(cmd: Command) -> Command {
    cmd
        .after_help("Author: Arc Asumity <arcasumity@hotmail.com>\n\nLicense:\n  Copyright (c) 2025 Arc Asumity\n  Licensed under the GPLv3 or later License.")
        .arg_required_else_help(true)
}

fn build_cli() -> Command {
    spawn_common_help(
        Command::new("Arcme")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("This is an ACME client for automated certificate management.")
            .disable_version_flag(true)
            // Version
            .arg(
                Arg::new("version")
                    .long("version")
                    .short('V')
                    .help("Show version information")
                    .action(ArgAction::SetTrue),
            )
            // Configure
            .subcommand(spawn_common_help(
                Command::new("config")
                    .about("Manage configuration files and options")
                    .subcommand(spawn_common_help(
                        Command::new("list")
                            .about("List all available configuration items")
                            .arg(
                                Arg::new("path")
                                    .help("Path to the configuration file")
                                    .required(true)
                                    .value_name("PATH"),
                            ),
                    ))
                    .subcommand(spawn_common_help(
                        Command::new("new")
                            .about("Create a new configuration file")
                            .arg(
                                Arg::new("path")
                                    .help("Path to the configuration file")
                                    .required(true)
                                    .value_name("PATH"),
                            ),
                    ))
                    .subcommand(spawn_common_help(
                        Command::new("change")
                            .about("Modify an existing configuration file")
                            .arg(
                                Arg::new("path")
                                    .help("Path to the configuration file")
                                    .required(true)
                                    .value_name("PATH"),
                            ),
                    )),
            ))
            .subcommand(spawn_common_help(
                    Command::new("run")
                    .about("Run the program")
                    .arg(
                        Arg::new("path")
                        .help("Configuration file currently in use at runtime")
                        .required(true)
                        .value_name("PATH"),
                    ),
            ))
            .subcommand(spawn_common_help(
                    Command::new("reload")
                    .about("Reload the configuration for a running process")
                    .arg(
                        Arg::new("socket")
                        .help("Path to the Unix socket used to communicate with the running process")
                        .required(true)
                        .value_name("SOCKET"),
                    ),
            ))
            .subcommand(spawn_common_help(
                    Command::new("stop")
                    .about("Stop a running process")
                    .arg(
                        Arg::new("socket")
                        .help("Path to the Unix socket used to stop the running process")
                        .required(true)
                        .value_name("SOCKET"),
                    ),
            ))
            .subcommand(spawn_common_help(
                    Command::new("log")
                    .about("View the log files of a running process")
                    .arg(
                        Arg::new("socket")
                        .help("Path to the Unix socket used to retrieve log output")
                        .required(true)
                        .value_name("SOCKET"),
                    )
                    .arg(
                        Arg::new("id")
                        .help("Log ID to display")
                        .required(false)
                        .value_name("ID"),
                    ),
            ))
    )
}

pub fn handle_cli() {
    let matches = build_cli().get_matches();
    if matches.get_flag("version") {
        println!("Arcme ACME Client Version {}\nCopyright (c) 2025 Arc Asumity\nLicensed under the GPLv3 or later License.", clap::crate_version!());
        return;
    }
    match matches.subcommand() {
        Some(("config", sub_m)) => {
            match sub_m.subcommand() {
                Some(("list", sub_sub_m)) => {
                    let path = sub_sub_m.get_one::<String>("path").unwrap();
                    // TODO `config list`
                }
                Some(("new", sub_sub_m)) => {
                    let path = sub_sub_m.get_one::<String>("path").unwrap();
                    // TODO `config new`
                }
                Some(("change", sub_sub_m)) => {
                    let path = sub_sub_m.get_one::<String>("path").unwrap();
                    // TODO `config change`
                }
                _ => {
                    println!("[config]: Unknown Arg");
                }
            }
        }
        Some (("run", sub_m)) => {
            let path = sub_m.get_one::<String>("path").unwrap();
            // TODO `run`
        }
        Some (("stop", sub_m)) => {
            let socket = sub_m.get_one::<String>("socket").unwrap();
            // TODO `stop`
        }
        Some (("relaod", sub_m)) => {
            let socket = sub_m.get_one::<String>("socket").unwrap();
            // TODO `reload`
        }
        Some (("log", sub_m)) => {
            let socket = sub_m.get_one::<String>("socket").unwrap();
            let id = sub_m.get_one::<String>("id");
            // TODO `log`
        }
        _ => {
            println!("[config]: Unknown Arg");
        }
    }
}
