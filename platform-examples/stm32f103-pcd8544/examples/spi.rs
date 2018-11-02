#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

extern crate cortex_m;
extern crate embedded_hal;
extern crate stm32f103xx_hal as hal;

use hal::stm32f103xx;
use hal::delay::Delay;
use hal::spi::Spi;
use hal::prelude::*;
use hal::gpio::{gpioa, Output, PushPull};

use embedded_hal::spi;

extern crate pcd8544_hal;
use pcd8544_hal::Pcd8544Spi;

fn main() {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let clocks = rcc.cfgr
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    let mut delay = Delay::new(cp.SYST, clocks);

    // setup SPI
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let spi_mode = spi::Mode {
        phase: spi::Phase::CaptureOnFirstTransition,
        polarity: spi::Polarity::IdleLow,
    };

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        spi_mode,
        4.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    // other pins for PCD8544
    let dc: gpioa::PA4<Output<PushPull>> = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.crl)
        .into(); // PA4
    let cs: gpioa::PA3<Output<PushPull>> = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.crl)
        .into(); // PA3
    let mut rst: gpioa::PA1<Output<PushPull>> = gpioa
        .pa1
        .into_push_pull_output(&mut gpioa.crl)
        .into(); // PA1

    let mut pcd8544 = Pcd8544Spi::new(spi, dc, cs, &mut rst, &mut delay);

    pcd8544_hal::demo::demo(&mut pcd8544);
}
