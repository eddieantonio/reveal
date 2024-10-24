// reveal - reveal lines interactively
//
// Copyright (C) 2024 Eddie Antonio Santos
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, IsTerminal};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

// See: https://en.wikipedia.org/wiki/ANSI_escape_code#CSI_(Control_Sequence_Introducer)_sequences
const CSI: &str = "\x1b[";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[m";
const DIM: &str = "\x1b[2m";

// Usage error. See `man sysexits`
const EX_USAGE: i32 = 64;

fn main() -> io::Result<()> {
    let stdin_is_pipe = !io::stdin().is_terminal();
    let filename: Option<String> = env::args().nth(1);

    // Usage:
    if let Some(filename) = filename {
        // reveal FILENAME
        reveal(File::open(filename)?)
    } else if stdin_is_pipe {
        // cmd | reveal
        reveal(reroute_stdin()?)
    } else {
        // *reveal
        usage_error()
    }
}

fn reveal(input: File) -> io::Result<()> {
    let input = BufReader::new(input);
    let controlling_terminal = File::open("/dev/tty")?;
    let mut stdin = BufReader::new(controlling_terminal);
    let mut dumpster = String::new();

    for line in input.lines() {
        let line = line?;
        let contents = line.trim_end();
        println!("{BOLD}{contents}{RESET}");

        stdin.read_line(&mut dumpster)?;
        dumpster.clear();

        // Move up two lines:
        //  - one line created by println!()
        //  - another line introduced by pressing <Enter> at the "prompt"
        cursor_preceding_line(2);
        erase_line();
        println!("{DIM}{contents}{RESET}");
    }

    Ok(())
}

/// Duplicates stdin and returns it as a regular file.
/// Closes stdin, which should be redirected input.
fn reroute_stdin() -> io::Result<File> {
    let stdin_fd = io::stdin().lock().as_raw_fd();
    let pipe_fd = nix::unistd::dup(stdin_fd)?;
    nix::unistd::close(stdin_fd)?;

    // We convert it into an OwnedFD so that the fd will be close()'d when the File is dropped.
    let pipe = unsafe { File::from_raw_fd(pipe_fd) };
    let pipe: OwnedFd = pipe.into();

    Ok(pipe.into())
}

/// Moves the cursor to the nth line before the current line.
///  - ECMA 48, 5th Edition, §8.3.13
fn cursor_preceding_line(n: u8) {
    print!("{CSI}{n}F")
}

/// Erases from the cursor to the end of the current line.
///  - ECMA 48, 5th Edition, §8.3.41
fn erase_line() {
    print!("{CSI}J")
}

/// Incorrect invocation. Show usage and exit with failure.
fn usage_error() -> ! {
    eprintln!("reveal: error: No file specified and input has not been redirected.");
    eprintln!("Usage:");
    eprintln!("\treveal FILENAME");
    eprintln!("\tCOMMAND | reveal");

    std::process::exit(EX_USAGE)
}
