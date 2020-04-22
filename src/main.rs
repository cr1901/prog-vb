extern crate calm_io;
extern crate eyre;
extern crate hidapi;
extern crate indicatif;

use std::fs::File;
use std::io::Read;

use argparse::{ArgumentParser, Print, Store};
use calm_io::stdoutln;
use eyre::{Report, WrapErr};
use indicatif::{ProgressBar, ProgressStyle};

pub mod vb_prog {
    use super::{Report, WrapErr};
    use hidapi;

    pub const HEADER_LEN: usize = 512 + 32; // ROM metadata + Interrupt vectors

    pub struct FlashBoy {
        dev: hidapi::HidDevice,
    }

    pub struct WriteToken {
        _int: (),
    }

    impl FlashBoy {
        pub fn open() -> Result<FlashBoy, Report> {
            let api = hidapi::HidApi::new()?;
            let device = api
                .open(0x1781, 0x09a2)
                .or_else(|e| Err(Error::FlashboyNotFound(Some(e))))
                .wrap_err("No USB device with VID:PID 0x1781:0x09a2 was found.")?;

            // Plenty of non-FlashBoy devices use Atmel micros, so check for string.
            if !device
                .get_product_string()
                .or_else(|e| Err(Error::FlashboyNotFound(Some(e))))?
                .ok_or(Error::FlashboyNotFound(None))
                .wrap_err(
                    "USB device with VID:PID 0x1781:0x9a2 found, but the product string\n\
                     was empty. Stopping to be safe.",
                )?
                .contains("FlashBoy")
            {
                return Err(Error::FlashboyNotFound(None)).wrap_err(
                    "A USB device with VID:PID 0x1781:0x9a2 was found, but it wasn't a\n\
                     FlashBoy. Do you have multiple Atmel devices attached?",
                );
            }

            Ok(FlashBoy { dev: device })
        }

        pub fn erase(&mut self) -> Result<(), Report> {
            let mut buf = [0; 65];

            buf[1] = Cmds::Erase as u8;
            self.dev.write(&buf)?;

            self.dev.read(&mut buf)?;
            self.check_response(&buf, Cmds::Erase)?;

            Ok(())
        }

        pub fn init_prog(&mut self) -> Result<WriteToken, Report> {
            let mut buf = [0; 65];

            buf[1] = Cmds::StartProg as u8;
            self.dev.write(&buf)?;

            Ok(WriteToken { _int: () })
        }

        pub fn write_chunk(&mut self, _tok: &WriteToken, buf: &[u8; 1024]) -> Result<(), Report> {
            let mut packet = [0; 65];

            packet[1] = Cmds::Write1024 as u8;
            self.dev.write(&packet)?;

            for p in buf.chunks_exact(64) {
                let (_, payload) = packet.split_at_mut(1);
                payload.clone_from_slice(p);
                self.dev.write(&packet)?;
            }

            self.dev.read(&mut packet)?;
            self.check_response(&packet, Cmds::Write1024)?;

            Ok(())
        }

        fn check_response(&self, buf: &[u8], cmd: Cmds) -> Result<(), Error> {
            if buf[0] == (cmd as u8) {
                Ok(())
            } else {
                match cmd {
                    Cmds::Erase => Err(Error::UnexpectedEraseResponse { code: buf[1] }),
                    _ => Err(Error::UnexpectedWriteResponse { code: buf[1] }),
                }
            }
        }
    }

    #[derive(Clone, Copy)]
    enum Cmds {
        Erase = 0xA1,
        StartProg = 0xB0,
        Write1024 = 0xB4,
    }

    #[derive(Debug)]
    pub enum Error {
        FlashboyNotFound(Option<hidapi::HidError>),
        UnexpectedEraseResponse { code: u8 },
        UnexpectedWriteResponse { code: u8 },
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::FlashboyNotFound(_) => write!(f, "FlashBoy was not found"),
                Error::UnexpectedEraseResponse { code } => write!(
                    f,
                    "Unexpected response {} when erasing, expected {}",
                    code,
                    Cmds::Erase as u8
                ),
                Error::UnexpectedWriteResponse { code } => write!(
                    f,
                    "Unexpected response {} when writing, expected {}",
                    code,
                    Cmds::Write1024 as u8
                ),
            }
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                // Error::FlashboyNotFound(Some(e)) => Some(&e), Why does this error?
                Error::FlashboyNotFound(Some(e)) => Some(e),
                _ => None,
            }
        }
    }
}

use self::vb_prog::*;

fn main() -> Result<(), Report> {
    let mut rom = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Command-line Virtual Boy Flash Programmer");
        ap.add_option(
            &["-v"],
            Print(
                option_env!("CARGO_PKG_VERSION")
                    .unwrap_or("No version- compiled without Cargo.")
                    .to_string(),
            ),
            "Show version.",
        );
        ap.refer(&mut rom)
            .add_argument("rom", Store, "Virtual Boy ROM image to flash.")
            .required();
        ap.parse_args_or_exit();
    }

    let mut f = File::open(&rom)?;
    let f_len = f.metadata()?.len();

    if !(f_len > 16 * 1024 && f_len <= 2 * 1024 * 1024 && f_len.is_power_of_two()) {
        let f_err = std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Input ROM was less than 16kB in length, greater than 2MB in length,\n\
             or a non power of two length.",
        );
        return Err(From::from(f_err));
    }

    let mut flash = FlashBoy::open()?;

    stdoutln!("Erasing device (5-10 seconds)...")?;
    flash.erase()?;

    stdoutln!("Flashing...")?;
    let tok = flash.init_prog()?;

    let mut buf = [0; 1024];
    let mut header = [0; HEADER_LEN];
    let mut packet_cnt = 0;
    let header_packet = (f_len / 1024) - 1;

    let pb = ProgressBar::new(2048);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} packets ({eta})")
            .progress_chars("#>-"),
    );

    while packet_cnt < 2048 {
        if packet_cnt <= header_packet {
            f.read_exact(&mut buf).wrap_err(format!(
                "File {} must be read in 1024-byte chunks.\n\
                 Chunk {} was not read properly.",
                rom, packet_cnt
            ))?;
        } else {
            // Flashboy optimizes for 0xFF chunks when programming.
            for i in buf.iter_mut() {
                *i = 0xFF;
            }
        }

        // We only need to pad if the ROM is < 2MB.
        if header_packet != 2047 {
            if packet_cnt == header_packet {
                header.copy_from_slice(buf.split_at_mut(1024 - HEADER_LEN).1);
            } else if packet_cnt == 2047 {
                buf.split_at_mut(1024 - HEADER_LEN)
                    .1
                    .copy_from_slice(&header);
            }
        }

        flash.write_chunk(&tok, &buf)?;
        packet_cnt += 1;
        pb.set_position(packet_cnt);
    }

    pb.finish();
    stdoutln!("Image flashed successfully.")?;
    Ok(())
}
