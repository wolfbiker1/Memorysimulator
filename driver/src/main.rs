extern crate mem;
fn main() {
   

    mem::write(0x20000000, 0xABCD0101);
    mem::write(0x20000000, mem::read(0x2000_0000) & !(0xFFFF));
    println(mem::read(0x2000_0000));
}

pub fn println(value: u32) {
    println!("0x{:08x}", value);
}

#[test]
fn test_read_write() {
    mem::init(0x2000_0000, 0x2000_0FFF);
    assert_eq!(0x00000000, mem::read(0x2000_0000));
    mem::write(0x2000_0000, 0xABAB_1111);
    assert_eq!(0xABAB_1111, mem::read(0x2000_0000));
    mem::write(0x2000_000F, 0x1111_ABAB);
    assert_eq!(0x1111_ABAB, mem::read(0x2000_000F));
}


#[test]
fn test_swap() {
    mem::init(0x2000_0000, 0x2000_0FFF);
    mem::write(0x2000_0000, 0xABAB_1111);
    mem::write(0x2000_000F, 0x1111_ABAB);
    
    mem::swap(0x2000_0000, 0x2000_000F);

    assert_eq!(0xABAB_1111, mem::read(0x2000_000F));
    assert_eq!(0x1111_ABAB, mem::read(0x2000_0000));
}