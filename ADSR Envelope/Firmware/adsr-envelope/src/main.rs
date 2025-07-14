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
        // Create variable for flash memory access
        let mut flash = p.FLASH;

        // Configure clock
        let mut rcc = p.RCC.configure().sysclk(48.mhz()).freeze(&mut flash);

        // Create variable for gpioa and gpiob
        let gpioa = p.GPIOA.split(&mut rcc);
        let gpiob = p.GPIOB.split(&mut rcc);

        // Configure PB0 as output
        // TODO        

        // Create tx and rx variables for USART1 (pins PA9 and PA10)
        let  (tx, rx) = cortex_m::interrupt::free(move |cs| {
            (
                gpioa.pa9.into_alternate_af1(cs),
                gpioa.pa10.into_alternate_af1(cs),
            )
        });
        
        // Create struct for serial communication (USART1)
        let mut serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), &mut rcc);
        // let (mut tx, _rx) = serial.split();      
        
        loop {
            // Send message
            print("Sending message...\n", &mut serial);
            // TODO: Turn on LED
            delay();     
            print("Please wait...\n", &mut serial);
            // TODO: Turn off LED
            delay();          
        }
    }

    loop {
        continue;
    }
}

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