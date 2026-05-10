//! Command-line entrypoint for the Lingchong bot migration substrate.

use std::process::ExitCode;

fn main() -> ExitCode {
    lingchong_bot::cli::run(std::env::args())
}
