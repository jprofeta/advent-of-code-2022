use std::sync::atomic::{AtomicBool, Ordering};

static DEBUG_PRINT_ENABLE: AtomicBool = AtomicBool::new(false);

// #[macro_export]
// macro_rules! dbgprint_enabled {
//     () => { use dbgprint::is_dbgprint_enabled; dbgprint::is_enabled() }
// }

#[macro_export]
macro_rules! dbgprintln {
    () =>(if dbgprint::is_enabled() { println!() });
    ($($arg:tt)*) => (if dbgprint::is_enabled() { println!($($arg)*) });
}

#[macro_export]
macro_rules! dbgprint {
    () =>(if dbgprint::is_enabled() { print!() });
    ($($arg:tt)*) => (if dbgprint::is_enabled() { print!($($arg)*) });
}

pub fn is_enabled() -> bool {
    DEBUG_PRINT_ENABLE.load(Ordering::Relaxed)
}

pub fn enable() {
    DEBUG_PRINT_ENABLE.store(true, Ordering::Relaxed);
}

pub fn disable() {
    DEBUG_PRINT_ENABLE.store(false, Ordering::Relaxed);
}
