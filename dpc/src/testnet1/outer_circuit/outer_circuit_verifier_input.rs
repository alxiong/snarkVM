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

use crate::testnet1::Testnet1Components;
use snarkvm_algorithms::{
    merkle_tree::MerkleTreeDigest,
    traits::{CommitmentScheme, EncryptionScheme, MerkleParameters, SignatureScheme, CRH},
};
use snarkvm_fields::{ConstraintFieldError, ToConstraintField};
use snarkvm_utilities::ToBytes;

#[derive(Derivative)]
#[derivative(Clone(bound = "C: Testnet1Components"))]
pub struct OuterCircuitVerifierInput<C: Testnet1Components> {
    // pub inner_snark_verifier_input: InnerCircuitVerifierInput<C>,
    // pub inner_circuit_id: <C::InnerCircuitIDCRH as CRH>::Output,
    // pub inner_snark_verifier_input: InnerCircuitVerifierInput<C>,
    // pub inner_circuit_id: <C::InnerCircuitIDCRH as CRH>::Output,
    pub program_verification_key_commitment: C::ProgramVerificationKeyCommitment,
    pub program_verification_key_crh: C::ProgramVerificationKeyCRH,
    // Program input commitment and local data root
    pub program_commitment: <C::ProgramVerificationKeyCommitment as CommitmentScheme>::Output,
    pub local_data_root: <C::LocalDataCRH as CRH>::Output,
}

impl<C: Testnet1Components> ToConstraintField<C::OuterScalarField> for OuterCircuitVerifierInput<C>
where
    <C::ProgramVerificationKeyCommitment as CommitmentScheme>::Parameters: ToConstraintField<C::OuterScalarField>,
    <C::ProgramVerificationKeyCommitment as CommitmentScheme>::Output: ToConstraintField<C::OuterScalarField>,
    <C::ProgramVerificationKeyCRH as CRH>::Parameters: ToConstraintField<C::OuterScalarField>,

    <C::InnerCircuitIDCRH as CRH>::Parameters: ToConstraintField<C::OuterScalarField>,
    <C::InnerCircuitIDCRH as CRH>::Output: ToConstraintField<C::OuterScalarField>,

    <C::AccountCommitment as CommitmentScheme>::Parameters: ToConstraintField<C::InnerScalarField>,
    <C::AccountCommitment as CommitmentScheme>::Output: ToConstraintField<C::InnerScalarField>,

    <C::AccountEncryption as EncryptionScheme>::Parameters: ToConstraintField<C::InnerScalarField>,

    <C::AccountSignature as SignatureScheme>::Parameters: ToConstraintField<C::InnerScalarField>,
    <C::AccountSignature as SignatureScheme>::PublicKey: ToConstraintField<C::InnerScalarField>,

    <C::RecordCommitment as CommitmentScheme>::Parameters: ToConstraintField<C::InnerScalarField>,
    <C::RecordCommitment as CommitmentScheme>::Output: ToConstraintField<C::InnerScalarField>,

    <C::EncryptedRecordCRH as CRH>::Parameters: ToConstraintField<C::InnerScalarField>,
    <C::EncryptedRecordCRH as CRH>::Output: ToConstraintField<C::InnerScalarField>,

    <C::SerialNumberNonceCRH as CRH>::Parameters: ToConstraintField<C::InnerScalarField>,

    <C::ProgramVerificationKeyCommitment as CommitmentScheme>::Parameters: ToConstraintField<C::InnerScalarField>,
    <C::ProgramVerificationKeyCommitment as CommitmentScheme>::Output: ToConstraintField<C::InnerScalarField>,

    <C::LocalDataCRH as CRH>::Parameters: ToConstraintField<C::InnerScalarField>,
    <C::LocalDataCRH as CRH>::Output: ToConstraintField<C::InnerScalarField>,

    <<C::MerkleParameters as MerkleParameters>::H as CRH>::Parameters: ToConstraintField<C::InnerScalarField>,
    MerkleTreeDigest<C::MerkleParameters>: ToConstraintField<C::InnerScalarField>,
{
    fn to_field_elements(&self) -> Result<Vec<C::OuterScalarField>, ConstraintFieldError> {
        let mut v = Vec::new();

        v.extend_from_slice(
            &self
                // .inner_snark_verifier_input
                // .system_parameters
                .program_verification_key_commitment
                .parameters()
                .to_field_elements()?,
        );
        v.extend_from_slice(&self
                // .inner_snark_verifier_input
                // .system_parameters
                .program_verification_key_crh
                .parameters()
                .to_field_elements()?);
        // v.extend_from_slice(
        //     &self
        //         .inner_snark_verifier_input
        //         .system_parameters
        //         .inner_circuit_id_crh
        //         .parameters()
        //         .to_field_elements()?,
        // );
        // // Convert inner snark verifier inputs into `OuterField` field elements

        // let inner_snark_field_elements = &self.inner_snark_verifier_input.to_field_elements()?;

        // for inner_snark_fe in inner_snark_field_elements {
        //     let inner_snark_fe_bytes = to_bytes_le![inner_snark_fe]?;
        //     v.extend_from_slice(&ToConstraintField::<C::OuterScalarField>::to_field_elements(
        //         inner_snark_fe_bytes.as_slice(),
        //     )?);
        // }

        // v.extend_from_slice(&self.inner_snark_verifier_input.program_commitment.to_field_elements()?);
        // v.extend_from_slice(&self.inner_circuit_id.to_field_elements()?);

        v.extend_from_slice(&ToConstraintField::<C::OuterScalarField>::to_field_elements(
            self.local_data_root.to_field_elements()?.to_bytes_le()?.as_slice(),
        )?);
        v.extend_from_slice(&self.program_commitment.to_field_elements()?);

        Ok(v)
    }
}
