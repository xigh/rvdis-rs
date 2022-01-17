use elf_rs::*;

pub struct RVElf<'a> {
    elf: Elf<'a>,
}

impl<'a> RVElf<'a> {
    pub fn decode_from(elf_buf: &'a [u8]) -> Option<Self> {
        let elf = match Elf::from_bytes(&elf_buf) {
            Ok(elf) => elf,
            Err(_) => return None,
        };

        Some(Self { elf })
    }

    pub fn entry_point(&self) -> u64 {
        self.elf.entry_point()
    }

    pub fn get_section(&self, name: &str) -> Option<(u64, &[u8])> {
        let name = name.as_bytes();
        let section = match self.elf.lookup_section(name) {
            Some(section) => section,
            None => return None,
        };
        Some((section.addr(), section.content()))
    }

    pub fn is_64bits(&self) -> bool {
        let hdr = self.elf.elf_header();
        let bit = hdr.class();
        return match bit {
            ElfClass::Elf32 => false,
            ElfClass::Elf64 => true,
            _ => panic!(),
        };
    }

    pub fn symbol_at(&self, addr: u64) -> Option<String> {
        let symtab = match self.get_section(".symtab") {
            Some((_, section)) => section,
            None => return None,
        };
        let srttab = match self.get_section(".strtab") {
            Some((_, section)) => section,
            None => return None,
        };
        if self.is_64bits() {
            self.symbol64_at(addr, symtab, srttab)
        } else {
            self.symbol32_at(addr, symtab, srttab)
        }
    }

    pub fn symbol32_at(&self, addr: u64, symtab: &[u8], strtab: &[u8]) -> Option<String> {
        let mut symtab = symtab;
        while symtab.len() > 15 {
            let w0 = symtab[0] as u64;
            let w1 = symtab[1] as u64;
            let w2 = symtab[2] as u64;
            let w3 = symtab[3] as u64;
            let wname = w0 | (w1 << 8) | (w2 << 16) | (w3 << 24);
            if wname != 0 {
                let w0 = symtab[4] as u64;
                let w1 = symtab[5] as u64;
                let w2 = symtab[6] as u64;
                let w3 = symtab[7] as u64;
                let wvalue = w0 | (w1 << 8) | (w2 << 16) | (w3 << 24);
                if wvalue == addr {
                    let wname = wname as usize;
                    let wname = crate::cstr::str_from_null_terminated_utf8_safe(&strtab[wname..]);
                    return Some(String::from(wname));
                }
            }
            symtab = &symtab[16..];
        }

        None
    }

    pub fn symbol64_at(&self, addr: u64, symtab: &[u8], strtab: &[u8]) -> Option<String> {
        let mut symtab = symtab;
        while symtab.len() > 23 {
            let w0 = symtab[0] as u64;
            let w1 = symtab[1] as u64;
            let w2 = symtab[2] as u64;
            let w3 = symtab[3] as u64;
            let wname = w0 | (w1 << 8) | (w2 << 16) | (w3 << 24);
            if wname != 0 {
                let w0 = symtab[8 + 0] as u64;
                let w1 = symtab[8 + 1] as u64;
                let w2 = symtab[8 + 2] as u64;
                let w3 = symtab[8 + 3] as u64;
                let w4 = symtab[8 + 4] as u64;
                let w5 = symtab[8 + 5] as u64;
                let w6 = symtab[8 + 6] as u64;
                let w7 = symtab[8 + 7] as u64;
                #[cfg_attr(rustfmt, rustfmt_skip)]
                let wvalue = w0 | (w1 << 8) | (w2 << 16) | (w3 << 24) |
                    (w4 << 32) | (w5 << 40) | (w6 << 48) | (w7 << 56);
                if wvalue == addr {
                    let wname = wname as usize;
                    let wname = crate::cstr::str_from_null_terminated_utf8_safe(&strtab[wname..]);
                    return Some(String::from(wname));
                }
            }
            symtab = &symtab[24..];
        }

        None
    }
}
