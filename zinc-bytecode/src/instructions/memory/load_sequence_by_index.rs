use crate::{utils, DecodingError, Instruction, InstructionCode, InstructionInfo};

/// Takes `index` from evaluation stack, loads several values from data stack from `address + index` onto evaluation stack.
#[derive(Debug, PartialEq, Clone)]
pub struct LoadSequenceByIndex {
    pub address: usize,
    pub array_len: usize,
    pub value_len: usize,
}

impl LoadSequenceByIndex {
    pub fn new(address: usize, array_len: usize, value_len: usize) -> Self {
        Self {
            address,
            array_len,
            value_len,
        }
    }
}

impl InstructionInfo for LoadSequenceByIndex {
    fn to_assembly(&self) -> String {
        format!(
            "load_array_by_index {} {} {}",
            self.address, self.array_len, self.value_len
        )
    }

    fn code() -> InstructionCode {
        InstructionCode::LoadSequenceByIndex
    }

    fn encode(&self) -> Vec<u8> {
        utils::encode_with_args(
            Self::code(),
            &[self.address, self.array_len, self.value_len],
        )
    }

    fn decode(bytes: &[u8]) -> Result<(Self, usize), DecodingError> {
        let (args, len) = utils::decode_with_usize_args(Self::code(), bytes, 3)?;

        Ok((Self::new(args[0], args[1], args[2]), len))
    }

    fn inputs_count(&self) -> usize {
        1
    }

    fn outputs_count(&self) -> usize {
        self.value_len
    }

    fn wrap(&self) -> Instruction {
        Instruction::LoadSequenceByIndex((*self).clone())
    }
}
