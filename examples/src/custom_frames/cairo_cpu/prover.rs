// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{
    BaseElement, CairoCpuAir, FieldElement, ProofOptions, Prover, PublicInputs, Trace, TraceTable, TRACE_WIDTH,
};

// FIBONACCI PROVER
// ================================================================================================

pub struct CairoCpuProver {
    options: ProofOptions,
}

impl CairoCpuProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    /// Builds an execution trace for computing a Fibonacci sequence of the specified length such
    /// that each row advances the sequence by 1 term.
    pub fn build_trace(&self, sequence_length: usize) -> TraceTable<BaseElement> {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        let mut trace = TraceTable::new(TRACE_WIDTH, sequence_length);
        trace.update_row(0, &[BaseElement::ONE; 3]);
        for i in 0..sequence_length {
            trace.update_row(i, &[BaseElement::ONE; 3]);
        }

        trace
    }
}

impl Prover for CairoCpuProver {
    type BaseField = BaseElement;
    type Air = CairoCpuAir;
    type Trace = TraceTable<BaseElement>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> PublicInputs {
        let frame_length = 18;
        let frame_shift = 16;
        let last_step = ((trace.length() - frame_length)/frame_shift)*frame_shift + frame_length;
        PublicInputs{
            fp: trace.get(1, 2),
            ap: trace.get(1, 1),
            pc: trace.get(0, 2),
            next_fp: trace.get(1, last_step),
            next_ap: trace.get(1, last_step - 1),
            next_pc: trace.get(2, last_step - 2)
        }

    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
