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

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::os::fd::{AsRawFd, FromRawFd};

const CSI: &str = "\x1b[";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[m";
const DIM: &str = "\x1b[2m";

fn main() -> io::Result<()> {
    let reader = reconfigure_stdin()?;

    print_loop(reader)
}

fn print_loop(reader: impl BufRead) -> io::Result<()> {
    let controlling_terminal = File::open("/dev/tty")?;
    let mut stdin = BufReader::new(controlling_terminal);
    let mut dumpster = String::new();

    for line in reader.lines() {
        let line = line?;
        let contents = line.trim();
        println!("{BOLD}{contents}{RESET}");

        stdin.read_line(&mut dumpster)?;
        dumpster.clear();

        cursor_previous_line(2);
        erase_line();
        println!("{DIM}{contents}{RESET}");
    }

    Ok(())
}

fn reconfigure_stdin() -> io::Result<BufReader<File>> {
    let stdin_fd = io::stdin().lock().as_raw_fd();
    let pipe_fd = nix::unistd::dup(stdin_fd)?;

    // BUG: there is nothing that calls close() on pipe_fd,
    // so there is technically a resource leak.
    let pipe = unsafe { File::from_raw_fd(pipe_fd) };

    Ok(BufReader::new(pipe))
}

fn cursor_previous_line(n: u8) {
    print!("{CSI}{n}F")
}

fn erase_line() {
    print!("{CSI}J")
}
