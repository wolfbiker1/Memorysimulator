extern crate mem;
fn main() {
    mem::init(0x2000_0000, 0x2000_0FFF);

    mem::write(0x20000000, 0xABCD0101);
    mem::write(0x20000000, mem::read(0x2000_0000) & !(0xFFFF));
    println(mem::read(0x2000_0000));
}

pub fn println(value: u32) {
    println!("0x{:08x}", value);
}
