use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use bitfield::bitfield;
use enum_primitive_derive::Primitive;
use num_traits::{FromPrimitive, ToPrimitive};
use strum_macros::Display;

macro_rules! impl_traits {
    ($t: ident) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }

            fn ne(&self, other: &Self) -> bool {
                self.0 != other.0
            }
        }

        impl Display for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                fmt::Debug::fmt(self, f) // Just reuse Debug implementation
            }
        }
    };
}

pub const INVALID_OPCODE7: Opcode7 = Opcode7(0);

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum InstructionSize {
    Byte       =  8,
    HalfWord   = 16,
    Word       = 32,
    DoubleWord = 64,
}

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum OpcodeType {
    Null                  = 0,         //  TODO: remove Null, use Option<OpcodeType>
    AddUpperImmediateToPC = 0b0010111, //  23
    Arithmetic            = 0b0110011, //  51
    ArithmeticImmediate   = 0b0010011, //  19
    Branch                = 0b1100011, //  99
    JumpAndLink           = 0b1101111, // 111
    JumpAndLinkRegister   = 0b1100111, // 103
    Load                  = 0b0000011, //   3
    LoadUpperImmediate    = 0b0110111, //  55
    Store                 = 0b0100011, //  35
}

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum Funct3Op {
    AddSub = 0,
    Sll    = 1,
    Slt    = 2,
    Sltu   = 3,
    Xor    = 4,
    SraSrl = 5,
    Or     = 6,
    And    = 7,
}

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum Funct3Branch {
    Beq  = 0,
    Bne  = 1,
    Blt  = 4,
    Bge  = 5,
    Bltu = 6,
    Bgeu = 7,
}

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum Funct3Load {
    Lb  = 0,
    Lh  = 1,
    Lw  = 2,
    Lbu = 4,
    Lhu = 5,
}

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum Funct3Store {
    Sb = 0,
    Sh = 1,
    Sw = 2,
}

#[derive(Debug, Eq, PartialEq, Primitive)]
pub enum Funct3System {
    // If the last 12 bits are 0, then it is ECALL otherwise EBREAK.:
    EcallEbreak = 0,
    Csrrw       = 1,
    Csrrs       = 2,
    Csrrc       = 3,
    Csrrwi      = 5,
    Csrrsi      = 6,
    Csrrci      = 7,
}

#[derive(Debug, PartialEq, Display)]
pub enum Instruction32 {
    R(RType32),
    I(IType32),
    S(SType32),
    B(BType32),
    U(UType32),
    J(JType32),
    Invalid(Raw32),
}

pub type Raw32 = u32;

bitfield! {
    pub struct Opcode7(u8);
    pub u8, get, set: 6, 0;
}

bitfield! {
    pub struct RType32(Raw32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, funct7, set_funct7: 31, 25;
}

bitfield! {
    pub struct IType32(Raw32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct SType32(Raw32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    imm_b7, set_imm_b7: 7;
    u8, imm_b11_8, set_imm_b11_8: 11, 8;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

// Note: same as SType32, but let's keep a separate type just in case
bitfield! {
    pub struct BType32(Raw32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    imm_b7, set_imm_b7: 7;
    u8, imm_b11_8, set_imm_b11_8: 11, 8;
    u8, funct3, set_funct3: 14, 12;
    u8, rs1, set_rs1: 19, 15;
    u8, rs2, set_rs2: 24, 20;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct UType32(Raw32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    u16, imm_b30_20, set_imm_b30_20: 30, 20;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct JType32(Raw32);
    impl Debug;
    u8, from into Opcode7, opcode, set_opcode: 6, 0;
    u8, rd, set_rd: 11, 7;
    u8, imm_b19_12, set_imm_b19_12: 19, 12;
    imm_b20, set_imm_b20: 20;
    u8, imm_b24_21, set_imm_b24_21: 24, 21;
    u8, imm_b30_25, set_imm_b30_25: 30, 25;
    imm_b31, set_imm_b31: 31;
}

bitfield! {
    pub struct Imm(Raw32);
    impl Debug;
    bit0, set_bit0: 0;
    u8, seq_b4_1, set_seq_b4_1: 4, 1;
    u8, seq_b10_5, set_seq_b10_5: 10, 5;
    bit11, set_bit11: 11;
    u8, seq_b19_12, set_seq_b19_12: 19, 12;
    u16, seq_b30_20, set_seq_b30_20: 30, 20;
    bit31, set_bit31: 31;
}

impl_traits!(Opcode7);
impl_traits!(RType32);
impl_traits!(IType32);
impl_traits!(SType32);
impl_traits!(BType32);
impl_traits!(UType32);
impl_traits!(JType32);
impl_traits!(Imm);

// TODO variable instruction length;
//      see https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf page 5

impl From<u8> for Opcode7 {
    fn from(value: u8) -> Self {
        Opcode7(value)
    }
}

impl From<OpcodeType> for Opcode7 {
    fn from(value: OpcodeType) -> Self {
        Opcode7(value.to_u8().unwrap())
    }
}

impl From<Opcode7> for u8 {
    fn from(Opcode7(value): Opcode7) -> Self {
        value
    }
}

impl From<OpcodeType> for u8 {
    fn from(value: OpcodeType) -> Self {
        value.to_u8().unwrap()
    }
}

impl Debug for Opcode7 {
    fn fmt(&self, form: &mut Formatter<'_>) -> fmt::Result {
        form.debug_tuple("Opcode7").field(&self.0).finish()
    }
}

impl From<Raw32> for Instruction32 {
    fn from(value: Raw32) -> Self {
        let r_type_value = RType32(value);
        let Opcode7(opcode) = r_type_value.opcode();
        let opcode_type = OpcodeType::from_u8(opcode).unwrap_or(OpcodeType::Null);

        match opcode_type {
            OpcodeType::ArithmeticImmediate | OpcodeType::JumpAndLinkRegister | OpcodeType::Load =>
                Instruction32::I(IType32(value)),
            OpcodeType::Arithmetic =>
                Instruction32::R(r_type_value), // we already have this one declared above...
            OpcodeType::JumpAndLink =>
                Instruction32::J(JType32(value)),
            OpcodeType::LoadUpperImmediate | OpcodeType::AddUpperImmediateToPC =>
                Instruction32::U(UType32(value)),
            OpcodeType::Branch =>
                Instruction32::B(BType32(value)),
            OpcodeType::Store =>
                Instruction32::S(SType32(value)),
            _ =>
                Instruction32::Invalid(value),
        }
    }
}

impl Instruction32 {
    pub fn get_opcode(&self) -> Opcode7 {
        match self {
            Instruction32::R(r) => r.opcode(),
            Instruction32::I(i) => i.opcode(),
            Instruction32::S(s) => s.opcode(),
            Instruction32::B(b) => b.opcode(),
            Instruction32::U(u) => u.opcode(),
            Instruction32::J(j) => j.opcode(),
            _ => INVALID_OPCODE7,
        }
    }

    pub fn get_imm(&self) -> Option<Imm> {
        let mut result = Imm(0);

        match self {
            Instruction32::I(i) => {
                result.set_bit0(i.imm_b20());
                result.set_seq_b4_1(i.imm_b24_21());
                result.set_seq_b10_5(i.imm_b30_25());
                result.set_bit31(i.imm_b31());
            }
            Instruction32::S(s) => {
                result.set_bit0(s.imm_b7());
                result.set_seq_b4_1(s.imm_b11_8());
                result.set_seq_b10_5(s.imm_b30_25());
                result.set_bit31(s.imm_b31());
            }
            Instruction32::B(b) => {
                result.set_seq_b4_1(b.imm_b11_8());
                result.set_seq_b10_5(b.imm_b30_25());
                result.set_bit11(b.imm_b7());
                result.set_bit31(b.imm_b31());
            }
            Instruction32::U(u) => {
                result.set_seq_b19_12(u.imm_b19_12());
                result.set_seq_b30_20(u.imm_b30_20());
                result.set_bit31(u.imm_b31());
            }
            Instruction32::J(j) => {
                result.set_seq_b4_1(j.imm_b24_21());
                result.set_seq_b10_5(j.imm_b30_25());
                result.set_bit11(j.imm_b20());
                result.set_seq_b19_12(j.imm_b19_12());
                result.set_bit31(j.imm_b31());
            }
            _ => return None,
        }

        Some(result)
    }
}
