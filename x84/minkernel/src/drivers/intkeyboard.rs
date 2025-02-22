// internal keyboard driver
//
//

use crate::arch::cpuio;
use crate::arch::cpuio::inb;
use crate::arch::cpuio::outb;
use crate::arch::cpuio::outw;
use crate::arch::cpuio::inw;

use crate::arch::interrupts;
use crate::arch::interrupts::irq::irq_handler;
use crate::arch::interrupts::irq::irq_install_handler;
use crate::arch::interrupts::irq::IRQ1;
use crate::arch::interrupts::irq::IRQ12;
use crate::arch::interrupts::irq::IRQ15;

fn keyboard_handler() {
    let scancode: u8 = inb(0x60);
    //print!("scancode: {}\n", scancode);
}

pub fn init() {
    unsafe {
        irq_install_handler(IRQ1, keyboard_handler);
        irq_install_handler(IRQ12, keyboard_handler);
        irq_install_handler(IRQ15, keyboard_handler);
    }
}

pub fn get_scancode() -> u8 {
    inb(0x60)
}
fn main() {
    print!("Hello, World!\n");
    init();
    loop {
        let scancode = get_scancode();
        print!("scancode: {}\n", scancode);
    }
    
}

fn input_verify() {
    if {
        input == {"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "y", "z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", ENTER, BACKSPACE, SPACE, TAB, ESCAPE, CAPSLOCK, SHIFT, CTRL, ALT, UP, DOWN, LEFT, RIGHT, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12 }
        input -> sync
        
    }
    else {
        print!("Invalid input\n");
    }
}
