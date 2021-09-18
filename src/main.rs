#![feature(stdio_locked)]

use anyhow::Result;
use clap::Clap;
use crossterm::{event::{read, Event, KeyCode, KeyModifiers}, terminal::{disable_raw_mode, enable_raw_mode}};
use serial::{open, BaudRate, SerialPort};
use std::io::{Read, Write};

mod args;

fn main() -> Result<()> {
    let m = args::TinyComArgs::parse();

    match open(&m.device) {
        Ok(mut com) => {
            match com.reconfigure(&|s| s.set_baud_rate(BaudRate::from_speed(m.baud_rate))) {
                Err(e) => eprintln!("Failed to configure the COM port. {}", e.to_string()),
                Ok(_) => {
                    enable_raw_mode().expect("Failed to setup terminal into raw mode.");

                    std::thread::spawn({
                        move || {
                            let mut cout = std::io::stdout();

                            for b in com.bytes().flatten() {
                                cout.write_all(&[b]).expect("Failed to write to STDOUT.");
                                cout.flush().expect("Failed to flush STDOUT.");
                            }
                        }
                    });

                    process_stdin()?;
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open a COM port. {}", e.to_string());
        }
    }

    Ok(())
}

fn process_stdin() -> Result<()> {
    while let Ok(e) = read() {
        if let Event::Key(e) = e {
            if e.modifiers.contains(KeyModifiers::CONTROL)
                && (e.code == KeyCode::Char('c') || e.code == KeyCode::Char('d'))
            {
                disable_raw_mode().expect("Failed to disable raw mode");
                std::process::exit(0);
            }
        }
    }

    Ok(())
}
