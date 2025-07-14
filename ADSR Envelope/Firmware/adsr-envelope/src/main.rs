#![no_main]
#![no_std]

use panic_halt as _;
use stm32f0xx_hal::{self as hal, gpio, pac::{usart1, USART1}, serial};
use crate::hal::{pac, prelude::*, serial::Serial};
use cortex_m_rt::entry;
use nb;
use embedded_hal::serial::Write;

#[entry]
fn main() -> ! {
    if let Some(p) = pac::Peripherals::take() {
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut flash);
        let gpioa = p.GPIOA.split(&mut rcc);

        let  (tx, rx) = cortex_m::interrupt::free(move |cs| {
            (
                gpioa.pa9.into_alternate_af1(cs),
                gpioa.pa10.into_alternate_af1(cs),
            )
        });
        
        let serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), &mut rcc);
        let (mut tx, _rx) = serial.split();        
        
        loop {
            // Send message
            print("Sending message...\n", &mut tx);
            delay();          
        }
    }

    loop {
        continue;
    }
}

    // let dp = stm32f0x0::Peripherals::take().unwrap();

    // // Enable GPIOB clock
    // dp.RCC.ahbenr.modify(|_, w| w.iopben().set_bit());

    // // Configure PB0 as output
    // dp.GPIOB.moder.modify(|_, w| w.moder0().bits(0b01)); // output mode
    // dp.GPIOB.otyper.modify(|_, w| w.ot0().clear_bit());  // push-pull    

    // loop {
    //     // Toggle PB0 ON
    //     dp.GPIOB.bsrr.write(|w| w.bs0().set_bit());
    //     delay();

    //     // Toggle PB0 OFF
    //     dp.GPIOB.bsrr.write(|w| w.br0().set_bit());
    //     delay();
    // }
fn print<T>(message: &'static str, serial: &mut T)
where 
    T: Write<u8>,
    
{
    for &byte in message.as_bytes() {
        nb::block!(serial.write(byte)).ok();
    }  
}

fn delay() {
    for _ in 0..1_000_000 {
        cortex_m::asm::nop();
    }
}