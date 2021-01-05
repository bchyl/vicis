use crate::codegen::{
    function::{
        data::Data as MachData,
        slot::{SlotId, Slots},
    },
    instruction::Instruction as MachInstruction,
    target::Target,
};
use crate::ir::{
    function::Data as IrData,
    instruction::{Instruction, InstructionId},
};
use rustc_hash::FxHashMap;

pub trait Lower<T: Target> {
    fn lower(
        &self,
        ctx: &mut LoweringContext<T>,
        inst: &Instruction,
    ) -> Vec<MachInstruction<T::InstData>>;
}

pub struct LoweringContext<'a, T: Target> {
    pub ir_data: &'a IrData,
    pub mach_data: &'a mut MachData<T::InstData>,
    pub slots: &'a mut Slots<T>,
    pub inst_id_to_slot_id: FxHashMap<InstructionId, SlotId>,
}
