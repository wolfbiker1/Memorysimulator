use std::{collections::HashMap, sync::Mutex};
#[macro_use]
extern crate lazy_static;

lazy_static! {
    #[derive(Debug, Copy, Clone)]
    static ref HASHMAP: Mutex<HashMap<u32, u32>> = Mutex::new(HashMap::new());
}

pub fn init(pba_start: u32, pba_end: u32) {
    let mut m = HASHMAP.lock().unwrap();

    for address in (pba_start..pba_end).step_by(0x04) {
        m.insert(address, 0x00000000);
    }
}

pub fn write(dst: u32, content: u32) {
    let mut m = HASHMAP.lock().unwrap();
    m.insert(dst, content);
}

fn get_sorted_hashmap_as_vec() -> Vec<(u32, u32)> {
    let m = HASHMAP.lock().unwrap().clone();
    let mut hash_vec: Vec<(u32, u32)> = m.into_iter().collect::<Vec<(u32, u32)>>();
    hash_vec.sort_by(|a, b| a.0.cmp(&b.0));

    hash_vec
}

pub fn dump_memory() {
    for (address, content) in get_sorted_hashmap_as_vec().into_iter().rev() {
        println!("0x{:08x}: 0x{:08x}", address, content);
    }
}

pub fn read(addr: u32) -> u32 {
    let data = get_sorted_hashmap_as_vec();
    let search_result = data.binary_search_by_key(&addr, |&(a, _b)| a);
    match search_result {
        Ok(index) => return data[index].1 as u32,
        Err(_) => return 0,
    };
}

pub fn show_content(addr: u32) {
    for (address, content) in get_sorted_hashmap_as_vec().into_iter().rev() {
        if addr == address {
            println!("0x{:08x}", content);
            return;
        }
    }
}
