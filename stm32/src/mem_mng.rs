extern crate mem;
pub const MEM_BLOCK_START: u32 = 0x2000_0FFF;
pub mod mem_mng {
    // use super::mem::*;
    pub const JOURNAL_BASE: u32 = 0x2000_0000;
    pub const JOURNAL_NUM_OF_ELEMENTS: u32 = JOURNAL_BASE;
    pub const JOURNAL_START: u32 = JOURNAL_BASE + 0x04;

    const MEM_BLOCK_START: u32 = 0x2000_0FFF;
    const ADDR_OF_HIGHEST_FREE_BLOCK: u32 = MEM_BLOCK_START + 0x04;

    pub fn init_mem_and_journal(pba_start: u32, pba_end: u32) {
        mem::init(pba_start, pba_end);
        mem::write(JOURNAL_BASE, 0x0000_0000);
        mem::write(ADDR_OF_HIGHEST_FREE_BLOCK, ADDR_OF_HIGHEST_FREE_BLOCK);
        mem::write(MEM_BLOCK_START, ADDR_OF_HIGHEST_FREE_BLOCK);
    }

    pub fn print_dump() {
        mem::dump_memory();
    }

    pub fn malloc(requested_size: u32) -> u32 {
        if mem::read(JOURNAL_NUM_OF_ELEMENTS) != 0 {
            // next fit
            let entries = mem::read(JOURNAL_NUM_OF_ELEMENTS);
            for journal_entry in 0..entries {
                let journal_entry_addr = JOURNAL_START + journal_entry * 0x04;
                let free_entry = mem::read(journal_entry_addr);
                let size_available = mem::read(free_entry);
                if requested_size <= size_available {
                    mem::write(journal_entry_addr, 0x0000_0000);

                    let journal_size = mem::read(JOURNAL_NUM_OF_ELEMENTS);
                    let end_adress = JOURNAL_BASE + journal_size * 0x04;

                    if journal_size == 1 {
                        mem::write(journal_entry_addr, 0x0000_0000);
                    } else {
                        mem::swap(end_adress, journal_entry_addr);
                    }
                    mem::write(
                        JOURNAL_NUM_OF_ELEMENTS,
                        mem::read(JOURNAL_NUM_OF_ELEMENTS) - 1,
                    );
                    mem::write(free_entry, requested_size);
                    return free_entry + 0x4;
                }
            }
            let ahf = mem::read(MEM_BLOCK_START);
            let chunk_start = ahf + 0x4;
            mem::write(ahf, requested_size);
            mem::write(MEM_BLOCK_START, chunk_start + requested_size);
            return chunk_start;
            // 0x666
        } else {
            let ahf = mem::read(MEM_BLOCK_START);
            let chunk_start = ahf + 0x4;
            mem::write(ahf, requested_size);
            mem::write(MEM_BLOCK_START, chunk_start + requested_size);
            return chunk_start;
        }
    }
    pub fn free(addr: u32) {
        mem::write(addr - 0x04, mem::read(addr - 0x4));
        let num_of_journal_entries = mem::read(JOURNAL_NUM_OF_ELEMENTS);

        mem::write(JOURNAL_NUM_OF_ELEMENTS, num_of_journal_entries + 1);
        mem::write(JOURNAL_START + num_of_journal_entries * 0x04, addr - 0x04);
    }
}
