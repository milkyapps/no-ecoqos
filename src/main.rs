mod w32;

use crate::w32::enum_processes;
use clap::Parser;
use std::collections::HashSet;
use w32::{disable_ecoqos, get_process_name, open_process};

#[derive(Parser, Debug)]
struct Args {
    /// List of processes ID (comma separated)
    #[arg(short, long)]
    id: Option<String>,

    /// List of processes names (comma separated)
    #[arg(short, long)]
    name: Option<String>,

    /// Set as verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Dont print anything
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    let ids: HashSet<usize> = HashSet::from_iter(
        args.id
            .iter()
            .flat_map(|x| x.split(','))
            .map(|x| x.parse().unwrap()),
    );
    let names: Vec<String> = args
        .name
        .unwrap_or(String::new())
        .split(',')
        .map(|x| x.trim().to_lowercase().to_string())
        .collect();

    let pids = enum_processes();

    for pid in pids {
        // id was specified and this does not match it
        if !ids.is_empty() && !ids.contains(&pid) {
            if args.verbose && !args.quiet {
                println!("{pid} filtered out");
            }
            continue;
        }

        let Some(name) = get_process_name(pid) else {
            continue;
        };

        // name was specified and this does not match it
        let lowercase_name = name.to_lowercase().to_string();
        if !names.is_empty() && !names.iter().any(|allowed| lowercase_name.contains(allowed)) {
            if args.verbose && !args.quiet {
                println!("[{name}] filtered out");
            }
            continue;
        }

        let Some(handle) = open_process(pid) else {
            if !args.quiet {
                eprintln!("{pid} OpenProcess failed");
            }
            continue;
        };

        if disable_ecoqos(handle) {
            if args.verbose && !args.quiet {
                println!("{pid} {name} disabled");
            }
        } else {
            if !args.quiet {
                eprintln!("{pid} failed")
            }
        }
    }
}
