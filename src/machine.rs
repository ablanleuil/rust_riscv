use types::MachineInteger;
use isa::{Instruction, OpCode, CsrId, CsrField};
use memory::Memory;

pub trait RiscvIMachine {
    type IntegerType : MachineInteger;

    fn cycle(&mut self);
    fn get_i_register(&self, id:usize) -> Self::IntegerType;
    fn set_i_register(&mut self, id:usize, value:Self::IntegerType);

    fn get_pc(&self) -> Self::IntegerType;
    fn set_pc(&mut self, value:Self::IntegerType);

    fn get_csr_field(&self, id:CsrField) -> Self::IntegerType;
    fn set_csr_field(&mut self, id:CsrField, value:Self::IntegerType);

    // TODO finish implementation
    fn get_csr(&self, id:CsrId) -> Self::IntegerType {
        match id {
            CsrId::MISA =>
                (self.get_csr_field(CsrField::MXL) << (Self::IntegerType::XLEN - 2)) |
                 self.get_csr_field(CsrField::Extensions),
            CsrId::MARCHID => self.get_csr_field(CsrField::ArchitectureID),
            CsrId::MVENDORID => {
                (self.get_csr_field(CsrField::Bank) << 7) |
                 self.get_csr_field(CsrField::Offset)
            },
            CsrId::MIMPID => self.get_csr_field(CsrField::Implementation),
            CsrId::MHARTID => self.get_csr_field(CsrField::HartID),
            CsrId::MSTATUS => {
                let base =
                    (self.get_csr_field(CsrField::TSR) << 22) |
                    (self.get_csr_field(CsrField::TW) << 21) |
                    (self.get_csr_field(CsrField::TVM) << 20) |
                    (self.get_csr_field(CsrField::MPRV) << 17) |
                    (self.get_csr_field(CsrField::MPP) << 11) |
                    (self.get_csr_field(CsrField::MPIE) << 7) |
                    (self.get_csr_field(CsrField::MIE) << 3) |
                    self.get_csr(CsrId::SSTATUS);
                if Self::IntegerType::XLEN > 32 {
                    (self.get_csr_field(CsrField::SXL) << 34) |
                    (self.get_csr_field(CsrField::UXL) << 32) |
                    base
                } else {
                    base
                }
            },
            CsrId::MTVEC => {
                (self.get_csr_field(CsrField::MTVecBASE) << 2) |
                self.get_csr_field(CsrField::MTVecMODE)
            },
            CsrId::MEDELEG => self.get_csr_field(CsrField::SynchronousExceptions),
            CsrId::MIDELEG => self.get_csr_field(CsrField::Interrupts),
            CsrId::MIP => {
                (self.get_csr_field(CsrField::MEIP) << 11) |
                (self.get_csr_field(CsrField::MTIP) << 7) |
                (self.get_csr_field(CsrField::MSIP) << 3) |
                 self.get_csr(CsrId::SIP)
            },
            CsrId::MIE => {
                (self.get_csr_field(CsrField::MEIE) << 11) |
                (self.get_csr_field(CsrField::MTIE) << 7) |
                (self.get_csr_field(CsrField::MSIE) << 3) |
                 self.get_csr(CsrId::SIE)
            },
            CsrId::MCYCLE => self.get_csr_field(CsrField::MCYCLE),
            CsrId::MCYCLEH => {
                if Self::IntegerType::XLEN == 32 {
                    self.get_csr_field(CsrField::MCYCLEH) >> 32
                } else { Self::IntegerType::from(0) }
            },
            CsrId::MINSTRET => self.get_csr_field(CsrField::MINSTRET),
            CsrId::MINSTRETH => {
                if Self::IntegerType::XLEN == 32 {
                    self.get_csr_field(CsrField::MINSTRETH) >> 32
                } else { Self::IntegerType::from(0) }
            },
            CsrId::MCOUNTEREN => {
                (self.get_csr_field(CsrField::MHPMEN) << 3) |
                (self.get_csr_field(CsrField::MIREN) << 2) |
                (self.get_csr_field(CsrField::MTMEN) << 1) |
                 self.get_csr_field(CsrField::MCYEN)
            },
            CsrId::MCOUNTINHIBIT => {
                (self.get_csr_field(CsrField::MHPMIN) << 3) |
                (self.get_csr_field(CsrField::MIRIN) << 2) |
                (self.get_csr_field(CsrField::MTMIN) << 1) |
                 self.get_csr_field(CsrField::MCYIN)
            },
            CsrId::MSCRATCH => self.get_csr_field(CsrField::MSCRATCH),
            CsrId::MEPC => self.get_csr_field(CsrField::MEPC),
            CsrId::MCAUSE => {
                (self.get_csr_field(CsrField::MCauseInterrupt)
                    << (Self::IntegerType::XLEN - 1)) |
                 self.get_csr_field(CsrField::MCauseCode)
            },
            CsrId::MTVAL => self.get_csr_field(CsrField::MTVAL),
            CsrId::SSTATUS => {
                let base =
                    (self.get_csr_field(CsrField::SD) <<
                        (Self::IntegerType::XLEN - 1)) |
                    (self.get_csr_field(CsrField::MXR ) << 19) |
                    (self.get_csr_field(CsrField::SUM ) << 18) |
                    (self.get_csr_field(CsrField::XS) << 15) |
                    (self.get_csr_field(CsrField::FS) << 13) |
                    (self.get_csr_field(CsrField::SPP) << 8) |
                    (self.get_csr_field(CsrField::SPIE) << 5) |
                    (self.get_csr_field(CsrField::UPIE) << 4) |
                    (self.get_csr_field(CsrField::SIE) << 1) |
                    (self.get_csr_field(CsrField::UIE ) << 0);
                if Self::IntegerType::XLEN > 32 {
                    (self.get_csr_field(CsrField::UXL) << 32) | base
                } else {
                    base
                }
            },
            CsrId::STVEC => {
                (self.get_csr_field(CsrField::STVecBASE) << 2) |
                self.get_csr_field(CsrField::STVecMODE)
            },
            CsrId::SIP => {
                (self.get_csr_field(CsrField::SEIP) << 9) |
                (self.get_csr_field(CsrField::UEIP) << 8) |
                (self.get_csr_field(CsrField::STIP) << 5) |
                (self.get_csr_field(CsrField::UTIP) << 4) |
                (self.get_csr_field(CsrField::SSIP) << 1) |
                 self.get_csr_field(CsrField::USIP)
            },
            CsrId::SIE => {
                (self.get_csr_field(CsrField::SEIE) << 9) |
                (self.get_csr_field(CsrField::UEIE) << 8) |
                (self.get_csr_field(CsrField::STIE) << 5) |
                (self.get_csr_field(CsrField::UTIE) << 4) |
                (self.get_csr_field(CsrField::SSIE) << 1) |
                 self.get_csr_field(CsrField::USIE)
            },
            CsrId::SCOUNTEREN => {
                (self.get_csr_field(CsrField::SHPMEN) << 3) |
                (self.get_csr_field(CsrField::SIREN) << 2) |
                (self.get_csr_field(CsrField::STMEN) << 1) |
                 self.get_csr_field(CsrField::SCYEN)
            },
            CsrId::SSCRATCH => self.get_csr_field(CsrField::SSCRATCH),
            CsrId::SEPC => self.get_csr_field(CsrField::SEPC),
            CsrId::SCAUSE => {
                (self.get_csr_field(CsrField::SCauseInterrupt)
                    << (Self::IntegerType::XLEN - 1)) |
                 self.get_csr_field(CsrField::SCauseCode)
            },
            CsrId::STVAL => self.get_csr_field(CsrField::STVAL),
            CsrId::SATP => {
                (self.get_csr_field(CsrField::MODE) << 31) |
                (self.get_csr_field(CsrField::ASID) << 22) |
                 self.get_csr_field(CsrField::PPN)
            },
            _ => Self::IntegerType::from(0),
        }
    }
}

/// Represent the data which we need to send to the [write back] step
struct WriteBackData {
    pub perform: bool,
    pub rd: usize,
    pub value: i32,
}

enum WordSize {
    B = 1,
    H = 2,
    W = 4,
    D = 8,
}

impl From<u8> for WordSize {
    /// Helper to create a WordSize out of an u8
    fn from(s:u8) -> WordSize {
        match s {
            1 => WordSize::B,
            2 => WordSize::H,
            4 => WordSize::W,
            _ => WordSize::D,
        }
    }
}

enum MemAction {
    Load,
    Store,
}

/// Represent the data which we need to send to the [mem] step
/// It also contains information to forward to the next step ([write back])
struct MemData {
    pub pc: i32,

    /// data forwarding from ex stage
    pub wb_perform: bool,
    pub wb_rd: usize,

    /// used as union for either WB value or Store value
    pub value: i32,

    /// data needed to perform the load/store
    pub perform: Option<MemAction>,
    pub addr: usize,
    pub size: WordSize,
}

#[derive(Copy, Clone)]
struct PipelineState {
    pub pc: i32,
    pub instruction: Instruction,
}

impl PipelineState {
    fn empty() -> PipelineState {
        PipelineState { pc: 0, instruction: Instruction(0) }
    }
}

pub struct RV32IMachine {
    registers: [i32; 31],
    pc: i32,

    if2dc: PipelineState,
    dc2ex: PipelineState,
    ex2mem: MemData,
    mem2wb: WriteBackData,

    memory: Box<Memory>,
}

impl RiscvIMachine for RV32IMachine {
    type IntegerType = i32;

    fn cycle(&mut self) {
        self.do_write_back();
        self.do_mem();
        self.do_execute();
        self.do_decode();
        self.do_fetch();
    }

    fn get_i_register(&self, i:usize) -> i32 {
        self.get_register(i)
    }
    fn set_i_register(&mut self, i:usize, value:i32) {
        self.set_register(i, value)
    }

    fn get_csr_field(&self, _i:CsrField) -> i32 { 0 }
    fn set_csr_field(&mut self, _i:CsrField, _value:i32) { }

    fn get_pc(&self) -> i32 { self.pc }
    fn set_pc(&mut self, value:i32) { self.pc = value }
}

impl RV32IMachine {

    pub fn new(mem:Box<Memory>) -> RV32IMachine {
        RV32IMachine {
            registers : [0; 31],
            pc: 0, 
            if2dc: PipelineState::empty(),
            dc2ex: PipelineState::empty(),
            ex2mem: MemData { pc: 0, wb_rd: 0, wb_perform: false, perform: None, 
                addr: 0, size: WordSize::B, value: 0 },
            mem2wb: WriteBackData { perform: false, rd: 0, value: 0 },
            memory: mem,
        }
    }

    pub fn get_pc(&self) -> i32 {
        self.pc
    }

    pub fn get_register(&self, i:usize) -> i32 {
        if i <= 0 || i > 31 {
            0
        } else {
            self.registers[i-1]
        }
    }

    pub fn set_register(&mut self, i:usize, x:i32) {
        // TODO implement exceptions when writing to r0
        if i > 0 && i < 32 {
            self.registers[i-1] = x
        }
    }

    pub fn get_csr_field(id:CsrId) {
        match id {
            CsrId::USTATUS => println!("coucou"),
            _ => println!("caca"),
        }
    }

    pub fn set_csr_field(id:CsrId) {
    }

    /// Executes a pipeline cycle
    pub fn cycle(&mut self) {
        // We perform operation in reverse order to simulate a pipeline. Each
        // step must execute something based on previously computed last step.
        self.do_write_back();
        self.do_mem();
        self.do_execute();
        self.do_decode();
        self.do_fetch()
    }

    fn do_write_back(&mut self) {
        if self.mem2wb.perform {
            let rd = self.mem2wb.rd;
            let value = self.mem2wb.value;
            self.set_register(rd, value)
        }
    }

    fn do_mem(&mut self) {
        let value : i32;
        let perform_wb : bool;
        let rd: usize = self.ex2mem.wb_rd;

        match &self.ex2mem.perform {
            Some(MemAction::Load) => {
                perform_wb = true;
                value = match self.ex2mem.size {
                    WordSize::B => self.memory.get_8(self.ex2mem.addr) as i32,
                    WordSize::H => self.memory.get_16(self.ex2mem.addr) as i32,
                    WordSize::W => self.memory.get_32(self.ex2mem.addr) as i32,
                    _ => 0,
                };
            },
            Some(MemAction::Store) => {
                let addr = self.ex2mem.addr;
                let val  = self.ex2mem.value;
                match self.ex2mem.size {
                    WordSize::B => self.memory.set_8(addr, val as u8),
                    WordSize::H => self.memory.set_16(addr, val as u16),
                    WordSize::W => self.memory.set_32(addr, val as u32),
                    _ => { },
                };
                perform_wb = false;
                value = 0
            },
            None => {
                perform_wb = self.ex2mem.wb_perform;
                value = self.ex2mem.value;
            }
        }

        self.mem2wb = WriteBackData { perform: perform_wb, value: value, rd: rd };

        // bypass
        if self.ex2mem.wb_perform {
            self.do_write_back()
        }
    }

    fn do_execute(&mut self) {
        let curr_pc = self.dc2ex.pc;
        let mut to_mem = MemData { pc: curr_pc, wb_perform: false, wb_rd: 0
            , value: 0, perform: None, addr: 0, size: WordSize::B };
        let i = self.dc2ex.instruction;
        match i.get_opcode_enum() {
            OpCode::LUI => {
                to_mem.wb_perform = true;
                to_mem.wb_rd = i.get_rd() as usize;
                to_mem.value = i.get_imm_u();
            },
            OpCode::AUIPC => {
                self.pc = curr_pc + i.get_imm_u();
            },
            OpCode::JAL => {
                to_mem.value = curr_pc.wrapping_add(4);
                to_mem.wb_perform = true;
                to_mem.wb_rd = i.get_rd() as usize;
                self.pc = curr_pc.wrapping_add(i.get_imm_j());
            },
            OpCode::JALR => {
                to_mem.value = curr_pc.wrapping_add(4);
                to_mem.wb_perform = true;
                to_mem.wb_rd = i.get_rd() as usize;
                self.pc = self.get_register(i.get_rs1() as usize).wrapping_add(i.get_imm_i());
            },
            OpCode::BRANCH => {
                let npc = curr_pc.wrapping_add(i.get_imm_b());
                
                let v1 = self.get_register(i.get_rs1() as usize);
                let uv1 = v1 as u32;

                let v2 = self.get_register(i.get_rs2() as usize);
                let uv2 = v2 as u32;

                self.pc = match i.get_funct3() {
                    0b000 => if  v1 ==  v2 { npc } else { self.pc }, // BEQ
                    0b001 => if  v1 !=  v2 { npc } else { self.pc }, // BNE
                    0b010 => if  v1 <   v2 { npc } else { self.pc }, // BLT
                    0b011 => if  v1 >=  v2 { npc } else { self.pc }, // BGE
                    0b100 => if uv1 <  uv2 { npc } else { self.pc }, // BLTU
                    0b101 => if uv1 >= uv2 { npc } else { self.pc }, // BGEU
                    _ => self.pc,
                }
            },
            OpCode::LOAD => {
                let width = WordSize::from(i.get_funct3());
                let base = self.get_register(i.get_rs1() as usize) as usize;
                to_mem.perform = Some(MemAction::Load);
                to_mem.addr = i.get_imm_i() as usize + base;
                to_mem.size = width;
            },
            OpCode::STORE => {
                let width = WordSize::from(i.get_funct3());
                let base = self.get_register(i.get_rs1() as usize) as usize;
                let src = self.get_register(i.get_rs2() as usize);
                to_mem.perform = Some(MemAction::Store);
                to_mem.addr = i.get_imm_s() as usize + base;
                to_mem.size = width;
                to_mem.value = src;
            },
            OpCode::OPIMM => {
                to_mem.wb_perform = true;
                to_mem.wb_rd = i.get_rd() as usize;

                let v1 = self.get_register(i.get_rs1() as usize);
                let v2 = if i.get_funct3() & 0b11 == 1 { i.get_rs2() as i32 }
                         else { i.get_imm_i() };

                to_mem.value = match i.get_funct3() {
                    0b000 => v1.wrapping_add(v2),
                    0b010 => (v1 < v2) as i32,
                    0b011 => ((v1 as u32) < v2 as u32) as i32,
                    0b100 => v1 ^ v2,
                    0b110 => v1 | v2,
                    0b111 => v1 & v2,
                    0b001 => v1 << v2,
                    0b101 => if i.get_funct7() != 0 { v1 >> v2 }
                             else { ((v1 as u32) >> v2) as i32 },
                    _ => 0, // Cannot be here, because funct3 is on 3 bits
                };
            },
            OpCode::OPREG => {
                to_mem.wb_perform = true;
                to_mem.wb_rd = i.get_rd() as usize;

                let v1 = self.get_register(i.get_rs1() as usize);
                let v2 = self.get_register(i.get_rs2() as usize);

                to_mem.value = match i.get_funct7() {
                    0b0000000 => match i.get_funct3() {
                        0b000 => v1.wrapping_add(v2),
                        0b001 => v1 >> v2,
                        0b010 => (v1 < v2) as i32,
                        0b011 => ((v1 as u32) < v2 as u32) as i32,
                        0b100 => v1 ^ v2,
                        0b101 => ((v1 as u32) >> v2) as i32,
                        0b110 => v1 | v2,
                        0b111 => v1 & v2,
                        _ => 0, // Cannot be here, because funct3 is on 3 bits
                    },
                    0b0000001 => match i.get_funct3() {
                        _ => 0, // TODO handle M extension (mul/div)
                    },
                    0b0100000 => match i.get_funct3() {
                        0b000 => v1.wrapping_sub(v2),
                        0b101 => v1 >> v2,
                        _ => 0, // TODO handle bad funct3 values
                    },
                    _ => 0, // TODO add other extensions (F has priority)
                };

            },
            _ => {}
        }

        self.ex2mem = to_mem
    }

    fn do_decode(&mut self) {
        self.dc2ex = self.if2dc
    }

    fn do_fetch(&mut self) {
        let i = Instruction(self.memory.get_32(self.pc as usize));
        self.if2dc = PipelineState { pc: self.pc, instruction: i };
        self.pc += 4
    }
}
