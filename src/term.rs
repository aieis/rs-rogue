// Copyright © 2015 by Optimal Computing Limited (of New Zealand)
// This code is licensed under the MIT license (see LICENSE-MIT for details)
extern crate libc;

use libc::{c_int,c_uint,c_uchar};
use std::fs;
use std::io;
use std::os::unix::io::AsRawFd;


static STDIN_FILENO: c_int = 0;
static TCSAFLUSH: c_int = 2;

/// Termios structure
#[repr(C)]
pub struct Termios {
    c_iflag: c_uint,
    c_oflag: c_uint,
    c_cflag: c_uint,
    c_lflag: c_uint,
    c_line: c_uchar,
    c_cc: [c_uchar; 32],
    c_ispeed: c_uint,
    c_ospeed: c_uint,
}
impl Termios {
    pub fn new() -> Termios {
        let mut t = Termios {
            c_iflag: 0, c_oflag: 0, c_cflag: 0, c_lflag: 0,
            c_line: 0,
            c_cc: [0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0,
                   0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0],
            c_ispeed: 0, c_ospeed: 0,
        };
        unsafe {
            if tcgetattr(STDIN_FILENO, &mut t) == -1 {
                panic!("Could not call tcgetattr");
            }
        }
        t
    }

    /// Turn echo on
    pub fn echo_on(&mut self) -> () {
        self.c_lflag |= libc::ECHO;
        unsafe {
            if tcsetattr(STDIN_FILENO, TCSAFLUSH, self) == -1 {
                panic!("Could not call tcsetattr");
            }
        }
    }

    /// Turn echo off
    pub fn echo_off(&mut self) -> () {
        self.c_lflag &= !(libc::ECHO | libc::ICANON);
        unsafe {
            if tcsetattr(STDIN_FILENO, TCSAFLUSH, self) == -1 {
                panic!("Could not call tcsetattr");
            }
        }
    }
}

pub fn init() -> Termios {
    let mut termios: Termios = Termios::new();
    termios.echo_off();
    termios
}

#[link(name = "c")]
extern {
    fn tcgetattr(fd: c_int, termios: &mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios: &Termios)
        -> c_int;
}


/* https://github.com/console-rs : unix_term.rs */

pub fn read_key() -> Option<char>{
    let tty_f;
    let fd = unsafe {
        if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            let tmp_tty_f = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open("/dev/tty");

            match tmp_tty_f {
                Ok(t) => {
                    tty_f = t;
                }
                Err(_) => {
                    return None;
                }
            }

            tty_f.as_raw_fd()
        }
    };

    let mut buf: [u8; 1] = [0];
    let count = 1;
    let read_amount = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, count) };
    if read_amount > 0 {
        return Some(buf[0] as char);
    } else {
        return  None;
    }
}


fn read_single_char(fd: i32) -> std::io::Result<Option<char>> {
    // timeout of zero means that it will not block
    let is_ready = poll_fd(fd, -1)?;
    
    if is_ready {
        // if there is something to be read, take 1 byte from it
        let mut buf: [u8; 1] = [0];

        read_bytes(fd, &mut buf, 1)?;
        Ok(Some(buf[0] as char))
    } else {
        //there is nothing to be read
        Ok(None)
    }
}



fn poll_fd(fd: i32, timeout: i32) -> std::io::Result<bool> {
    let mut pollfd = libc::pollfd {
        fd,
        events: libc::POLLIN,
        revents: 0,
    };
    let ret = unsafe { libc::poll(&mut pollfd as *mut _, 1, timeout) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(pollfd.revents & libc::POLLIN != 0)
    }
}

fn read_bytes(fd: i32, buf: &mut [u8], count: u8) -> io::Result<u8> {
    let read = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, count as usize) };
    if read < 0 {
        Err(io::Error::last_os_error())
    } else if read == 0 {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Reached end of file",
        ))
    } else if buf[0] == b'\x03' {
        Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "read interrupted",
        ))
    } else {
        Ok(read as u8)
    }
}

