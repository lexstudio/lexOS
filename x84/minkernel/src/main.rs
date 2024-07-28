mod lib;
mod init;
mod allocator;
mod str;
mod sync;
mod types;
mod work;



fn main() {
    init::init();
    allocator::allocator();
    str::str();
    sync::sync();
    types::types();
    work::work();
    lib::lib();
     
}
