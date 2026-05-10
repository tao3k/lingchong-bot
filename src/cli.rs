//! Command-line adapter for Lingchong bot runtime diagnostics.

use std::process::ExitCode;

use crate::runtime::{LiveLaunchPlan, LivePreflightReport, RuntimeConfig};

/// Run the command-line adapter from an argument iterator.
#[must_use]
pub fn run(args: impl IntoIterator<Item = String>) -> ExitCode {
    let command = args.into_iter().nth(1);
    match command.as_deref() {
        Some("check-config") => match RuntimeConfig::from_env() {
            Ok(config) => {
                for line in config.summary().render_lines() {
                    println!("{line}");
                }
                ExitCode::SUCCESS
            }
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        },
        Some("preflight") => match RuntimeConfig::from_env() {
            Ok(config) => {
                for line in config.summary().render_lines() {
                    println!("{line}");
                }
                let report = LivePreflightReport::from_config(&config);
                for line in report.render_lines() {
                    println!("{line}");
                }
                if report.is_ready() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        },
        Some("plan-live") => match RuntimeConfig::from_env() {
            Ok(config) => {
                let report = LivePreflightReport::from_config(&config);
                for line in report.render_lines() {
                    println!("{line}");
                }
                let plan = LiveLaunchPlan::from_config(&config);
                for line in plan.render_lines() {
                    println!("{line}");
                }
                if report.is_ready() && plan.is_runnable() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        },
        Some("--help" | "-h") => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_help();
            ExitCode::FAILURE
        }
        None => {
            println!("lingchong-bot migration substrate is installed");
            print_help();
            ExitCode::SUCCESS
        }
    }
}

fn print_help() {
    println!("usage: lingchong-bot [check-config|preflight|plan-live]");
}
