mod memory;
mod state;
mod internal;
mod data_stack;

pub use memory::*;
pub use state::*;
pub use internal::*;

use crate::primitive::{Primitive, PrimitiveOperations};
use num_bigint::BigInt;
use zinc_bytecode::{dispatch_instruction, Instruction, InstructionInfo};
use franklin_crypto::bellman::SynthesisError;
use crate::vm::memory::Memory;
use crate::vm::data_stack::DataStack;

pub trait VMInstruction<E, O>: InstructionInfo
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError>;
}

#[derive(Debug)]
pub enum RuntimeError {
    InvalidArguments,
    StackUnderflow,
    StackOverflow,
    UnexpectedEndOfFile,
    SynthesisError(SynthesisError),
    InternalError(String),
    IntegerOverflow,
    UnexpectedLoopEnd,
    UnexpectedReturn,
    UnexpectedFrameExit,
    UnexpectedElse,
    UnexpectedEndIf,
    AssertionError,
    FirstInstructionNotCall,
    WrongInputsCount,
    StackIndexOutOfRange,
    UninitializedStorageAccess,
    MissingArgument,
    BranchStacksDoNotMatch,
    IndexOutOfBounds,
    UsingMerkleTreeAsValue,
}

pub struct VirtualMachine<E: Primitive, O: PrimitiveOperations<E>> {
    state: State<E>,
    operator: O,
    outputs: Vec<E>,
}

impl<P: Primitive, O: PrimitiveOperations<P>> VirtualMachine<P, O> {
    pub fn new(operator: O) -> Self {
        Self {
            state: State {
                instruction_counter: 0,
                conditions_stack: vec![],
                data_stack: DataStack::new(),
                function_frames: vec![],
            },
            operator,
            outputs: vec![],
        }
    }

    pub fn run(
        &mut self,
        instructions: &[Instruction],
        inputs: Option<&[BigInt]>,
    ) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let one = self.operator.constant_bigint(&1.into())?;
        self.condition_push(one)?;

        match instructions.first() {
            Some(Instruction::Call(call)) => {
                self.init_root_frame(call.inputs_count, inputs)?;
            },
            _ => {
                unimplemented!("Call instruction must be the first one!")
            }
        }

        while self.state.instruction_counter < instructions.len() {
            let instruction = &instructions[self.state.instruction_counter];
            self.state.instruction_counter += 1;
            log::info!(
                "> {}",
                dispatch_instruction!(instruction => instruction.to_assembly())
            );
            dispatch_instruction!(instruction => instruction.execute(self))?;
            log::info!("{}", self.stack_to_string());
        }

        self.get_outputs()
    }

    fn init_root_frame(&mut self, inputs_count: usize, inputs: Option<&[BigInt]>) -> Result<(), RuntimeError> {
        self.state.function_frames.push(
            FunctionFrame::new(0, std::usize::MAX, &[])
        );

        match inputs {
            None => {
                for _ in 0..inputs_count {
                    let variable = self.operator.variable_none()?;
                    self.push(variable)?;
                }
            },
            Some(values) => {
                for value in values.iter() {
                    let variable = self.operator.variable_bigint(value)?;
                    self.push(variable)?;
                }
            },
        }

        Ok(())
    }

    fn get_outputs(&mut self) -> Result<Vec<Option<BigInt>>, RuntimeError> {
        let mut outputs = Vec::new();

        for o in self.outputs.iter() {
            let e = self.operator.output(o.clone())?;
            outputs.push(e.to_bigint());
        }

        Ok(outputs)
    }

    fn stack_to_string(&self) -> String {
        format!("{:?}", self.state)
    }

    pub fn get_operator(&mut self) -> &mut O {
        &mut self.operator
    }

    pub fn condition_push(&mut self, element: P) -> Result<(), RuntimeError> {
        self.state.conditions_stack.push(element);
        Ok(())
    }

    pub fn condition_pop(&mut self) -> Result<P, RuntimeError> {
        self.state.conditions_stack.pop().ok_or(RuntimeError::StackUnderflow)
    }

    pub fn condition_top(&mut self) -> Result<P, RuntimeError> {
        self.state.conditions_stack
            .last()
            .map(|e| (*e).clone())
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn memory(&mut self) -> Result<&mut Memory<P>, RuntimeError> {
        self.frame()?.memory_snapshots.last_mut().ok_or(RuntimeError::StackUnderflow)
    }

    fn frame(&mut self) -> Result<&mut FunctionFrame<P>, RuntimeError> {
        self.state.function_frames.last_mut().ok_or(RuntimeError::StackUnderflow)
    }
}
