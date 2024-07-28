use core::memory;
use core::kernel;


fn BUFFER() {
    in kernel::BUFFER;
    for i in 0..BUFFER.len() {
        BUFFER[i] = 0;
    }
    set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
    use relm4::{ComponentSender, Worker};

    pub struct AsyncHandler;

    #[derive(Debug)]
    pub enum AsyncHandlerInput {}

    impl Worker for AsyncHandler {
        type Init = ();
        type Input = AsyncHandlerInput;
        type Output = ();

        fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
            Self
        }
        

        fn update(&mut self, msg: AsyncHandlerInput, sender: ComponentSender<Self>) {
            match msg {}
        }
    }
    impl AsyncHandler {
        pub fn new() -> ComponentSender<Self> {
            relm4::create_worker::<Self>(())
        }
    }
    let _ = AsyncHandler::new();
    for _ in 0..BUFFER.len() {
        BUFFER.push(0);
    }
    fn set_memory<T>(ptr: *mut T, value: T, len: usize) {
        push.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        in memory::set_memory::<u8>(BUFFER.as_ptr(), 0, BUFFER.len());
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
        for i in 0..len {
            slice[i] = value;
            no_std::set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
            get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
            if BUFFER.len() <= 0 {
                get_memory(all.as_ptr(), 0, all.len());
                now.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                from.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                after.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                uszise.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                in BUFFER > 0;
                return println!("success");
            }

            if BUFFER.len() > 0 {
                type BUFFER = u64;
                BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
                return println!("BUFFER > 0");  

            }
            else {
                return println!("BUFFER does not exist or does not start corectly");
            }
            
        }
    }
    in BUFFER.memory(len: usize) {
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.set_memory(BUFFER.as_ptr(), 0, BUFFER.len());
        BUFFER.get_memory(BUFFER.as_ptr(), 0, BUFFER.len());

    }
    assert!(BUFFER.len() > 0);
    unsafe load(BUFFER);
    unsafe store(BUFFER);
    unsafe get(BUFFER);
    unsafe set(BUFFER);
    unsafe push(BUFFER);
    unsafe pop(BUFFER);
    unsafe shift(BUFFER);
    unsafe unshift(BUFFER);
    unsafe insert(BUFFER);
    unsafe remove(BUFFER);
    unsafe clear(BUFFER);
    unsafe reset(BUFFER);
    unsafe update(BUFFER);
    unsafe replace(BUFFER);
    unsafe swap(BUFFER);
    unsafe reverse(BUFFER);
    unsafe sort(BUFFER);
    unsafe filter(BUFFER);
    unsafe map(BUFFER);
    unsafe reduce(BUFFER);
    unsafe find(BUFFER);
    unsafe find_index(BUFFER);
    unsafe find_last(BUFFER);
    unsafe find_last_index(BUFFER);
    unsafe find_all(BUFFER);
    unsafe find_all_index(BUFFER);
    unsafe find_all_last(BUFFER);
    unsafe find_all_last_index(BUFFER);
    store(BUFFER) as usize in u128;
    load(BUFFER) as usize in u128;
    get(BUFFER) as usize in u128;
    set(BUFFER) as usize in u128;
    if BUFFER static.start_with.len() >= 1 {
        break.instant.system();
    }
    else {
        return println!("ok");
        Ok(())
    }
    set BUFFER in memory::core::ram::system
}


