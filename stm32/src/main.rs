mod mem_mng;


use mem_mng::mem_mng::*;
fn main() {
    init_mem_and_journal();
    // let a = malloc(8);
    for i in 0..10 {
        // let b = malloc(3);
        // free(b);
        // malloc(2);
    }
    let b = malloc(6);
    free(b);
    malloc(2);
    malloc(2);
    malloc(2);

    // free(a);
    // malloc(12);
    print_dump();
    // init_mem_and_journal();
    // let a1 = malloc(8);
    // let a2 = malloc(16);
    // let a3 = malloc(8);
    // free(a1);
    // free(a3);
    // let a4 = malloc(8);
    // let a1 = malloc(8);
    // free(a1);
    // let a3 = malloc(8);
    // print_dump();
    // println(a1);
    // println(a3);

    // let b1 = malloc(8);
    // malloc(11);
}

pub fn println(value: u32) {
    println!("0x{:08x}", value);
}

#[test]
fn test() {
    init_mem_and_journal();
    assert_eq!(0x20000068, malloc(8));
    assert_eq!(0x20000074, malloc(8));
    assert_eq!(0x20000080, malloc(11));
    assert_eq!(0x2000008F, malloc(2));
}

#[test]
fn test1() {
    init_mem_and_journal();
    assert_eq!(0x20000068, malloc(8));
    free(0x20000068);
    assert_eq!(0x20000068, malloc(8));
}