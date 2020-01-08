use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::LoadSequenceByIndex;

impl<P, O> VMInstruction<P, O> for LoadSequenceByIndex
where
    P: Primitive,
    O: PrimitiveOperations<P>,
{
    fn execute(&self, vm: &mut VirtualMachine<P, O>) -> Result<(), RuntimeError> {
        let index = vm.pop()?.value()?;

        let mut array = Vec::with_capacity(self.array_len);
        for i in 0..self.array_len {
            let value = vm.load(self.address + i)?.value()?;
            array.push(value);
        }

        let mut values = Vec::with_capacity(self.value_len);
        for i in 0..self.value_len {
            let offset = vm.operations().constant_bigint(&i.into())?;
            let address = vm.operations().add(index.clone(), offset)?;

            let value = vm.operations().array_get(array.as_slice(), address)?;
            values.push(value);
        }

        for value in values.into_iter().rev() {
            vm.push(Cell::Value(value))?;
        }

        Ok(())
    }
}
