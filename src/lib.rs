type Register = usize;

struct Processor {
    // XXX make registers just 4 bytes that are interpreted as necessary,
    //     e.g. SLTIU wants things treated as unsigned.
    registers: [u32; 33],  // registers[0] is unused; hard-wired to 0.
}

impl Processor {
    fn new() -> Processor {
        Processor{registers: [0; 33]}
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
            _ => self.registers[reg] = val
        }
    }

    /// Add a sign-extended immediate to `rs1`.
    ///
    /// Overflow is ignored.
    /// `ADDI rd, rs1, 0` == `MV rd, rs1`
    /// (section 2.4)
    fn addi(&mut self, rd: Register, rs1: Register, imm: u32) {
        let signed_imm = imm as i32;
        let rs1_val = self.get(rs1) as i32;
        let (result, _) = rs1_val.overflowing_add(signed_imm);
        self.set(rd, result as u32);
    }
}

fn sign_extend(imm: u32) -> u32 {
    // From https://github.com/riscv/riscv-tests/blob/master/isa/macros/scalar/test_macros.h
    let signed_imm = imm as i32;
    let extended_imm = signed_imm | (-(((signed_imm) >> 11) & 1) << 11);
    extended_imm as u32
}

#[test]
fn addi() {
    // https://github.com/riscv/riscv-tests/blob/master/isa/rv32ui/addi.S
    let mut cpu = Processor::new();
    let rd: Register = 1;
    let rs1: Register = 3;

    // Arithmetic
    cpu.set(rs1, 0x00000000);
    cpu.addi(rd, rs1, sign_extend(0x000));
    assert_eq!(0x00000000, cpu.get(rd));

    cpu.set(rs1, 0x00000001);
    cpu.addi(rd, rs1, sign_extend(0x001));
    assert_eq!(0x00000002, cpu.get(rd));

    cpu.set(rs1, 0x00000003);
    cpu.addi(rd, rs1, sign_extend(0x007));
    assert_eq!(0x0000000a, cpu.get(rd));

    cpu.set(rs1, 0x00000000);
    cpu.addi(rd, rs1, sign_extend(0x800));
    assert_eq!(0xfffff800, cpu.get(rd));

    cpu.set(rs1, 0x80000000);
    cpu.addi(rd, rs1, sign_extend(0x000));
    assert_eq!(0x80000000, cpu.get(rd));

    cpu.set(rs1, 0x80000000);
    cpu.addi(rd, rs1, sign_extend(0x800));
    assert_eq!(0x7ffff800, cpu.get(rd));

    cpu.set(rs1, 0x00000000);
    cpu.addi(rd, rs1, sign_extend(0x7ff));
    assert_eq!(0x000007ff, cpu.get(rd));

    cpu.set(rs1, 0x7fffffff);
    cpu.addi(rd, rs1, sign_extend(0x000));
    assert_eq!(0x7fffffff, cpu.get(rd));

    cpu.set(rs1, 0x7fffffff);
    cpu.addi(rd, rs1, sign_extend(0x7ff));
    assert_eq!(0x800007fe, cpu.get(rd));

    cpu.set(rs1, 0x80000000);
    cpu.addi(rd, rs1, sign_extend(0x7ff));
    assert_eq!(0x800007ff, cpu.get(rd));

    cpu.set(rs1, 0x7fffffff);
    cpu.addi(rd, rs1, sign_extend(0x800));
    assert_eq!(0x7ffff7ff, cpu.get(rd));

    cpu.set(rs1, 0x00000000);
    cpu.addi(rd, rs1, sign_extend(0xfff));
    assert_eq!(0xffffffff, cpu.get(rd));

    cpu.set(rs1, 0xffffffff);
    cpu.addi(rd, rs1, sign_extend(0x001));
    assert_eq!(0x00000000, cpu.get(rd));

    cpu.set(rs1, 0xffffffff);
    cpu.addi(rd, rs1, sign_extend(0xfff));
    assert_eq!(0xfffffffe, cpu.get(rd));

    cpu.set(rs1, 0x7fffffff);
    cpu.addi(rd, rs1, sign_extend(0x001));
    assert_eq!(0x80000000, cpu.get(rd));

    // Same source & destination
    cpu.set(1, 13);
    cpu.addi(1, 1, sign_extend(11));
    assert_eq!(24, cpu.get(1));

    // Operations involving x0
    cpu.addi(rd, 0, sign_extend(32));
    assert_eq!(32, cpu.get(rd));

    cpu.set(rs1, 33);
    cpu.addi(0, rs1, sign_extend(50));
    assert_eq!(0, cpu.get(0));

    // Ignoring bypassing tests as there's no need to worry about no-ops.
}
