// Copyright (C) 2019-2022 Aleo Systems Inc.
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

impl<N: Network> Zero for Scalar<N> {
    /// Returns the `0` element of the scalar.
    fn zero() -> Self {
        Self::new(N::Scalar::zero())
    }

    /// Returns `true` if the element is zero.
    fn is_zero(&self) -> bool {
        self.scalar.is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;

    type CurrentNetwork = Testnet3;

    const ITERATIONS: u64 = 100;

    #[test]
    fn test_zero() {
        let zero = Scalar::<CurrentNetwork>::zero();

        for bit in zero.to_bits_le().iter() {
            assert!(!bit)
        }
    }

    #[test]
    fn test_is_zero() {
        assert!(Scalar::<CurrentNetwork>::zero().is_zero());

        // Note: This test technically has a `1 / MODULUS` probability of being flaky.
        for _ in 0..ITERATIONS {
            let scalar: Scalar<CurrentNetwork> = Uniform::rand(&mut test_rng());
            assert!(!scalar.is_zero());
        }
    }
}
