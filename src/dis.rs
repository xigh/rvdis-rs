use riscv::{Inst, Gpr};

pub fn dump_inst(text_addr: u64, inst: Inst) {
    match inst {
        Inst::ERROR => return,
        Inst::UNDEF(w) => {
            println!(
                "    {:08x}:    undef  w={:08x} op=0b{:05b}11 f3=0b{:03b} f7=0x{:02x}",
                text_addr,
                w,
                (w >> 2) & 0b11111,
                (w >> 12) & 0b111,
                (w >> 25) & 0b1111111
            );
            return;
        }
        // RV32I
        Inst::ADD(rd, rs1, rs2) => match rs1 {
            Gpr::zero => println!(
                "    {:08x}:    li     {}, {}",
                text_addr,
                rd,
                rs2
            ),
            _ => println!(
                "    {:08x}:    add    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            ),
        },
        Inst::SUB(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    sub    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::XOR(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    xor    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::OR(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    or     {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::AND(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    and    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::SLL(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    sll    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::SRL(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    srl    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::SRA(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    sra    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::SLT(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    slt    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::SLTU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    sltu   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            )
        }
        Inst::ADDI(rd, rs1, imm) => match rs1 {
            Gpr::zero => println!(
                "    {:08x}:    li     {}, {}",
                text_addr,
                rd,
                imm
            ),
            _ => println!(
                "    {:08x}:    addi   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            ),
        },
        Inst::XORI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    xori   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::ORI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    ori    {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::ANDI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    andi   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::SLLI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    slli   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::SRLI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    srli   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::SRAI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    srai   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::SLTI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    slti   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::SLTUI(rd, rs1, imm) => {
            println!(
                "    {:08x}:    sltui  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            )
        }
        Inst::LB(rd, rs1, imm) => {
            println!(
                "    {:08x}:    lb     {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::LH(rd, rs1, imm) => {
            println!(
                "    {:08x}:    lh     {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::LW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    lw     {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::LD(rd, rs1, imm) => {
            println!(
                "    {:08x}:    ld     {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::LBU(rd, rs1, imm) => {
            println!(
                "    {:08x}:    lbu    {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::LHU(rd, rs1, imm) => {
            println!(
                "    {:08x}:    lhu    {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::LWU(rd, rs1, imm) => {
            println!(
                "    {:08x}:    lwu    {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            )
        }
        Inst::SB(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    sb     {}({}), {}",
                text_addr,
                imm,
                rs2,
                rs1
            )
        }
        Inst::SH(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    sh     {}({}), {}",
                text_addr,
                imm,
                rs2,
                rs1
            )
        }
        Inst::SW(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    sw     {}({}), {}",
                text_addr,
                imm,
                rs2,
                rs1
            )
        }
        Inst::SD(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    sd     {}({}), {}",
                text_addr,
                imm,
                rs2,
                rs1
            )
        }
        Inst::BEQ(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    beq    {}, {}, {}",
                text_addr,
                rs1,
                rs2,
                imm
            )
        }
        Inst::BNE(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    bne    {}, {}, {}",
                text_addr,
                rs1,
                rs2,
                imm
            )
        }
        Inst::BLT(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    blt    {}, {}, {}",
                text_addr,
                rs1,
                rs2,
                imm
            )
        }
        Inst::BGE(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    bge    {}, {}, {}",
                text_addr,
                rs1,
                rs2,
                imm
            )
        }
        Inst::BLTU(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    bltu   {}, {}, {}",
                text_addr,
                rs1,
                rs2,
                imm
            )
        }
        Inst::BGEU(rs1, rs2, imm) => {
            println!(
                "    {:08x}:    bgeu   {}, {}, {}",
                text_addr,
                rs1,
                rs2,
                imm
            )
        }
        Inst::JAL(rd, imm) => match rd {
            Gpr::zero => println!(
                "    {:08x}:    j      {}",
                text_addr,
                imm
            ),
            _ => println!(
                "    {:08x}:    jal    {}, {}",
                text_addr,
                rd,
                imm
            ),
        },
        Inst::JALR(rd, rs1, imm) => match (rd, rs1, imm) {
            (Gpr::zero, Gpr::ra, 0) => {
                println!("    {:08x}:    ret", text_addr)
            }
            _ => println!(
                "    {:08x}:    jalr   {}, {}({})",
                text_addr,
                rd,
                imm,
                rs1
            ),
        },
        Inst::LUI(rd, imm) => println!(
            "    {:08x}:    lui    {}, 0x{:x}",
            text_addr,
            rd,
            imm
        ),
        Inst::AUIPC(rd, imm) => {
            println!(
                "    {:08x}:    auipc  {}, {}",
                text_addr,
                rd,
                imm
            )
        }
        // CSR
        Inst::CSRRW(rd, rs1, csr) => println!("    {:08x}:    csrrw  {}, {}, {}",
            text_addr, rd, rs1, csr),
        Inst::CSRRS(rd, rs1, csr) => println!("    {:08x}:    csrrs  {}, {}, {}",
            text_addr, rd, rs1, csr),
        Inst::CSRRC(rd, rs1, csr) => println!("    {:08x}:    csrrc  {}, {}, {}",
            text_addr, rd, rs1, csr),
        Inst::CSRRWI(rd, uimm, csr) => println!("    {:08x}:    csrrwi {}, {}, {}",
            text_addr, rd, uimm, csr),
        Inst::CSRRSI(rd, uimm, csr) => println!("    {:08x}:    csrrsi {}, {}, {}",
            text_addr, rd, uimm, csr),
        Inst::CSRRCI(rd, uimm, csr) => println!("    {:08x}:    csrrci {}, {}, {}",
            text_addr, rd, uimm, csr),
        // SYS
        Inst::FENCEI(rd, rs1, imm) => println!("    {:08x}:    fencei {}, {}, {}",
            text_addr, rd, rs1, imm),
        Inst::ECALL => println!("    {:08x}:    ecall", text_addr),
        Inst::EBREAK => println!("    {:08x}:    ebreak", text_addr),
        // Priviledged instructions
        Inst::SRET => println!("    {:08x}:    sret", text_addr),
        Inst::MRET => println!("    {:08x}:    mret", text_addr),
        Inst::WFI => println!("    {:08x}:    wfi", text_addr),
        // RV64I extensions
        Inst::ADDIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    addiw  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            );
        }
        Inst::SLLIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    slliw  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            );
        }
        Inst::SRLIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    srliw  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            );
        }
        Inst::SRAIW(rd, rs1, imm) => {
            println!(
                "    {:08x}:    sraiw  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                imm
            );
        }
        Inst::ADDW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    addw   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::SUBW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    subw   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::SLLW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    sllw   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::SRLW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    srlw   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::SRAW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    sraw   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        // mul extensions
        Inst::MUL(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    mul   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::MULH(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    mulh  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::MULSU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    mulsu {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::MULU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    mulu  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        // mul extensions: 64 bits
        Inst::MULW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    mul   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        // divide/remaining extensions
        Inst::DIV(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    div   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::DIVU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    divu  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::REM(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    rem   {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::REMU(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    remu  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        // divide/remaining extensions: 64 bits
        Inst::DIVW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    divw  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::DIVUW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    divuw {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::REMW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    remw  {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        Inst::REMUW(rd, rs1, rs2) => {
            println!(
                "    {:08x}:    remuw {}, {}, {}",
                text_addr,
                rd,
                rs1,
                rs2
            );
        }
        // _ => {
        //     println!("    {:08x}:    {:?}", text_addr, inst);
        //     unimplemented!()
        // }
    }
}
