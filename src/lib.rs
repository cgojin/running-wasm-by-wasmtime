#[no_mangle]
pub extern "C" fn print_hello() {
    return println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
