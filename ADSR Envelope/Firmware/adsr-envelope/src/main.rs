#![no_main]
#![no_std]

use panic_halt as _;
use stm32f0xx_hal::{self as hal, gpio, pac::{usart1, USART1}, serial};
use crate::hal::{pac, prelude::*, serial::Serial};
use cortex_m_rt::entry;
use nb;
use embedded_hal::{blocking::serial::write, serial::Write};
use core::fmt::Write as OtherWrite;
use heapless::{self, String};

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
           
        
        
        // Draw one bar        
        draw_line(5, 20, 10, 10, &mut serial);
        draw_line(5, 20, 15, 15, &mut serial);
        draw_line(20, 20, 11, 15, &mut serial);
        draw_line(4, 4, 11, 15, &mut serial);
        

        /*

        /---\               
        |   |
        |   |
        |   |
        |   |
        |   |
        |   |
        |###|
        |###|
        |###|
        \---/

        ATTACK
         30%       
             
        */
        
        
    }

    loop {
        continue;
    }
}

fn print<T>(message: &str, serial: &mut T)
where 
    T: Write<u8>,
    
{
    for &byte in message.as_bytes() {
        nb::block!(serial.write(byte)).ok();
    }  
}

fn delay() {
    for _ in 0..500_000 {
        cortex_m::asm::nop();
    }
}


fn draw_line<T>(row_init: u32, row_final: u32, col_init: u32, col_final: u32, serial: &mut T)
where 
    T: Write<u8>,
{
    
    if col_init == col_final {
        // Vertical direction
        for current_row in row_init..row_final{
            // Print escape code            
            let mut escape_code = String::<16>::new();
            write!(escape_code, "\x1b[{};{}H", current_row, col_init).unwrap();            
            print(&escape_code.as_str(), serial);            

            // Print vertical line            
            print("|", serial);
        }      

    } else if row_init == row_final {
        // Horizontal direction        
        for current_col in col_init..col_final{                        
            // Print escape code            
            let mut escape_code = String::<16>::new();
            write!(escape_code, "\x1b[{};{}H", row_init, current_col).unwrap();            
            print(&escape_code.as_str(), serial);            

            // Print horizontal line            
            print("-", serial);
        }
    }
}