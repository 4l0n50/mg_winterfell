// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{Example, ExampleOptions};
use log::debug;
use std::time::Instant;
use winterfell::{
    math::{fields::f128::BaseElement, log2, FieldElement},
    ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};

mod air;
use air::{CairoCpuAir, PublicInputs};

mod prover;
use prover::CairoCpuProver;

mod frame;
use frame::CairoCpuFrame;

#[cfg(test)]
mod tests;

// CONSTANTS
// ================================================================================================

const TRACE_WIDTH: usize = 3;

// FIBONACCI EXAMPLE
// ================================================================================================

pub fn get_example(options: ExampleOptions, sequence_length: usize) -> Box<dyn Example> {
    Box::new(CairoCpuExample::new(
        sequence_length,
        options.to_proof_options(28, 8),
    ))
}

pub struct CairoCpuExample {
    options: ProofOptions,
    sequence_length: usize
}

impl CairoCpuExample {
    pub fn new(sequence_length: usize, options: ProofOptions) -> CairoCpuExample {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        CairoCpuExample {
            options,
            sequence_length
        }
    }
}

// EXAMPLE IMPLEMENTATION
// ================================================================================================

impl Example for CairoCpuExample {
    fn prove(&self) -> StarkProof {
        debug!(
            "Generating proof for computing CairoCpu sequence (2 terms per step) up to {}th term\n\
            ---------------------",
            self.sequence_length
        );

        // create a prover
        let prover = CairoCpuProver::new(self.options.clone());

        // generate execution trace
        let now = Instant::now();
        let trace = prover.build_trace(self.sequence_length);

        let trace_width = trace.width();
        let trace_length = trace.length();
        debug!(
            "Generated execution trace of {} registers and 2^{} steps in {} ms",
            trace_width,
            log2(trace_length),
            now.elapsed().as_millis()
        );

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<CairoCpuAir>(
            proof,
            PublicInputs{ 
                ap: BaseElement::ONE,
                fp: BaseElement::ONE,
                pc: BaseElement::ONE,
                next_ap: BaseElement::ONE,
                next_fp: BaseElement::ONE,
                next_pc: BaseElement::ONE
            })
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        // TODO really?
        winterfell::verify::<CairoCpuAir>(proof, PublicInputs{ 
            ap: BaseElement::ZERO,
            fp: BaseElement::ZERO,
            pc: BaseElement::ZERO,
            next_ap: BaseElement::ZERO,
            next_fp: BaseElement::ZERO,
            next_pc: BaseElement::ZERO
        })
    }
}
