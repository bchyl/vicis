use crate::codegen::{
    function::Function,
    module::Module,
    register::Reg,
    target::x86_64::{
        instruction::{Opcode, Operand, OperandData},
        X86_64,
    },
};
use std::fmt;

pub fn print(f: &mut fmt::Formatter<'_>, module: &Module<X86_64>) -> fmt::Result {
    writeln!(f, "  .text")?;
    writeln!(f, "  .intel_syntax noprefix")?;

    for (_, func) in &module.functions {
        print_function(f, func)?
    }

    Ok(())
}

pub fn print_function(f: &mut fmt::Formatter<'_>, function: &Function<X86_64>) -> fmt::Result {
    writeln!(f, "  .globl {}", function.name)?;
    writeln!(f, "{}:", function.name)?;

    for block in function.layout.block_iter() {
        writeln!(f, ".LBL{}:", block.index())?;
        for inst in function.layout.inst_iter(block) {
            let inst = function.data.inst_ref(inst);
            write!(f, "  {} ", inst.data.opcode)?;
            let mut i = 0;
            while i < inst.data.operands.len() {
                let operand = &inst.data.operands[i];
                if operand.implicit {
                    i += 1;
                    continue;
                }
                if matches!(operand.data, OperandData::MemStart) {
                    i += 1;
                    write!(f, "{} ptr ", mem_size(&inst.data.opcode))?;
                    write!(f, "{}", mem_op(&inst.data.operands[i..i + 5]))?;
                    i += 5 - 1;
                } else {
                    write!(f, "{}", operand.data)?;
                }
                if i < inst.data.operands.len() - 1 {
                    write!(f, ", ")?
                }
                i += 1;
            }
            writeln!(f)?;
        }
    }

    Ok(())
}

impl fmt::Display for Module<X86_64> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        print(f, self)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::PUSH64 => "push",
                Self::POP64 => "pop",
                Self::ADDr64i32 => "add",
                Self::ADDri32 => "add",
                Self::ADDrr32 => "add",
                Self::SUBr64i32 => "sub",
                Self::MOVrr32 => "mov",
                Self::MOVrr64 => "mov",
                Self::MOVri32 => "mov",
                Self::MOVrm32 => "mov",
                Self::MOVmi32 => "mov",
                Self::MOVmr32 => "mov",
                Self::MOVSXDr64r32 => "movsxd",
                Self::CMPri32 => "cmp",
                Self::JMP => "jmp",
                Self::JE => "je",
                Self::JNE => "jne",
                Self::JLE => "jle",
                Self::JL => "jl",
                Self::JGE => "jge",
                Self::JG => "jg",
                Self::CALL => "call",
                Self::RET => "ret",
                Self::Phi => "PHI",
            }
        )
    }
}

impl fmt::Display for OperandData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reg(r) => write!(f, "{}", reg_to_str(r)),
            Self::VReg(r) => write!(f, "%{}", r.0),
            // Self::Mem(mem) => write!(f, "{}", mem),
            Self::Slot(slot) => write!(f, "{:?}", slot),
            Self::Int32(i) => write!(f, "{}", i),
            Self::Block(block) => write!(f, ".LBL{}", block.index()),
            Self::Label(name) => write!(f, "{}", name),
            Self::MemStart => Ok(()),
            Self::None => write!(f, "none"),
        }
    }
}

fn reg_to_str(r: &Reg) -> &'static str {
    let gr32 = [
        "eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi", "r8", "r9d", "r10d", "r11d",
        "r12d", "r13d", "r14d", "r15d",
    ];
    let gr64 = [
        "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12",
        "r13", "r14", "r15",
    ];
    match r {
        Reg(0, i) => gr32[*i as usize],
        Reg(1, i) => gr64[*i as usize],
        e => todo!("{:?}", e),
    }
}

fn mem_size(opcode: &Opcode) -> &'static str {
    match opcode {
        Opcode::MOVrm32 | Opcode::MOVmi32 | Opcode::MOVmr32 => "dword",
        _ => todo!(),
    }
}

fn mem_op(args: &[Operand]) -> String {
    assert!(matches!(&args[0].data, &OperandData::None)); // assure slot is eliminated
    match (&args[1].data, &args[2].data, &args[3].data, &args[4].data) {
        (OperandData::Int32(imm), OperandData::Reg(reg), OperandData::None, OperandData::None) => {
            format!(
                "[{}{}{}]",
                reg_to_str(reg),
                if *imm < 0 { "" } else { "+" },
                *imm
            )
        }
        (
            OperandData::Int32(imm),
            OperandData::Reg(reg1),
            OperandData::Reg(reg2),
            OperandData::Int32(shift),
        ) => {
            format!(
                "[{}{}{}+{}*{}]",
                reg_to_str(reg1),
                if *imm < 0 { "" } else { "+" },
                *imm,
                reg_to_str(reg2),
                shift
            )
        }
        _ => todo!(),
    }
}
