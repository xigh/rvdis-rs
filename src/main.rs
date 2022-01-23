use std::env;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

mod cstr;

mod elf;
use elf::RVElf;

mod dis;
use dis::dump_inst;

use riscv::Inst;

fn read_elf(elf_buf: &[u8]) {
    let elf = match RVElf::decode_from(elf_buf.as_ref()) {
        Some(elf) => elf,
        None => {
            eprintln!("could not decode file");
            return;
        }
    };

    let bits = if elf.is_64bits() { 64 } else { 32 };

    let (text_addr, text_data) = match elf.get_section(".text") {
        Some((addr, data)) => (addr, data),
        None => return,
    };

    let mut text_bytes = riscv::ByteSlice::from(text_data);
    let mut text_addr = text_addr;

    loop {
        let (inst, len) = riscv::decode(&mut text_bytes, bits);
        match inst {
            Inst::ERROR => break,
            _ => (),
        }

        match elf.symbol_at(text_addr) {
            Some(label) => println!("{}:", label),
            None => (),
        }

        if text_addr == elf.entry_point() {
            println!("// entry-point");
        }

        dump_inst(text_addr, inst);

        text_addr += len as u64;
    }
}

fn read_bin(buf: &[u8], org: u64, bits: u8) {
    let mut text_addr = org;
    let mut text_bytes = riscv::ByteSlice::from(buf);

    loop {
        let (inst, len) = riscv::decode(&mut text_bytes, bits);
        match inst {
            Inst::ERROR => break,
            _ => (),
        }
        
        dump_inst(text_addr, inst);

        text_addr += len as u64;
    }
}

fn main() {
    let mut org = 0u64;
    let mut bin = false;
    let mut bits = 32u8;
    let mut prev = String::new();

    let mut files: Vec<String> = vec!();

    for arg in env::args().skip(1) {
        match prev.as_str() {
            "" => (),
            "--org" => org = match u64::from_str_radix(arg.as_str(), 16) {
                Ok(org) => org,
                Err(err) => {
                    eprintln!("could not parse --org parameter: {}", err);
                    return;
                },
            },
            _ => {
                prev = String::new();
                continue;
            },
        }
        
        match arg.as_str() {
            "-b" | "--binary" => bin = true,
            "-32" => { bin = true; bits = 32 },
            "-64" => { bin = true; bits = 64 },
            "-128" => { bin = true; bits = 128 },
            "--org" => prev = arg, 
            _ => files.push(arg),
        }
    }

    for arg in files {
        let filename = &arg;

        let mut elf_file = match File::open(filename) {
            Ok(elf_file) => elf_file,
            Err(_) => {
                eprintln!("{}: could not open file", filename);
                return;
            }
        };
    
        let mut elf_buf = Vec::<u8>::new();
        if let Err(_) = elf_file.read_to_end(&mut elf_buf) {
            eprintln!("{}: could not read file", filename);
            return;
        }
    
        if bin {
            read_bin(&elf_buf, org, bits);
        } else {
            read_elf(&elf_buf);
        }
    }
}
