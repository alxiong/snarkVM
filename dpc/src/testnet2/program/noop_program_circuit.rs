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

use crate::testnet2::Testnet2Components;
use snarkvm_algorithms::prelude::*;
use snarkvm_gadgets::prelude::*;
use snarkvm_r1cs::{Assignment, ConstraintSynthesizer, ConstraintSystem, SynthesisError};

/// Always-accept program
pub struct NoopCircuit<C: Testnet2Components> {
    /// Local data commitment parameters
    pub local_data_commitment_parameters: <C::LocalDataCommitment as CommitmentScheme>::Parameters,
    /// Commitment to the program input.
    pub local_data_root: Option<<C::LocalDataCRH as CRH>::Output>,
    /// Record position
    pub position: u8,
}

impl<C: Testnet2Components> NoopCircuit<C> {
    pub fn blank(local_data_commitment_parameters: &<C::LocalDataCommitment as CommitmentScheme>::Parameters) -> Self {
        Self {
            local_data_commitment_parameters: local_data_commitment_parameters.clone(),
            local_data_root: Some(<C::LocalDataCRH as CRH>::Output::default()),
            position: 0u8,
        }
    }

    pub fn new(
        local_data_commitment_parameters: &<C::LocalDataCommitment as CommitmentScheme>::Parameters,
        local_data_root: &<C::LocalDataCRH as CRH>::Output,
        position: u8,
    ) -> Self {
        Self {
            local_data_commitment_parameters: local_data_commitment_parameters.clone(),
            local_data_root: Some(local_data_root.clone()),
            position,
        }
    }
}

impl<C: Testnet2Components> ConstraintSynthesizer<C::InnerScalarField> for NoopCircuit<C> {
    fn generate_constraints<CS: ConstraintSystem<C::InnerScalarField>>(
        &self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let local_data_root = self.local_data_root.get_ref()?;
        let position = self.position;

        let _position = UInt8::alloc_input_vec_le(cs.ns(|| "Alloc position"), &[position])?;

        let _local_data_commitment_parameters_gadget =
            <C::LocalDataCommitmentGadget as CommitmentGadget<_, _>>::ParametersGadget::alloc_input(
                &mut cs.ns(|| "Declare local data commitment parameters"),
                || Ok(self.local_data_commitment_parameters.clone()),
            )?;

        // artificially drive up constraint number to around 2^15 by
        // adding some primtive gadget that always satisify without using any local data, for benchmarking purpose only
        let rng = &mut snarkvm_utilities::test_rng();
        for i in 0..24 {
            // similar to encryption/test.rs
            let encryption_scheme = <C::AccountEncryption as EncryptionScheme>::setup(rng);
            let private_key = <C::AccountEncryption as EncryptionScheme>::generate_private_key(&encryption_scheme, rng);
            let public_key =
                <C::AccountEncryption as EncryptionScheme>::generate_public_key(&encryption_scheme, &private_key)
                    .unwrap();
            let parameters_gadget =
                <C::AccountEncryptionGadget as EncryptionGadget<C::AccountEncryption, _>>::ParametersGadget::alloc(
                    &mut cs.ns(|| format!("parameters_gadget {}", i)),
                    || {
                        Ok(<C::AccountEncryption as EncryptionScheme>::parameters(
                            &encryption_scheme,
                        ))
                    },
                )
                .unwrap();
            let private_key_gadget =
                <C::AccountEncryptionGadget as EncryptionGadget<C::AccountEncryption, _>>::PrivateKeyGadget::alloc(
                    &mut cs.ns(|| format!("private_key_gadget {}", i)),
                    || Ok(&private_key),
                )
                .unwrap();
            let expected_public_key_gadget =
                <C::AccountEncryptionGadget as EncryptionGadget<C::AccountEncryption, _>>::PublicKeyGadget::alloc(
                    &mut cs.ns(|| format!("public_key_gadget {}", i)),
                    || Ok(&public_key),
                )
                .unwrap();
            let public_key_gadget =
                <C::AccountEncryptionGadget as EncryptionGadget<C::AccountEncryption, _>>::check_public_key_gadget(
                    &mut cs.ns(|| format!("public_key_gadget_evaluation {}", i)),
                    &parameters_gadget,
                    &private_key_gadget,
                )
                .unwrap();

            expected_public_key_gadget
                .enforce_equal(
                    cs.ns(|| format!("Check that declared and computed public keys are equal {}", i)),
                    &public_key_gadget,
                )
                .unwrap();
        }

        let _local_data_root_gadget = <C::LocalDataCRHGadget as CRHGadget<_, _>>::OutputGadget::alloc_input(
            &mut cs.ns(|| "Allocate local data root"),
            || Ok(local_data_root),
        )?;

        // more aribtrary increase in constraint, get closer to 2^15
        for i in cs.num_constraints()..(1 << 15) {
            if i % 2 == 1 {
                let f = C::InnerScalarField::from(i as u64);
                let f_alloc = AllocatedFp::from(&mut cs.ns(|| format!("dummy field {}", i)), &f);
                let a = FpGadget::from(f_alloc);
                let _b = a
                    .mul(&mut cs.ns(|| format!("dummy field mul result {}", i)), &a)
                    .unwrap();
            }
        }
        if cfg!(debug_assertions) {
            println!("total number of NoopCircuit constraints: {}", cs.num_constraints());
        }
        Ok(())
    }
}
