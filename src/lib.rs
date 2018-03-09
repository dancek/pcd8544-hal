#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_std]

#[macro_use(block)]
extern crate nb;

extern crate embedded_hal;

mod pcd8544_spi;
mod pcd8544_gpio;
mod font;

pub use pcd8544_spi::Pcd8544Spi;
pub use pcd8544_gpio::Pcd8544Gpio;

pub trait Pcd8544 {
    fn command(&mut self, u8);
    fn data(&mut self, u8);

    fn init(&mut self) {
        self.command(0x21); // chip active; horizontal addressing mode (V = 0); use extended instruction set (H = 1)
                            // set LCD Vop (contrast), which may require some tweaking:
        self.command(0xB8); // try 0xB1 (for 3.3V red SparkFun), 0xB8 (for 3.3V blue SparkFun), 0xBF if your display is too dark, or 0x80 to 0xFF if experimenting
        self.command(0x04); // set temp coefficient
        self.command(0x14); // LCD bias mode 1:48: try 0x13 or 0x14

        self.command(0x20); // we must send 0x20 before modifying the display control mode
        self.command(0x0C); // set display control to normal mode: 0x0D for inverse

        self.command(0x80);
        self.command(0x40);
    }

    fn print_char(&mut self, c: u8) {
        let i = (c as usize) - 0x20;

        for c in font::ASCII[i].iter() {
            self.data(*c);
        }
        self.data(0x00);
    }

    fn print(&mut self, s: &str) {
        for c in s.bytes() {
            self.print_char(c);
        }
    }
}