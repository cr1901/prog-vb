#[macro_use]
extern crate failure;
extern crate hidapi;
extern crate exitfailure;

use std::fs::File;
use std::io::prelude::*;
use argparse::{ArgumentParser, Store};
use failure::Error;
use exitfailure::ExitFailure;


pub mod vb_prog {
    use hidapi;

    pub struct FlashBoy {
        dev : hidapi::HidDevice,
    }

    impl FlashBoy {
        pub fn open() -> Result<FlashBoy, failure::Error> {
            let api = hidapi::HidApi::new()?;
            let device = match api.open(0x1781, 0x09a2) {
                Ok(x) => { x },
                Err(_) => { return Err(From::from(Error::FlashboyNotFound)) }
            };

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
            Ok(())
        }
    }

    enum Cmds {
        Erase = 0xA1,
    }

    #[derive(Debug, Fail)]
    pub enum Error {
        #[fail(display = "Could not find Flashboy Plus device")]
        FlashboyNotFound,
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

    Ok(())
}
