use riscv::{Gpr, Inst};

pub fn dump_inst(text_addr: u64, inst: Inst) {
    match inst {
        Inst::ERROR => return,
        Inst::UNDEF(w) => {
            println!(
                "    {:08x}:    {:12} w={:08x} op=0b{:05b}11 f3=0b{:03b} f7=0x{:02x}",
                text_addr,
                "undef",
                w,
                (w >> 2) & 0b11111,
                (w >> 12) & 0b111,
                (w >> 25) & 0b1111111
            );
            return;
        }
        // RV32I
        Inst::ADD(rd, rs1, rs2) => match rs1 {
            Gpr::zero => println!("    {:08x}:    {:12} {}, {}", text_addr, "li", rd, rs2),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "add", rd, rs1, rs2
            ),
        },
        Inst::SUB(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sub", rd, rs1, rs2
            )
        }
        Inst::XOR(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "xor", rd, rs1, rs2
            )
        }
        Inst::OR(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "or", rd, rs1, rs2
            )
        }
        Inst::AND(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "and", rd, rs1, rs2
            )
        }
        Inst::SLL(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sll", rd, rs1, rs2
            )
        }
        Inst::SRL(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "srl", rd, rs1, rs2
            )
        }
        Inst::SRA(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sra", rd, rs1, rs2
            )
        }
        Inst::SLT(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "slt", rd, rs1, rs2
            )
        }
        Inst::SLTU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sltu", rd, rs1, rs2
            )
        }
        Inst::ADDI(rd, rs1, imm) => match (rs1, imm) {
            (Gpr::zero, _) => println!("    {:08x}:    {:12} {}, {}", text_addr, "li", rd, imm),
            (_, 0) => println!("    {:08x}:    {:12} {}, {}", text_addr, "mv", rd, rs1),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "addi", rd, rs1, imm
            ),
        },
        Inst::XORI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "xori", rd, rs1, imm
            )
        }
        Inst::ORI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "ori", rd, rs1, imm
            )
        }
        Inst::ANDI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "andi", rd, rs1, imm
            )
        }
        Inst::SLLI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "slli", rd, rs1, imm
            )
        }
        Inst::SRLI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "srli", rd, rs1, imm
            )
        }
        Inst::SRAI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "srai", rd, rs1, imm
            )
        }
        Inst::SLTI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "slti", rd, rs1, imm
            )
        }
        Inst::SLTUI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sltui", rd, rs1, imm
            )
        }
        Inst::LB(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "lb", rd, imm, rs1
            )
        }
        Inst::LH(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "lh", rd, imm, rs1
            )
        }
        Inst::LW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "lw", rd, imm, rs1
            )
        }
        Inst::LD(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "ld", rd, imm, rs1
            )
        }
        Inst::LBU(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "lbu", rd, imm, rs1
            )
        }
        Inst::LHU(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "lhu", rd, imm, rs1
            )
        }
        Inst::LWU(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "lwu", rd, imm, rs1
            )
        }
        Inst::SB(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    {:12} {}({}), {}",
                text_addr, "sb", imm, rs2, rs1
            )
        }
        Inst::SH(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    {:12} {}({}), {}",
                text_addr, "sh", imm, rs2, rs1
            )
        }
        Inst::SW(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    {:12} {}({}), {}",
                text_addr, "sw", imm, rs2, rs1
            )
        }
        Inst::SD(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    {:12} {}({}), {}",
                text_addr, "sd", imm, rs2, rs1
            )
        }
        Inst::BEQ(rs1, rs2, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            match rs2 {
                Gpr::zero => println!(
                    "    {:08x}:    {:12} {}, 0x{:x}",
                    text_addr, "beqz", rs1, addr
                ),
                _ => println!(
                    "    {:08x}:    {:12} {}, {}, 0x{:x}",
                    text_addr, "beq", rs1, rs2, addr
                ),
            }
        }
        Inst::BNE(rs1, rs2, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            println!(
                "    {:08x}:    {:12} {}, {}, 0x{:x}",
                text_addr, "bne", rs1, rs2, addr
            )
        }
        Inst::BLT(rs1, rs2, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            println!(
                "    {:08x}:    {:12} {}, {}, 0x{:x}",
                text_addr, "blt", rs1, rs2, addr
            )
        }
        Inst::BGE(rs1, rs2, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            match rs2 {
                Gpr::zero => println!(
                    "    {:08x}:    {:12} {}, 0x{:x}",
                    text_addr, "bgez", rs1, addr
                ),
                _ => println!(
                    "    {:08x}:    {:12} {}, {}, 0x{:x}",
                    text_addr, "bge", rs1, rs2, addr
                ),
            }
        }
        Inst::BLTU(rs1, rs2, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            println!(
                "    {:08x}:    {:12} {}, {}, 0x{:x}",
                text_addr, "bltu", rs1, rs2, addr
            )
        }
        Inst::BGEU(rs1, rs2, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            println!(
                "    {:08x}:    {:12} {}, {}, 0x{:x}",
                text_addr, "bgeu", rs1, rs2, addr
            )
        }
        Inst::JAL(rd, imm) => {
            let addr = (text_addr as i64 + imm as i64) as u64;
            match rd {
                Gpr::zero => println!("    {:08x}:    {:12} 0x{:x}", text_addr, "j", addr),
                _ => println!(
                    "    {:08x}:    {:12} {}, 0x{:x}",
                    text_addr, "jal", rd, addr
                ),
            }
        }
        Inst::JALR(rd, rs1, imm) => match (rd, rs1, imm) {
            (Gpr::zero, Gpr::ra, 0) => {
                println!("    {:08x}:    {:12}", text_addr, "ret")
            }
            _ => println!(
                "    {:08x}:    {:12} {}, {}({})",
                text_addr, "jalr", rd, imm, rs1
            ),
        },
        Inst::LUI(rd, imm) => {
            println!("    {:08x}:    {:12} {}, 0x{:x}", text_addr, "lui", rd, imm)
        }
        Inst::AUIPC(rd, imm) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "auipc", rd, imm)
        }
        // CSR
        Inst::CSRRW(rd, rs1, csr) => match rd {
            Gpr::zero => println!("    {:08x}:    {:12} {}, {}", text_addr, "csrw", csr, rs1),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "csrrw", rd, csr, rs1
            ),
        },
        Inst::CSRRS(rd, rs1, csr) => match (rd, rs1) {
            (Gpr::zero, _) => println!("    {:08x}:    {:12} {}, {}", text_addr, "csrs", csr, rs1),
            (_, Gpr::zero) => println!("    {:08x}:    {:12} {}, {}", text_addr, "csrr", rd, csr),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "csrrs", rd, rs1, csr
            ),
        },
        Inst::CSRRC(rd, rs1, csr) => match rd {
            Gpr::zero => println!("    {:08x}:    {:12} {}, {}", text_addr, "csrc", csr, rs1),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "csrrc", rd, csr, rs1
            ),
        },
        Inst::CSRRWI(rd, uimm, csr) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "csrrwi", rd, uimm, csr
            )
        }
        Inst::CSRRSI(rd, uimm, csr) => match rd {
            Gpr::zero => println!("    {:08x}:    {:12} {}, {}", text_addr, "csrsi", csr, uimm),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "csrrsi", rd, uimm, csr
            ),
        },
        Inst::CSRRCI(rd, uimm, csr) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "csrrci", rd, csr, uimm
            )
        }
        // SYS
        Inst::FENCE(rd, rs1, imm) => match (rd, rs1, imm) {
            (Gpr::zero, Gpr::zero, _) => println!("    {:08x}:    {:12}", text_addr, "fence"),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "fence", rd, rs1, imm
            ),
        },
        Inst::FENCEI(rd, rs1, imm) => match (rd, rs1, imm) {
            (Gpr::zero, Gpr::zero, 0) => println!("    {:08x}:    {:12}", text_addr, "fencei"),
            _ => println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "fencei", rd, rs1, imm
            ),
        },
        Inst::ECALL => println!("    {:08x}:    {:12}", text_addr, "ecall"),
        Inst::EBREAK => println!("    {:08x}:    {:12}", text_addr, "ebreak"),
        // Priviledged instructions
        Inst::SRET => println!("    {:08x}:    {:12}", text_addr, "sret"),
        Inst::MRET => println!("    {:08x}:    {:12}", text_addr, "mret"),
        Inst::WFI => println!("    {:08x}:    {:12}", text_addr, "wfi"),
        // Atomic Extension
        Inst::AMOSWAPW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoswap.w", rd, rs2, rs1
            )
        }
        Inst::AMOADDW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoadd.w", rd, rs2, rs1
            )
        }
        Inst::AMOXORW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoxor.w", rd, rs2, rs1
            )
        }
        Inst::AMOANDW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoand.w", rd, rs2, rs1
            )
        }
        Inst::AMOORW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoor.w", rd, rs2, rs1
            )
        }
        Inst::AMOMINW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amomin.w", rd, rs2, rs1
            )
        }
        Inst::AMOMAXW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amomax.w", rd, rs2, rs1
            )
        }
        Inst::AMOMINUW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amominu.w", rd, rs2, rs1
            )
        }
        Inst::AMOMAXUW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amomaxu.w", rd, rs2, rs1
            )
        }
        // Atomic Extension (64 bits)
        Inst::AMOSWAPD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoswap.d", rd, rs2, rs1
            )
        }
        Inst::AMOADDD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoadd.d", rd, rs2, rs1
            )
        }
        Inst::AMOXORD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoxor.d", rd, rs2, rs1
            )
        }
        Inst::AMOANDD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoand.d", rd, rs2, rs1
            )
        }
        Inst::AMOORD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amoor.d", rd, rs2, rs1
            )
        }
        Inst::AMOMIND(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amomin.d", rd, rs2, rs1
            )
        }
        Inst::AMOMAXD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amomax.d", rd, rs2, rs1
            )
        }
        Inst::AMOMINUD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amominu.d", rd, rs2, rs1
            )
        }
        Inst::AMOMAXUD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, ({})",
                text_addr, "amomaxu.d", rd, rs2, rs1
            )
        }
        // RV64I extensions
        Inst::ADDIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "addiw", rd, rs1, imm
            );
        }
        Inst::SLLIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "slliw", rd, rs1, imm
            );
        }
        Inst::SRLIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "srliw", rd, rs1, imm
            );
        }
        Inst::SRAIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sraiw", rd, rs1, imm
            );
        }
        Inst::ADDW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "addw", rd, rs1, rs2
            );
        }
        Inst::SUBW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "subw", rd, rs1, rs2
            );
        }
        Inst::SLLW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sllw", rd, rs1, rs2
            );
        }
        Inst::SRLW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "srlw", rd, rs1, rs2
            );
        }
        Inst::SRAW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sraw", rd, rs1, rs2
            );
        }
        // mul extensions
        Inst::MUL(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "mul", rd, rs1, rs2
            );
        }
        Inst::MULH(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "mulh", rd, rs1, rs2
            );
        }
        Inst::MULSU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "mulsu", rd, rs1, rs2
            );
        }
        Inst::MULU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "mulu", rd, rs1, rs2
            );
        }
        // mul extensions: 64 bits
        Inst::MULW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "mul", rd, rs1, rs2
            );
        }
        // divide/remaining extensions
        Inst::DIV(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "div", rd, rs1, rs2
            );
        }
        Inst::DIVU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "divu", rd, rs1, rs2
            );
        }
        Inst::REM(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "rem", rd, rs1, rs2
            );
        }
        Inst::REMU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "remu", rd, rs1, rs2
            );
        }
        // divide/remaining extensions: 64 bits
        Inst::DIVW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "divw", rd, rs1, rs2
            );
        }
        Inst::DIVUW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "divuw", rd, rs1, rs2
            );
        }
        Inst::REMW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "remw", rd, rs1, rs2
            );
        }
        Inst::REMUW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "remuw", rd, rs1, rs2
            );
        }
        // Load eXclusive / Store Conditional Extension
        Inst::LRW(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "lr.w", rd, rs1);
        }
        Inst::SCW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sc.w", rd, rs1, rs2
            );
        }
        // Load eXclusive / Store Conditional Extension (64 bits)
        Inst::LRD(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "lr.d", rd, rs1);
        }
        Inst::SCD(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}, {}",
                text_addr, "sc.d", rd, rs1, rs2
            );
        }
        // Supervisor Memory-Management Instructions
        Inst::SFENCEWINVAL => {
            println!("    {:08x}:    {:12}", text_addr, "sfence.w.inval");
        }
        Inst::SFENCEINVALIR => {
            println!("    {:08x}:    {:12}", text_addr, "sfence.inval.ir");
        }
        Inst::SFENCEVMA(rs1, rs2) => match (rs1, rs2) {
            (Gpr::zero, Gpr::zero) => println!("    {:08x}:    {:12}", text_addr, "sfence.vma"),
            _ => println!(
                "    {:08x}:    {:12} {}, {}",
                text_addr, "sfence.vma", rs1, rs2
            ),
        },
        Inst::SINVALVMA(rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}",
                text_addr, "sinval.vma", rs1, rs2
            );
        }
        // Hypervisor Memory-Management Instructions
        Inst::HFENCEVVMA(rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}",
                text_addr, "hfence.vvma", rs1, rs2
            );
        }
        Inst::HFENCEGVMA(rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}",
                text_addr, "hfence.gvma", rs1, rs2
            );
        }
        Inst::HINVALVVMA(rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}",
                text_addr, "hinval.vvma", rs1, rs2
            );
        }
        Inst::HINVALGVMA(rs1, rs2) => {
            println!(
                "    {:08x}:    {:12} {}, {}",
                text_addr, "hinval.gvma", rs1, rs2
            );
        }
        // Hypervisor Virtual-Machine Load and Store Instructions
        Inst::HLVB(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.b", rd, rs1);
        }
        Inst::HLVBU(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.bu", rd, rs1);
        }
        Inst::HLVH(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.h", rd, rs1);
        }
        Inst::HLVHU(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.hu", rd, rs1);
        }
        Inst::HLVXHU(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlvx.hu", rd, rs1);
        }
        Inst::HLVW(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.w", rd, rs1);
        }
        Inst::HLVXWU(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlvx.wu", rd, rs1);
        }
        Inst::HSVB(rs1, rs2) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hsv.b", rs1, rs2);
        }
        Inst::HSVH(rs1, rs2) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hsv.h", rs1, rs2);
        }
        Inst::HSVW(rs1, rs2) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hsv.w", rs1, rs2);
        }
        // Hypervisor Virtual-Machine Load and Store Instructions, RV64 only
        Inst::HLVWU(rs1, rs2) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.wu", rs1, rs2);
        }
        Inst::HLVD(rd, rs1) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hlv.d", rd, rs1);
        }
        Inst::HSVD(rs1, rs2) => {
            println!("    {:08x}:    {:12} {}, {}", text_addr, "hsv.d", rs1, rs2);
        }

        Inst::CUNDEF => println!("    {:08x}:    {:12}", text_addr, "c.undef"),
        Inst::CILLEGAL => println!("    {:08x}:    {:12}", text_addr, "c.illegal"),
        Inst::CADDI4SPN => println!("    {:08x}:    {:12}", text_addr, "c.addi4spn"),
        Inst::CFLD => println!("    {:08x}:    {:12}", text_addr, "c.fld"),
        Inst::CLQ => println!("    {:08x}:    {:12}", text_addr, "c.lq"),
        Inst::CLW => println!("    {:08x}:    {:12}", text_addr, "c.lw"),
        Inst::CFLW => println!("    {:08x}:    {:12}", text_addr, "c.flw"),
        Inst::CLD => println!("    {:08x}:    {:12}", text_addr, "c.ld"),
        Inst::CFSD => println!("    {:08x}:    {:12}", text_addr, "c.fsd"),
        Inst::CSQ => println!("    {:08x}:    {:12}", text_addr, "c.sq"),
        Inst::CSW => println!("    {:08x}:    {:12}", text_addr, "c.sw"),
        Inst::CFSW => println!("    {:08x}:    {:12}", text_addr, "c.fsw"),
        Inst::CSD => println!("    {:08x}:    {:12}", text_addr, "c.sd"),
        Inst::CNOP => println!("    {:08x}:    {:12}", text_addr, "c.nop"),
        Inst::CADDI => println!("    {:08x}:    {:12}", text_addr, "c.addi"),
        Inst::CJAL => println!("    {:08x}:    {:12}", text_addr, "c.jal"),
        Inst::CADDIW => println!("    {:08x}:    {:12}", text_addr, "c.addiw"),
        Inst::CLI => println!("    {:08x}:    {:12}", text_addr, "c.li"),
        Inst::CADDI16SP => println!("    {:08x}:    {:12}", text_addr, "c.addi16sp"),
        Inst::CLUI => println!("    {:08x}:    {:12}", text_addr, "c.lui"),
        Inst::CSRLI => println!("    {:08x}:    {:12}", text_addr, "c.srli"),
        Inst::CSRLI64 => println!("    {:08x}:    {:12}", text_addr, "c.srli64"),
        Inst::CSRAI => println!("    {:08x}:    {:12}", text_addr, "c.srai"),
        Inst::CSRAI64 => println!("    {:08x}:    {:12}", text_addr, "c.srai64"),
        Inst::CANDI => println!("    {:08x}:    {:12}", text_addr, "c.andi"),
        Inst::CSUB => println!("    {:08x}:    {:12}", text_addr, "c.sub"),
        Inst::CXOR => println!("    {:08x}:    {:12}", text_addr, "c.xor"),
        Inst::COR => println!("    {:08x}:    {:12}", text_addr, "c.or"),
        Inst::CAND => println!("    {:08x}:    {:12}", text_addr, "c.and"),
        Inst::CSUBW => println!("    {:08x}:    {:12}", text_addr, "c.subw"),
        Inst::CADDW => println!("    {:08x}:    {:12}", text_addr, "c.addw"),
        Inst::CJ => println!("    {:08x}:    {:12}", text_addr, "c.j"),
        Inst::CBEQZ => println!("    {:08x}:    {:12}", text_addr, "c.beqz"),
        Inst::CBNEZ => println!("    {:08x}:    {:12}", text_addr, "c.bnez"),
        Inst::CSLLI => println!("    {:08x}:    {:12}", text_addr, "c.slli"),
        Inst::CSLLI64 => println!("    {:08x}:    {:12}", text_addr, "c.slli64"),
        Inst::CFLDSP => println!("    {:08x}:    {:12}", text_addr, "c.fldsp"),
        Inst::CLQSP => println!("    {:08x}:    {:12}", text_addr, "c.lqsp"),
        Inst::CLWSP => println!("    {:08x}:    {:12}", text_addr, "c.lwsp"),
        Inst::CFLWSP => println!("    {:08x}:    {:12}", text_addr, "c.flwsp"),
        Inst::CLDSP => println!("    {:08x}:    {:12}", text_addr, "c.ldsp"),
        Inst::CJR => println!("    {:08x}:    {:12}", text_addr, "c.jr"),
        Inst::CMV => println!("    {:08x}:    {:12}", text_addr, "c.mv"),
        Inst::CEBREAK => println!("    {:08x}:    {:12}", text_addr, "c.ebreak"),
        Inst::CJALR => println!("    {:08x}:    {:12}", text_addr, "c.jalr"),
        Inst::CADD => println!("    {:08x}:    {:12}", text_addr, "c.add"),
        Inst::CFSDSP => println!("    {:08x}:    {:12}", text_addr, "c.fsdsp"),
        Inst::CSQSP => println!("    {:08x}:    {:12}", text_addr, "c.sqsp"),
        Inst::CSWSP => println!("    {:08x}:    {:12}", text_addr, "c.swsp"),
        Inst::CFSWSP => println!("    {:08x}:    {:12}", text_addr, "c.fswsp"),
        Inst::CSDSP => println!("    {:08x}:    {:12}", text_addr, "c.sdsp"),
        // no _ ... I want all instruction to be implemented !!!
    }
}
