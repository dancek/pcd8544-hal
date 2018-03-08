use embedded_hal::spi::FullDuplex;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::OutputPin;

use Pcd8544;

pub struct Pcd8544Spi<SPI, DC, CS> {
    spi: SPI,
    dc: DC,
    cs: CS,
}

impl<SPI, DC, CS> Pcd8544Spi<SPI, DC, CS>
where
    SPI: FullDuplex<u8>,
    DC: OutputPin,
    CS: OutputPin,
{
    pub fn new(
        spi: SPI,
        dc: DC,
        cs: CS,
        rst: &mut OutputPin,
        delay: &mut DelayMs<u8>,
    ) -> Pcd8544Spi<SPI, DC, CS> {
        rst.set_low();
        delay.delay_ms(10);
        rst.set_high();

        let mut pcd = Pcd8544Spi { spi, dc, cs };
        pcd.init();
        pcd
    }
}

impl<SPI, DC, CS> Pcd8544 for Pcd8544Spi<SPI, DC, CS>
where
    SPI: FullDuplex<u8>,
    DC: OutputPin,
    CS: OutputPin,
{
    fn command(&mut self, cmd: u8) {
        self.dc.set_low();
        self.cs.set_low();
        if let Err(e) = block!(self.spi.send(cmd)) {
panic!();
        }
        if let Err(e) = block!(self.spi.read()) {
panic!();
        }
        self.cs.set_high();
    }

    fn data(&mut self, data: u8) {
        self.dc.set_high();
        self.cs.set_low();
        if let Err(e) = block!(self.spi.send(data)) {
panic!();
        }
        if let Err(e) = block!(self.spi.read()) {
panic!();
        }
        self.cs.set_high();
    }
}
