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

use crate::algorithms::CommitmentScheme;
use crate::curves::Field;
use crate::gadgets::r1cs::ConstraintSystem;
use crate::gadgets::utilities::alloc::AllocGadget;
use crate::gadgets::utilities::eq::{ConditionalEqGadget, EqGadget};
use crate::gadgets::utilities::select::CondSelectGadget;
use crate::gadgets::utilities::uint::UInt8;
use crate::gadgets::utilities::ToBytesGadget;
use snarkvm_r1cs::errors::SynthesisError;

use std::fmt::Debug;

pub trait CommitmentGadget<C: CommitmentScheme, F: Field> {
    type OutputGadget: ConditionalEqGadget<F>
        + CondSelectGadget<F>
        + EqGadget<F>
        + ToBytesGadget<F>
        + AllocGadget<C::Output, F>
        + Clone
        + Sized
        + Debug;
    type ParametersGadget: AllocGadget<C::Parameters, F> + Clone;
    type RandomnessGadget: AllocGadget<C::Randomness, F> + Clone;

    fn check_commitment_gadget<CS: ConstraintSystem<F>>(
        cs: CS,
        parameters: &Self::ParametersGadget,
        input: &[UInt8],
        r: &Self::RandomnessGadget,
    ) -> Result<Self::OutputGadget, SynthesisError>;
}
