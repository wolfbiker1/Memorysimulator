mod mem_mng;
use rand::Rng;

const PBA_START: u32 = 0x2000_0000;
const PBA_END: u32 = 0x2000_FFFF;

use mem_mng::mem_mng::*;
use mem_mng::MEM_BLOCK_START;
fn main() {
    // let m = malloc(4);
    // println!("m: {:08x}", m);
    // free(m);
    // let m = malloc(4);
    // println!("m: {:08x}", m);

    // test_catalog_gaps();
    init_mem_and_journal(PBA_START, PBA_END);
    pub const JOURNAL_BASE: u32 = 0x2000_0000;
    let mut adresses: Vec<u32> = Vec::new();
    for _ in 0..100 {
        let m = malloc(rand::thread_rng().gen_range(1..10));
        adresses.push(m);
    }
    let num_of_frees: u32 = rand::thread_rng().gen_range(1..74);
    for i in 0..num_of_frees {
        free(adresses[i as usize]);
    }
    // free(m);
    let free_entries = mem::read(JOURNAL_BASE);
    for f in (JOURNAL_START..JOURNAL_START + 0x04 * free_entries).step_by(0x04) {
        println!("{:08x}", mem::read(f));
    }
    // print_dump();
    // const JOURNAL_BASE: u32 = 0x2000_0000;
    // const JOURNAL_NUM_OF_ELEMENTS: u32 = JOURNAL_BASE;
    // const JOURNAL_START: u32 = JOURNAL_BASE + 0x04;
    // let mut ahf = 0x20000064;
    // for _ in 0..10 {
    //     let rng = rand::thread_rng().gen_range(1..10);

    //     let m = malloc(4);
    //     free(m);
    //     // assert_eq!(1, mem::read(JOURNAL_NUM_OF_ELEMENTS));
    // }
}

pub fn println(value: u32) {
    println!("0x{:08x}", value);
}

#[test]
fn test_on_malloc_without_free() {
    init_mem_and_journal(PBA_START, PBA_END);
    const ADDR_OF_HIGHEST_FREE_BLOCK: u32 = MEM_BLOCK_START + 0x04;
    let mut ahf = ADDR_OF_HIGHEST_FREE_BLOCK;
    for _ in 0..200 {
        let rng = rand::thread_rng().gen_range(1..1000);
        ahf += rng + 0x04;
        malloc(rng);
        assert_eq!(ahf, mem::read(MEM_BLOCK_START));
    }
}

#[test]
fn test_on_malloc_with_free() {
    init_mem_and_journal(PBA_START, PBA_END);
    const ADDR_OF_HIGHEST_FREE_BLOCK: u32 = MEM_BLOCK_START + 0x04;
    for _ in 0..200 {
        let m = malloc(8);
        free(m);
        let m = malloc(8);
        assert_eq!(ADDR_OF_HIGHEST_FREE_BLOCK + 0x04, m);
        free(m);
    }
}
    // #[test]
    // fn repetition_test_on_malloc_with_free() {
    //     init_mem_and_journal(PBA_START, PBA_END);
    //     const ADDR_OF_HIGHEST_FREE_BLOCK: u32 = MEM_BLOCK_START + 0x04;
    //     for _ in 0..200 {
    //         let r = rand::thread_rng().gen_range(1..1000);
    //         let m = malloc(8);
    //         // println!("m: {:08x}", m);
    //         free(m);
    //         let m = malloc(8);
    //         assert_eq!(ADDR_OF_HIGHEST_FREE_BLOCK + 0x04, m);
    //         free(m);
    //     }
    // }

#[test]
fn test_catalog_size() {
    init_mem_and_journal(PBA_START, PBA_END);
    pub const JOURNAL_BASE: u32 = 0x2000_0000;
    let mut adresses: Vec<u32> = Vec::new();
    for _ in 0..300 {
        let m = malloc(rand::thread_rng().gen_range(1..1000));
        adresses.push(m);
    }
    let num_of_frees: u32 = rand::thread_rng().gen_range(1..174);
    for i in 0..num_of_frees {
        free(adresses[i as usize]);
    }
    assert_eq!(num_of_frees, mem::read(JOURNAL_BASE));
}

#[test]
fn test_catalog_gaps() {
    init_mem_and_journal(PBA_START, PBA_END);
    pub const JOURNAL_BASE: u32 = 0x2000_0000;
    pub const JOURNAL_START: u32 = JOURNAL_BASE + 0x04;

    let mut adresses: Vec<u32> = Vec::new();
    for _ in 0..100 {
        let m = malloc(rand::thread_rng().gen_range(1..10));
        adresses.push(m);
    }
    let num_of_frees: u32 = rand::thread_rng().gen_range(1..74);
    for i in 0..num_of_frees {
        free(adresses[i as usize]);
    }

    let free_entries = mem::read(JOURNAL_BASE);
    for f in (JOURNAL_START..JOURNAL_START + 0x04 * free_entries).step_by(0x04) {
        println!("{:08x}", mem::read(f));
        assert_ne!(0x0000_0000, mem::read(JOURNAL_BASE));
    }
}
