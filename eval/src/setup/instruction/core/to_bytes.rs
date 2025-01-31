// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

// This macro can be more efficient once concat_idents is merged in.
macro_rules! to_bytes_impl {
    ($le_function_name:ident, $le_constant_name:ident, $le_constant_value:literal, $be_function_name:ident, $be_constant_name:ident, $be_constant_value:literal) => {
        pub const $le_constant_name: &str = $le_constant_value;

        impl<'a, F: PrimeField, G: GroupType<F>> EvaluatorState<'a, F, G> {
            pub fn $le_function_name<CS: ConstraintSystem<F>>(
                &mut self,
                arguments: &[ConstrainedValue<F, G>],
                cs: &mut CS,
            ) -> Result<ConstrainedValue<F, G>> {
                let _cs = self.cs(cs);

                let bytes = match arguments.get(0) {
                    None => return Err(anyhow!("illegal `to_bytes_le` call, expected call on target")),
                    Some(value) => value,
                };

                Err(anyhow!(
                    "`to_bytes_le` call not implemented for target `{}`",
                    bytes
                ))

                /* Ok(ConstrainedValue::Array(
                    bytes
                        .into_iter()
                        .map(Integer::U8)
                        .map(ConstrainedValue::Integer)
                        .collect(),
                )) */
            }
        }

        pub const $be_constant_name: &str = $be_constant_value;

        impl<'a, F: PrimeField, G: GroupType<F>> EvaluatorState<'a, F, G> {
            pub fn $be_function_name<CS: ConstraintSystem<F>>(
                &mut self,
                arguments: &[ConstrainedValue<F, G>],
                cs: &mut CS,
            ) -> Result<ConstrainedValue<F, G>> {
                let _cs = self.cs(cs);

                let bytes = match arguments.get(0) {
                    None => return Err(anyhow!("illegal `to_bytes_be` call, expected call on target")),
                    Some(value) => value,
                };

                Err(anyhow!(
                    "`to_bytes_be` call not implemented for target `{}`",
                    bytes
                ))

                /* Ok(ConstrainedValue::Array(
                    bytes
                        .into_iter()
                        .map(Integer::U8)
                        .map(ConstrainedValue::Integer)
                        .collect(),
                )) */
            }
        }
    };
}

// LE
to_bytes_impl!(
    call_core_address_to_bytes_le,
    ADDRESS_TO_BYTES_LE_CORE,
    "address_to_bytes_le",
    call_core_address_to_bytes_be,
    ADDRESS_TO_BYTES_BE_CORE,
    "address_to_bytes_be"
);
to_bytes_impl!(
    call_core_bool_to_bytes_le,
    BOOL_TO_BYTES_LE_CORE,
    "bool_to_bytes_le",
    call_core_bool_to_bytes_be,
    BOOL_TO_BYTES_BE_CORE,
    "bool_to_bytes_be"
);
to_bytes_impl!(
    call_core_char_to_bytes_le,
    CHAR_TO_BYTES_LE_CORE,
    "char_to_bytes_le",
    call_core_char_to_bytes_be,
    CHAR_TO_BYTES_BE_CORE,
    "char_to_bytes_be"
);
to_bytes_impl!(
    call_core_field_to_bytes_le,
    FIELD_TO_BYTES_LE_CORE,
    "field_to_bytes_le",
    call_core_field_to_bytes_be,
    FIELD_TO_BYTES_BE_CORE,
    "field_to_bytes_be"
);
to_bytes_impl!(
    call_core_group_to_bytes_le,
    GROUP_TO_BYTES_LE_CORE,
    "group_to_bytes_le",
    call_core_group_to_bytes_be,
    GROUP_TO_BYTES_BE_CORE,
    "group_to_bytes_be"
);
to_bytes_impl!(
    call_core_i8_to_bytes_le,
    I8_TO_BYTES_LE_CORE,
    "i8_to_bytes_le",
    call_core_i8_to_bytes_be,
    I8_TO_BYTES_BE_CORE,
    "i8_to_bytes_be"
);
to_bytes_impl!(
    call_core_i16_to_bytes_le,
    I16_TO_BYTES_LE_CORE,
    "i16_to_bytes_le",
    call_core_i16_to_bytes_be,
    I16_TO_BYTES_BE_CORE,
    "i16_to_bytes_be"
);
to_bytes_impl!(
    call_core_i32_to_bytes_le,
    I32_TO_BYTES_LE_CORE,
    "i32_to_bytes_le",
    call_core_i32_to_bytes_be,
    I32_TO_BYTES_BE_CORE,
    "i32_to_bytes_be"
);
to_bytes_impl!(
    call_core_i64_to_bytes_le,
    I64_TO_BYTES_LE_CORE,
    "i64_to_bytes_le",
    call_core_i64_to_bytes_be,
    I64_TO_BYTES_BE_CORE,
    "i64_to_bytes_be"
);
to_bytes_impl!(
    call_core_i128_to_bytes_le,
    I128_TO_BYTES_LE_CORE,
    "i128_to_bytes_le",
    call_core_i128_to_bytes_be,
    I128_TO_BYTES_BE_CORE,
    "i128_to_bytes_be"
);
to_bytes_impl!(
    call_core_u8_to_bytes_le,
    U8_TO_BYTES_LE_CORE,
    "u8_to_bytes_le",
    call_core_u8_to_bytes_be,
    U8_TO_BYTES_BE_CORE,
    "u8_to_bytes_be"
);
to_bytes_impl!(
    call_core_u16_to_bytes_le,
    U16_TO_BYTES_LE_CORE,
    "u16_to_bytes_le",
    call_core_u16_to_bytes_be,
    U16_TO_BYTES_BE_CORE,
    "u16_to_bytes_be"
);
to_bytes_impl!(
    call_core_u32_to_bytes_le,
    U32_TO_BYTES_LE_CORE,
    "u32_to_bytes_le",
    call_core_u32_to_bytes_be,
    U32_TO_BYTES_BE_CORE,
    "u32_to_bytes_be"
);
to_bytes_impl!(
    call_core_u64_to_bytes_le,
    U64_TO_BYTES_LE_CORE,
    "u64_to_bytes_le",
    call_core_u64_to_bytes_be,
    U64_TO_BYTES_BE_CORE,
    "u64_to_bytes_be"
);
to_bytes_impl!(
    call_core_u128_to_bytes_le,
    U128_TO_BYTES_LE_CORE,
    "u128_to_bytes_le",
    call_core_u128_to_bytes_be,
    U128_TO_BYTES_BE_CORE,
    "u128_to_bytes_be"
);
