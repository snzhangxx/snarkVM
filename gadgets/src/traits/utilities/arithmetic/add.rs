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

use crate::curves::{Field, PrimeField};
use crate::gadgets::r1cs::ConstraintSystem;
use crate::gadgets::utilities::uint::{UInt, UInt128, UInt16, UInt32, UInt64, UInt8};
use snarkvm_r1cs::errors::SynthesisError;

/// Returns addition of `self` + `other` in the constraint system.
pub trait Add<F: Field, Rhs = Self>
where
    Self: std::marker::Sized,
{
    type ErrorType;

    fn add<CS: ConstraintSystem<F>>(&self, cs: CS, other: &Self) -> Result<Self, Self::ErrorType>;
}

// Implement unsigned integers
macro_rules! add_uint_impl {
    ($($gadget: ident),*) => ($(
        impl<F: Field + PrimeField> Add<F> for $gadget {
            type ErrorType = SynthesisError;

            fn add<CS: ConstraintSystem<F>>(
                &self,
                cs: CS,
                other: &Self
            ) -> Result<Self, Self::ErrorType> {
                <$gadget as UInt>::addmany(cs, &[self.clone(), other.clone()])
            }
        }
    )*)
}

add_uint_impl!(UInt8, UInt16, UInt32, UInt64, UInt128);
