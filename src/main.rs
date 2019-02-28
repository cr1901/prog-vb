#[macro_use]
extern crate failure;
extern crate hidapi;
extern crate exitfailure;
extern crate indicatif;

use std::fs::File;
use std::io::Read;

use argparse::{ArgumentParser, Store};
use exitfailure::ExitFailure;
use indicatif::{ProgressBar, ProgressStyle};


pub mod vb_prog {
    use hidapi;
    use failure::Fail;

    pub struct FlashBoy {
        dev : hidapi::HidDevice,
    }

    pub struct WriteToken {
        _int : (),
    }

    impl FlashBoy {
        pub fn open() -> Result<FlashBoy, failure::Error> {
            let api = hidapi::HidApi::new()?;
            let device = api.open(0x1781, 0x09a2).or(Err(Error::FlashboyNotFound))?;

            // Plenty of non-FlashBoy devices use Atmel micros, so check for string.
            if !device.get_product_string()?
                      .ok_or(Error::FlashboyNotFound)?
                      .contains("FlashBoy") {
                return Err(From::from(Error::FlashboyNotFound));
            }

            Ok(FlashBoy {
                dev : device,
            })
        }

        pub fn erase(&mut self) -> Result<(), failure::Error> {
            let mut buf = [0; 65];

            buf[1] = Cmds::Erase as u8;
            self.dev.write(&buf)?;

            self.dev.read(&mut buf)?;
            self.check_response(&buf, Cmds::Erase)?;

            Ok(())
        }

        pub fn init_prog(&mut self) -> Result<WriteToken, failure::Error> {
            let mut buf = [0; 65];

            buf[1] = Cmds::StartProg as u8;
            self.dev.write(&buf)?;

            Ok(WriteToken { _int : () })
        }

        pub fn write_chunk(&mut self, _tok: &WriteToken, buf : &[u8; 1024]) -> Result<(), failure::Error> {
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

        fn check_response(&self, buf : &[u8], cmd : Cmds) -> Result<(), Error> {
            if buf[0] == (cmd as u8) {
                return Ok(())
            } else {
                match cmd {
                    Cmds::Erase => { return Err(Error::UnexpectedEraseResponse { code : buf[1] }) },
                    _ => { return Err(Error::UnexpectedWriteResponse { code : buf[1] }) },
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

    #[derive(Debug, Fail)]
    pub enum Error {
        #[fail(display = "Could not find Flashboy Plus device")]
        FlashboyNotFound,

        #[fail(display = "Bad response from FlashBoy after erase command {:X}", code)]
        UnexpectedEraseResponse {
            code : u8,
        },

        #[fail(display = "Bad response from FlashBoy after write command {:X}", code)]
        UnexpectedWriteResponse {
            code : u8,
        },
    }
}


use self::vb_prog::*;

fn main() -> Result<(), ExitFailure> {
    let mut rom = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Command-line Virtual Boy Flash Programmer");
        ap.refer(&mut rom)
            .add_argument("rom", Store, "Virtual Boy ROM image to flash.")
            .required();
        ap.parse_args_or_exit();
    }

    let mut f = File::open(rom)?;
    let mut flash = FlashBoy::open()?;

    println!("Erasing device...");
    flash.erase()?;

    println!("Flashing...");
    let tok = flash.init_prog()?;

    let mut buf = [0; 1024];
    let mut packet_cnt = 0;

    let pb = ProgressBar::new(2048);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:40}] {pos}/{len} packets ({eta})")
        .progress_chars("#>-"));

    while packet_cnt < 2048 {
        f.read_exact(&mut buf)?;
        flash.write_chunk(&tok, &buf)?;
        packet_cnt += 1;
        pb.set_position(packet_cnt);
    }

    pb.finish();
    println!("Image flashed successfully.");
    Ok(())
}
