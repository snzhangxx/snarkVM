// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![forbid(unsafe_code)]

pub use snarkvm_circuit_environment as environment;
pub use snarkvm_circuit_types_address as address;
pub use snarkvm_circuit_types_boolean as boolean;
pub use snarkvm_circuit_types_field as field;
pub use snarkvm_circuit_types_group as group;
pub use snarkvm_circuit_types_integers as integers;
pub use snarkvm_circuit_types_scalar as scalar;
pub use snarkvm_circuit_types_string as string;

pub use address::Address;
pub use boolean::Boolean;
pub use environment::prelude::*;
pub use field::Field;
pub use group::Group;
pub use integers::{I128, I16, I32, I64, I8, U128, U16, U32, U64, U8};
pub use scalar::Scalar;
pub use string::StringType;

pub mod prelude {
    pub use super::*;
    pub use environment::prelude::*;
}
