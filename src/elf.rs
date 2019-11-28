use std::collections::HashMap;
use memory::Memory;

/// Use the function to load code (`.text` section) into a memory buffer.
pub fn load_instructions(file:&elflib::File, mem:&mut dyn Memory) -> bool {
    file.get_section(".text").map_or(false, | text | -> bool {
        let mut i = 0;

        println!("Allocating words starting from {:x} with size {}", text.shdr.addr, text.data.len());
        mem.allocate_at(text.shdr.addr as usize, text.data.len());

        while i < text.data.len() {
            let x = if file.ehdr.data == elflib::types::ELFDATA2LSB {
                u16::from_le(text.data.get_16(i))
            } else {
                u16::from_be(text.data.get_16(i))
            };

            let addr = (text.shdr.addr as usize) + i;
            mem.set_16(addr, x);

            i += 2
        }

        true
    })
}

/// Use this function to load `.rodata` section into a memory buffer.
pub fn load_rodata(file:&elflib::File, mem:&mut dyn Memory) -> bool {
    file.get_section(".rodata").map_or(false, | section | -> bool {
        let mut rodata_i = section.shdr.addr as usize;

        mem.allocate_at(rodata_i, section.data.len());
        for byte in &section.data {
            mem.set_8(rodata_i, *byte);
            rodata_i += 1
        }

        true
    })
}

pub fn load_section(file:&elflib::File, section:&str, mem:&mut dyn Memory) -> bool {
    file.get_section(section).map_or(false, | section | -> bool {
        let mut rodata_i = section.shdr.addr as usize;

        mem.allocate_at(rodata_i, section.data.len());
        for byte in &section.data {
            mem.set_8(rodata_i, *byte);
            rodata_i += 1
        }

        true
    })
}
/// Use this function to retrieve the mapping of linked functions addresses to
/// their name. Useful when emulating `libc` calls.
pub fn get_plt_symbols(file:&elflib::File) -> Option<HashMap<i32, String>> {
    let symtab = file.get_section(".symtab")?;
    let symbols = file.get_symbols(symtab).ok()?;
    let plt = file.get_section(".plt")?;
    let mut ret = HashMap::new();

    for sym in symbols {
        if sym.value >= plt.shdr.addr && sym.value < plt.shdr.addr + plt.shdr.size {
            ret.insert(sym.value as i32, sym.name);
        }
    }

    Some(ret)
}


/// Use this function to retrieve the address of a binary symbol in the elf.
pub fn get_main_pc(file:&elflib::File) -> Option<i32> {
    let symtab = file.get_section(".symtab")?;
    let symbols = file.get_symbols(symtab).ok()?;

    for sym in symbols {
        if sym.name == "main" {
            return Some(sym.value as i32)
        }
    }

    None
}

/// Use this function to retrieve the address of any symbol in the elf.
/// Return `None` if the symbol is not present.
pub fn get_symbol_address(file:&elflib::File, symbol:&str) -> Option<i32> {
    let symtab = file.get_section(".symtab")?;
    let symbols = file.get_symbols(symtab).ok()?;

    for sym in symbols {
        if sym.name.contains(symbol) {
            return Some(sym.value as i32)
        }
    }

    None
}

/// Use this function to load an entier `.elf` file in memory. Every needed
/// sections are loaded in memory. For now this function doesn't load 
/// dynamic libraries. It will in a future version.
pub fn load_program(file:&elflib::File, mem:&mut dyn Memory) -> bool {
    for section in &file.sections {
        let addr = section.shdr.addr as usize;
        if addr == 0 { continue }

        if !load_section(file, section.shdr.name.as_str(), mem) {
            return false
        }
    }
    false
}
