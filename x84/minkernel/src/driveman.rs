// driver manager
// in mx2 and mx3 avaible
// in mx1 not avaible
// in cpu core 1 or 4

use crate::drivers::driver::Driver;
use crate::drivers::driver::DriverType;

pub struct DriverManager {
    drivers: Vec<Driver>,
}

mod driver;
fn man() {
    let mut DriverManager = DriverManager { drivers: Vec::drivers() }
    DriverManager.add_driver(Driver::new(DriverType::Keyboard));
    DriverManager.add_driver(Driver::new(DriverType::Display));
    DriverManager.add_driver(Driver::new(DriverType::Port));
    DriverManager.add_driver(Driver::new(DriverType::Network));

}
