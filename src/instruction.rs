extern crate bitmatch;

use std::fmt;

use bitmatch::bitmatch;

use crate::instruction::Instruction::*;
use crate::instruction::OpCode::*;

const EXPAND: [i8; 4] = [0, 1, -2, -1];
const OPCODE_NAMES: [&'static str; 16] = [
    "T", "N", "T+N", "T∧N", "T∨N", "T⊻N", "¬T", "N=T",
    "N<T", "N≫T", "T-1", "R", "[T]", "N≪T", "D", "Nu<T"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    // Literal value
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   1  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── value
    //
    Literal(u16),

    // Jump instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   0  0  0  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── target
    //
    Jump(u16),

    // Conditional jump instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   0  0  1  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── target
    //
    Conditional(u16),

    // Call instruction
    //
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   0  1  0  └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴── target
    //
    Call(u16),

    // ALU instruction
    //
    //   0  1  1  ?  b  b  b  b  c  d  e  a  x  x  y  y   match below
    //  15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
    //   │  │  │  │  │  │  │  │  │  │  │  │  │  │  0  1 ─ d + 1
    //   │  │  │  │  │  │  │  │  │  │  │  │  │  │  1  1 ─ d - 1
    //   │  │  │  │  │  │  │  │  │  │  │  │  0  1  ────── r + 1
    //   │  │  │  │  │  │  │  │  │  │  │  │  1  0  ────── r - 2
    //   │  │  │  │  │  │  │  │  │  │  │  │  1  1  ────── r - 1
    //   │  │  │  │  │  │  │  │  │  │  │  └────────────── R → PC
    //   │  │  │  │  │  │  │  │  │  │  └───────────────── N → [T]
    //   │  │  │  │  │  │  │  │  │  └──────────────────── T → R
    //   │  │  │  │  │  │  │  │  └─────────────────────── T → N
    //   │  │  │  │  │  │  │  │
    //   │  │  │  │  └──┴──┴──┴────────────────────────── t'
    //   │  │  │  └────────────────────────────────────── unused
    //   └──┴──┴───────────────────────────────────────── 0 1 1
    //
    ALU(AluAttributes),
}

#[bitmatch]
pub fn decode(v: u16) -> Result<Instruction, String> {
    #[bitmatch]
    match v {
        "1aaa_aaaa_aaaa_aaaa" => Ok(Literal(a)),
        "000a_aaaa_aaaa_aaaa" => Ok(Jump(a)),
        "001a_aaaa_aaaa_aaaa" => Ok(Conditional(a)),
        "010a_aaaa_aaaa_aaaa" => Ok(Call(a)),
        "011?_bbbb_cdea_xxyy" => {
            let opcode = OpCode::from(b);
            if opcode == None {
                return Err(format!("Invalid opcode for ALU instruction: {:0>4x}", v))
            }
            let alu_attributes = AluAttributes {
                opcode: OpCode::from(b).unwrap(),
                r2pc: a != 0,
                t2n: c != 0,
                t2r: d != 0,
                n2_at_t: e != 0,
                r_dir: EXPAND[x as usize],
                d_dir: EXPAND[y as usize],
            };
            Ok(ALU(alu_attributes))
        }
        _ => Err(format!("Invalid Instruction: {:0>4x}", v)),
    }
}

impl Instruction {
    pub fn show(&self) -> String {
        match self {
            Literal(v) => format!("LIT     {:0>4X}", v),
            Jump(v) => format!("UBRANCH {:0>4X}", (v << 1)),
            Conditional(v) => format!("0BRANCH {:0>4X}", (v << 1)),
            Call(v) => format!("CALL    {:0>4X}", (v << 1)),
            ALU(alu) => alu.show(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Jump(v) => write!(f, "Instruction::Jump(0x{:0>4X})", v),
            Instruction::Conditional(v) => write!(f, "Instruction::Conditional(0x{:0>4X})", v),
            Instruction::Call(v) => write!(f, "Instruction::Call(0x{:0>4X})", v),
            Instruction::Literal(v) => write!(f, "Instruction::Literal(0x{:0>4X})", v),
            Instruction::ALU(v) => write!(f, "Instruction::ALU({:?})", *v),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpCode {
    OpT = 0,
    OpN = 1,
    OpTplusN = 2,
    OpTandN = 3,
    OpTorN = 4,
    OpTxorN = 5,
    OpNotT = 6,
    OpNeqT = 7,
    OpNleT = 8,
    OpNrshiftT = 9,
    OpTminus1 = 10,
    OpR = 11,
    OpAtT = 12,
    OpNlshiftT = 13,
    OpDepth = 14,
    OpNuleT = 15,
}

impl Default for OpCode {
    fn default() -> Self { OpT }
}

impl OpCode {
    pub fn from(x: u16) -> Option<OpCode> {
        match x {
            0 => Some(OpT),
            1 => Some(OpN),
            2 => Some(OpTplusN),
            3 => Some(OpTandN),
            4 => Some(OpTorN),
            5 => Some(OpTxorN),
            6 => Some(OpNotT),
            7 => Some(OpNeqT),
            8 => Some(OpNleT),
            9 => Some(OpNrshiftT),
            10 => Some(OpTminus1),
            11 => Some(OpR),
            12 => Some(OpAtT),
            13 => Some(OpNlshiftT),
            14 => Some(OpDepth),
            15 => Some(OpNuleT),
            _ => None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AluAttributes {
    pub opcode: OpCode,
    pub r2pc: bool,
    pub t2n: bool,
    pub t2r: bool,
    pub n2_at_t: bool,
    pub r_dir: i8,
    pub d_dir: i8,
}

impl AluAttributes {
    pub fn show(&self) -> String {
        let mut s = "ALU     ".to_string();
        s = format!("{}{}", s, OPCODE_NAMES[self.opcode as usize]);
        if self.r2pc { s = format!("{} R→PC", s) }
        if self.t2n { s = format!("{} T→N", s) }
        if self.t2r { s = format!("{} T→R", s) }
        if self.n2_at_t { s = format!("{} N→[T]", s) }
        if self.r_dir != 0 { s = format!("{} r{:+}", s, self.r_dir) }
        if self.d_dir != 0 { s = format!("{} d{:+}", s, self.d_dir) }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::*;
    use crate::instruction::OpCode::{OpDepth, OpT};

    #[test]
    fn op_code() {
        let op_t = OpT as u16;
        let op_depth = OpDepth as u16;
        assert_eq!((0u16, 14u16), (op_t, op_depth))
    }

    #[test]
    fn alu_attributes() {
        // println!("default = {:?}", AluAttributes::default());
        assert_eq!(
            AluAttributes::default(),
            AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 0 }
        );
    }

    #[test]
    fn instruction_decode() {
        let test_cases = [
            (0x0000, Jump(0x0000)),
            (0x1fff, Jump(0x1fff)),
            (0x2000, Conditional(0x0000)),
            (0x3fff, Conditional(0x1fff)),
            (0x4000, Call(0x0000)),
            (0x5fff, Call(0x1fff)),
            (0x8000, Literal(0x0000)),
            (0xffff, Literal(0x7fff)),
            (0x6000, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 0 })),
            (0x6100, ALU(AluAttributes { opcode: OpN, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 0 })),
            (0x6010, ALU(AluAttributes { opcode: OpT, r2pc: true, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 0 })),
            (0x6080, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: true, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 0 })),
            (0x6040, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: true, n2_at_t: false, r_dir: 0, d_dir: 0 })),
            (0x6020, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: true, r_dir: 0, d_dir: 0 })),
            (0x600c, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: -1, d_dir: 0 })),
            (0x6004, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 1, d_dir: 0 })),
            (0x6003, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: -1 })),
            (0x6001, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 1 })),
            (0x6f00, ALU(AluAttributes { opcode: OpNuleT, r2pc: false, t2n: false, t2r: false, n2_at_t: false, r_dir: 0, d_dir: 0 })),
            (0x70e5, ALU(AluAttributes { opcode: OpT, r2pc: false, t2n: true, t2r: true, n2_at_t: true, r_dir: 1, d_dir: 1 })),
            (0x7fef, ALU(AluAttributes { opcode: OpNuleT, r2pc: false, t2n: true, t2r: true, n2_at_t: true, r_dir: -1, d_dir: -1 })),
        ];
        for (bin, expected_instruction) in test_cases.iter() {
            let decoded = decode(*bin).unwrap();
            assert_eq!(*expected_instruction, decoded);
            // println!("decode(0x{:0>4x}) = {}", *bin, decoded)
        }
    }

    #[test]
    fn instruction_show() {
        let test_cases = [
            (0x0000, "UBRANCH 0000".to_string()),
            (0x1fff, "UBRANCH 3FFE".to_string()),
            (0x2000, "0BRANCH 0000".to_string()),
            (0x3fff, "0BRANCH 3FFE".to_string()),
            (0x4000, "CALL    0000".to_string()),
            (0x5fff, "CALL    3FFE".to_string()),
            (0x8000, "LIT     0000".to_string()),
            (0xffff, "LIT     7FFF".to_string()),
            (0x6000, "ALU     T".to_string()),
            (0x6100, "ALU     N".to_string()),
            (0x7000, "ALU     T".to_string()),
            (0x6011, "ALU     T R→PC d+1".to_string()),
            (0x6012, "ALU     T R→PC d-2".to_string()),
            (0x6013, "ALU     T R→PC d-1".to_string()),
            (0x6080, "ALU     T T→N".to_string()),
            (0x6040, "ALU     T T→R".to_string()),
            (0x6020, "ALU     T N→[T]".to_string()),
            (0x600c, "ALU     T r-1".to_string()),
            (0x6004, "ALU     T r+1".to_string()),
            (0x6003, "ALU     T d-1".to_string()),
            (0x6001, "ALU     T d+1".to_string()),
            (0x6f00, "ALU     Nu<T".to_string()),
            (0x70e5, "ALU     T T→N T→R N→[T] r+1 d+1".to_string()),
            (0x7fef, "ALU     Nu<T T→N T→R N→[T] r-1 d-1".to_string()),
        ];
        for (bin, expected_show) in test_cases.iter() {
            let decoded = decode(*bin).unwrap();

            let decoded_show = decoded.show();
            assert_eq!(*expected_show, decoded_show);
            // println!("i.show()   : {}", decoded_show);
        }
    }
}

