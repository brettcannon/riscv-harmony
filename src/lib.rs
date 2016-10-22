/// A RISC-V simulator baed on
/// ([the RISC-V Instruction Set Manual](https://riscv.org/specifications/),
///  Volume 1, Version, 2.1, Section 2.4).

type Register = usize;

struct Processor {
    // XXX make registers just 4 bytes that are interpreted as necessary,
    //     e.g. SLTIU wants things treated as unsigned.
    registers: [u32; 33], // registers[0] is unused; hard-wired to 0.
}

impl Processor {
    fn new() -> Processor {
        Processor { registers: [0; 33] }
    }

    fn get(&mut self, reg: Register) -> u32 {
        match reg {
            0 => 0,
            _ => self.registers[reg],
        }
    }

    fn set(&mut self, reg: Register, val: u32) {
        match reg {
            0 => (),  // No-op
            _ => self.registers[reg] = val,
        }
    }

    /// Add a sign-extended immediate to `rs1`.
    ///
    /// Overflow is ignored.
    /// `ADDI rd, rs1, 0` == `MV rd, rs1`
    fn addi(&mut self, rd: Register, rs1: Register, imm: u32) {
        let signed_imm = imm as i32;
        let rs1_val = self.get(rs1) as i32;
        let (result, _) = rs1_val.overflowing_add(signed_imm);
        self.set(rd, result as u32);
    }

    /// Check if `rs1` is less than the sign-extended `imm`.
    fn slti(&mut self, rd: Register, rs1: Register, imm: u32) {
        let signed_imm = imm as i32;
        let rs1_val = self.get(rs1) as i32;
        self.set(rd, if rs1_val < signed_imm { 1 } else { 0 })
    }

    /// Check if `rs1` is less than sign-extended `imm` in an unsigned comparison.
    ///
    /// `SLTIU rd, rs1, 1` == `SEQZ rd, rs`
    fn sltiu(&mut self, rd: Register, rs1: Register, imm: u32) {
        let rs1_val: u32 = self.get(rs1);
        if imm == 1 {
            // SEQZ pseudo-op.
            self.set(rd, if rs1_val == 0 { 1 } else { 0 })
        } else {
            self.set(rd, if rs1_val < imm { 1 } else { 0 })
        }
    }

    /// Perform a bitwise AND against `imm`.
    fn andi(&mut self, rd: Register, rs1: Register, imm: u32) {
        let rs1_val = self.get(rs1);
        self.set(rd, rs1_val & imm);
    }

    /// Perform a bitwise OR against `imm`.
    fn ori(&mut self, rd: Register, rs1: Register, imm: u32) {
        let rs1_val = self.get(rs1);
        self.set(rd, rs1_val | imm);
    }

    /// Perform a bitwise XOR against `imm`.
    ///
    /// `XORI rd, sr1, -1` == `NOT rd, rs`
    fn xori(&mut self, rd: Register, rs1: Register, imm: u32) {
        let rs1_val = self.get(rs1);
        self.set(rd, rs1_val ^ imm);
    }
}

fn sign_extend(imm: u32) -> u32 {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/macros/scalar/test_macros.h
    let signed_imm = imm as i32;
    let extended_imm = signed_imm | (-(((signed_imm) >> 11) & 1) << 11);
    extended_imm as u32
}

macro_rules! test_imm_op {
    ($test_num: expr, $inst:ident, $result:expr, $val1:expr, $imm:expr) => {{
        let mut cpu = Processor::new();
        let rd: Register = 1;
        let rs1: Register = 3;
        cpu.set(rs1, $val1);
        cpu.$inst(rd, rs1, sign_extend($imm));
        assert_eq!($result, cpu.get(rd));
    }};
}

macro_rules! test_imm_src1_eq_dest {
    ($test_num:expr, $inst:ident, $result:expr, $val1:expr, $imm:expr) => {{
        let mut cpu = Processor::new();
        let rd: Register = 1;
        let rs1: Register = 1;
        cpu.set(rs1, $val1);
        cpu.$inst(rd, rs1, sign_extend($imm));
        assert_eq!($result, cpu.get(rd));
    }}
}

macro_rules! test_imm_zerosrc1 {
    ($test_num:expr, $inst:ident, $result:expr, $imm:expr) => {{
        let mut cpu = Processor::new();
        let rd: Register = 1;
        let rs1: Register = 0;
        cpu.$inst(rd, rs1, sign_extend($imm));
        assert_eq!($result, cpu.get(rd));
    }}
}

macro_rules! test_imm_zerodest {
    ($test_num:expr, $inst:ident, $val1:expr, $imm:expr) => {{
        let mut cpu = Processor::new();
        let rd: Register = 0;
        let rs1: Register = 1;
        cpu.$inst(rd, rs1, $imm);
        assert_eq!(0, cpu.get(rd));
    }}
}

#[test]
fn addi() {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/rv64ui/addi.S
    test_imm_op!(2, addi, 0x00000000, 0x00000000, 0x000);
    test_imm_op!(3, addi, 0x00000002, 0x00000001, 0x001);
    test_imm_op!(4, addi, 0x0000000a, 0x00000003, 0x007);

    test_imm_op!(5, addi, 0xfffff800, 0x00000000, 0x800);
    test_imm_op!(6, addi, 0x80000000, 0x80000000, 0x000);
    test_imm_op!(7, addi, 0x7ffff800, 0x80000000, 0x800);

    test_imm_op!(8, addi, 0x000007ff, 0x00000000, 0x7ff);
    test_imm_op!(9, addi, 0x7fffffff, 0x7fffffff, 0x000);
    test_imm_op!(10, addi, 0x800007fe, 0x7fffffff, 0x7ff);

    test_imm_op!(11, addi, 0x800007ff, 0x80000000, 0x7ff);
    test_imm_op!(12, addi, 0x7ffff7ff, 0x7fffffff, 0x800);

    test_imm_op!(13, addi, 0xffffffff, 0x00000000, 0xfff);
    test_imm_op!(14, addi, 0x00000000, 0xffffffff, 0x001);
    test_imm_op!(15, addi, 0xfffffffe, 0xffffffff, 0xfff);

    test_imm_op!(16, addi, 0x80000000, 0x7fffffff, 0x001);

    test_imm_src1_eq_dest!(17, addi, 24, 13, 11);

    test_imm_zerosrc1!(24, addi, 32, 32);
    test_imm_zerodest!(25, addi, 33, 50);
}

#[test]
fn slti() {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/rv64ui/slti.S
    test_imm_op!(2, slti, 0, 0x00000000, 0x000);
    test_imm_op!(3, slti, 0, 0x00000001, 0x001);
    test_imm_op!(4, slti, 1, 0x00000003, 0x007);
    test_imm_op!(5, slti, 0, 0x00000007, 0x003);

    test_imm_op!(6, slti, 0, 0x00000000, 0x800);
    test_imm_op!(7, slti, 1, 0x80000000, 0x000);
    test_imm_op!(8, slti, 1, 0x80000000, 0x800);

    test_imm_op!(9, slti, 1, 0x00000000, 0x7ff);
    test_imm_op!(10, slti, 0, 0x7fffffff, 0x000);
    test_imm_op!(11, slti, 0, 0x7fffffff, 0x7ff);

    test_imm_op!(12, slti, 1, 0x80000000, 0x7ff);
    test_imm_op!(13, slti, 0, 0x7fffffff, 0x800);

    test_imm_op!(14, slti, 0, 0x00000000, 0xfff);
    test_imm_op!(15, slti, 1, 0xffffffff, 0x001);
    test_imm_op!(16, slti, 0, 0xffffffff, 0xfff);

    test_imm_src1_eq_dest!(17, slti, 1, 11, 13);

    test_imm_zerosrc1!(24, slti, 0, 0xfff);
    test_imm_zerodest!(25, slti, 0x00ff00ff, 0xfff);
}


#[test]
fn sltiu() {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/rv64ui/sltiu.S
    test_imm_op!(2, sltiu, 0, 0x00000000, 0x000);
    test_imm_op!(4, sltiu, 1, 0x00000003, 0x007);
    test_imm_op!(5, sltiu, 0, 0x00000007, 0x003);

    test_imm_op!(6, sltiu, 1, 0x00000000, 0x800);
    test_imm_op!(7, sltiu, 0, 0x80000000, 0x000);
    test_imm_op!(8, sltiu, 1, 0x80000000, 0x800);

    test_imm_op!(9, sltiu, 1, 0x00000000, 0x7ff);
    test_imm_op!(10, sltiu, 0, 0x7fffffff, 0x000);
    test_imm_op!(11, sltiu, 0, 0x7fffffff, 0x7ff);

    test_imm_op!(12, sltiu, 0, 0x80000000, 0x7ff);
    test_imm_op!(13, sltiu, 1, 0x7fffffff, 0x800);

    test_imm_op!(14, sltiu, 1, 0x00000000, 0xfff);
    test_imm_op!(15, sltiu, 0, 0xffffffff, 0x001);
    test_imm_op!(16, sltiu, 0, 0xffffffff, 0xfff);

    test_imm_src1_eq_dest!(17, sltiu, 1, 11, 13);

    test_imm_zerosrc1!(24, sltiu, 1, 0xfff);
    test_imm_zerodest!(25, sltiu, 0x00ff00ff, 0xfff);

    // SEQZ
    test_imm_op!(3, sltiu, 1, 0x00000000, 0x001);
    test_imm_op!(3, sltiu, 0, 0x00000001, 0x001);
    test_imm_op!(3, sltiu, 0, 0x00000002, 0x001);
}

#[test]
fn andi() {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/rv64ui/andi.S
    test_imm_op!(2, andi, 0xff00ff00, 0xff00ff00, 0xf0f);
    test_imm_op!(3, andi, 0x000000f0, 0x0ff00ff0, 0x0f0);
    test_imm_op!(4, andi, 0x0000000f, 0x00ff00ff, 0x70f);
    test_imm_op!(5, andi, 0x00000000, 0xf00ff00f, 0x0f0);

    test_imm_src1_eq_dest!(6, andi, 0x00000000, 0xff00ff00, 0x0f0);

    test_imm_zerosrc1!(13, andi, 0, 0x0f0);
    test_imm_zerodest!(14, andi, 0x00ff00ff, 0x70f);
}

#[test]
fn ori() {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/rv64ui/ori.S
    test_imm_op!(2, ori, 0xffffff0f, 0xff00ff00, 0xf0f);
    test_imm_op!(3, ori, 0x0ff00ff0, 0x0ff00ff0, 0x0f0);
    test_imm_op!(4, ori, 0x00ff07ff, 0x00ff00ff, 0x70f);
    test_imm_op!(5, ori, 0xf00ff0ff, 0xf00ff00f, 0x0f0);

    test_imm_src1_eq_dest!(6, ori, 0xff00fff0, 0xff00ff00, 0x0f0);

    test_imm_zerosrc1!(13, ori, 0x0f0, 0x0f0);
    test_imm_zerodest!(14, ori, 0x00ff00ff, 0x70f);
}

#[test]
fn xori() {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/rv64ui/xori.S
    test_imm_op!(2, xori, 0xff00f00f, 0x00ff0f00, 0xf0f);
    test_imm_op!(3, xori, 0x0ff00f00, 0x0ff00ff0, 0x0f0);
    test_imm_op!(4, xori, 0x00ff0ff0, 0x00ff08ff, 0x70f);
    test_imm_op!(5, xori, 0xf00ff0ff, 0xf00ff00f, 0x0f0);

    test_imm_src1_eq_dest!(6, xori, 0xff00f00f, 0xff00f700, 0x70f);

    test_imm_zerosrc1!(13, xori, 0x0f0, 0x0f0);
    test_imm_zerodest!(14, xori, 0x00ff00ff, 0x70f);
}
