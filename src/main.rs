//! TCS34725 sensor printing to the console
//! 
//! https://docs.rs/tcs3472/0.1.1/tcs3472/ - crate description
//! 
//! https://ams.com/documents/20143/36005/TCS3472_DS000390_2-00.pdf - sensor datasheet
//! 


#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
//extern crate panic_halt;
extern crate panic_semihosting;
extern crate stm32f0xx_hal as hal;
//extern crate tcs3472;

use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;
//use ssd1306::{prelude::*, Builder as SSD1306Builder};

use tcs3472::{
    Tcs3472,
    RgbCGain,
    RgbCInterruptPersistence,
    };

use crate::hal::{
    prelude::*,
    stm32,
    delay::Delay,
    i2c::I2c,
};

const BOOT_DELAY_MS: u16 = 200;

#[entry]

fn main() -> ! {
    
    let mut p = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        let mut delay = Delay::new(cp.SYST, &rcc);

        delay.delay_ms(BOOT_DELAY_MS);

        let gpiob = p.GPIOB.split(&mut rcc);
        let scl = gpiob.pb8.into_alternate_af1(cs);
        let sda = gpiob.pb7.into_alternate_af1(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), &mut rcc);

        let mut tcs = Tcs3472::new(i2c);

        tcs.enable().unwrap();

        //Enable and read the color measurement

        tcs.enable_rgbc().unwrap();

        //Change the RGB converter gain and integration cycles

        tcs.set_rgbc_gain(RgbCGain::_16x).unwrap();
        tcs.set_integration_cycles(32).unwrap();

        //Enable and configure RGB converter interrupt generation

        tcs.set_rgbc_interrupt_low_threshold(1024).unwrap();
        tcs.set_rgbc_interrupt_high_threshold(61440).unwrap();
        tcs.set_rgbc_interrupt_persistence(RgbCInterruptPersistence::_5).unwrap();
        tcs.enable_rgbc_interrupts().unwrap();

        while !tcs.is_rgbc_status_valid().unwrap() {};

        // let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();

        // disp.init().unwrap();

        loop {

            let clear = tcs.read_clear_channel().unwrap();
            let red = tcs.read_red_channel().unwrap();
            let green = tcs.read_green_channel().unwrap();
            let blue = tcs.read_blue_channel().unwrap();
            
            hprintln!("sensor readings: clear {}, red {}, green {}, blue {}", clear, red, green, blue).unwrap();
            
            delay.delay_ms(2000_u16);

            /*

            for n in 0..32 {
                disp.set_pixel(n%32,n%32,1);
            
                disp.flush().unwrap();

                disp.set_pixel(n%32,n%32,0)
                        
            }

             */

        }

    });


    loop {continue;}

}