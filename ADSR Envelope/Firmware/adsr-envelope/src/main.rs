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

        // -------------------------------------------------------------
        // ----------------------- STM32 Setup -------------------------
        // -------------------------------------------------------------

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

           
        // -------------------------------------------------------------
        // ------------------------ GUI Setup --------------------------
        // -------------------------------------------------------------

        // Print module name, revision and designer
        print("\x1b[0mEurorack Module: \x1b[2mADSR Envelope\r\n", &mut serial);
        print("\x1b[0mHardware Revision: \x1b[2mVersion 2.0\r\n", &mut serial);
        print("\x1b[0mDesigner: \x1b[2mFilip K\x1b[0m\r\n", &mut serial);

        // Create the attack, decay, release and sustain bars
        let attack_bar = build_bar(5, 5, "ATTACK", &mut serial);
        let decay_bar = build_bar(5, 20, "DECAY", &mut serial);
        let sustain_bar = build_bar(5, 35, "SUSTAIN", &mut serial);
        let release_bar = build_bar(5, 50, "RELEASE", &mut serial); 

        // // Add temporary bar levels
        // print("\x1b[38;5;32m", &mut serial);    // Cool blue colour
        // draw_multi_level(30, 6, 19, &mut serial); 
        // draw_multi_level(60, 21, 19, &mut serial); 
        // draw_multi_level(10, 36, 19, &mut serial); 
        // draw_multi_level(40, 51, 19, &mut serial);
        
    }

    loop {
        continue;
    }
}

struct Bar<'a> {
    row_init: u32,  
    col_init: u32,  
    label: &'a str, 
    level: u32,
}

fn build_bar<'a, T>(row_init: u32, col_init: u32, label: &'a str, serial: &'a mut T) -> Bar<'a>
where
    T: Write<u8>,
{
    let mut bar_output = Bar {
        row_init: row_init,
        col_init: col_init, 
        label: label, 
        level: 0,
    };

    let row_final: u32 = row_init + 15;
    let col_final: u32 = col_init + 5;

    draw_bar(row_init, row_final, col_init, col_final, serial);
    print_string_at_location(label, col_init, row_final+2, serial);
    print_u32_at_location(bar_output.level, col_init+1, row_final+3, serial);
    print("%", serial); 

    return bar_output;
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

fn print_string_at_location<T>(message: &str, x: u32, y: u32, serial: &mut T)
where 
    T: Write<u8>,
{
    let mut escape_code = String::<16>::new();
    write!(escape_code, "\x1b[{};{}H", y, x).unwrap();            
    print(&escape_code.as_str(), serial);
    print(message, serial);
}

fn print_u32_at_location<T>(message: u32, x: u32, y: u32, serial: &mut T)
where 
    T: Write<u8>,
{
    let mut escape_code = String::<16>::new();
    write!(escape_code, "\x1b[{};{}H", y, x).unwrap();            
    print(&escape_code.as_str(), serial);

    let mut escape_code = String::<16>::new();
    write!(escape_code, "{}", message).unwrap();            
    print(&escape_code.as_str(), serial);    
}


fn draw_line<T>(row_init: u32, row_final: u32, col_init: u32, col_final: u32, serial: &mut T)
where 
    T: Write<u8>,
{
    
    if col_init == col_final {
        // Vertical direction
        for current_row in row_init..row_final+1{            
            print_string_at_location("|", col_init, current_row, serial);
        }      

    } else if row_init == row_final {
        // Horizontal direction        
        for current_col in col_init..col_final+1{                        
            print_string_at_location("-", current_col, row_init, serial);
        }
    }
}

fn draw_bar<T>(row_init: u32, row_final: u32, col_init: u32, col_final: u32, serial: &mut T)
where 
    T: Write<u8>,
{

    // Draw vertical lines
    draw_line(row_init+1, row_final-1, col_init, col_init, serial);
    draw_line(row_init+1, row_final-1, col_final, col_final, serial);

    // Draw horizontal lines
    draw_line(row_init, row_init, col_init+1, col_final-1, serial);
    draw_line(row_final, row_final, col_init+1, col_final-1, serial);

    // Draw corners
    print_string_at_location("/", col_init, row_init, serial);
    print_string_at_location("/", col_final, row_final, serial);
    print_string_at_location("\\", col_final, row_init, serial);
    print_string_at_location("\\", col_init, row_final, serial);  

}

fn draw_level<T>(x_init: u32, y: u32, serial: &mut T)
where 
    T: Write<u8>,
{
    print_string_at_location("#", x_init, y, serial);
    print_string_at_location("#", x_init+1, y, serial);
    print_string_at_location("#", x_init+2, y, serial);
    print_string_at_location("#", x_init+3, y, serial);
}

fn draw_multi_level<T>(percentage: u32, x_init: u32, y: u32, serial: &mut T)
where 
    T: Write<u8>,
{
    // Bar height is by default 14 levels high    
    let num_levels = percentage/7;    

    for level in 0..num_levels{
        draw_level(x_init, y-level, serial);
    }
}
