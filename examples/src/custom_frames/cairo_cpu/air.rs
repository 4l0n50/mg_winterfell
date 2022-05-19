// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{BaseElement, CairoCpuFrame, FieldElement, ProofOptions, TRACE_WIDTH};
use crate::utils::are_equal;
use winterfell::{
    Air, AirContext, Assertion, ByteWriter, EvaluationFrame, DefaultEvaluationFrame, Serializable, TraceInfo, TransitionConstraintDegree,
};

pub struct PublicInputs {
    pub fp: BaseElement,
    pub ap: BaseElement,
    pub pc: BaseElement,
    pub next_fp: BaseElement,
    pub next_ap: BaseElement,
    pub next_pc: BaseElement
}

impl Serializable for PublicInputs {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write(self.fp);
        target.write(self.ap);
        target.write(self.pc);
    }
}

// FIBONACCI AIR
// ================================================================================================

pub struct CairoCpuAir {
    context: AirContext<BaseElement>
}

impl Air for CairoCpuAir {
    type BaseField = BaseElement;
    type PublicInputs = PublicInputs;
    type Frame<E: FieldElement> = CairoCpuFrame<E>;
    type AuxFrame<E: FieldElement> = DefaultEvaluationFrame<E>;

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        let degrees = vec![TransitionConstraintDegree::new(2); 31];
        assert_eq!(TRACE_WIDTH, trace_info.width());
        let context =
            AirContext::new(trace_info, degrees, 0, options);
        CairoCpuAir {
            context,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &Self::Frame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let one = E::ONE;
        let two = E::from(2u128);
        let two_to_15 = E::from((1 as u128) << 15);
        let two_to_16 = E::from((1 as u128) << 16);
        let two_to_32 = E::from((1 as u128) << 32);
        let two_to_48 = E::from((1 as u128) << 48);

        let row_0 = frame.row(0);
        let row_1 = frame.row(1);
        let row_2 = frame.row(2);
        let row_3 = frame.row(3);
        let row_4 = frame.row(4);
        let row_5 = frame.row(5);
        let row_6 = frame.row(6);
        let row_7 = frame.row(7);
        let row_8 = frame.row(8);
        let row_9 = frame.row(9);
        let row_10 = frame.row(10);
        let row_11 = frame.row(11);
        let row_12 = frame.row(12);
        let row_13 = frame.row(13);
        let row_14 = frame.row(14);
        let row_15 = frame.row(15);
        let row_16 = frame.row(16);
        let row_17 = frame.row(17);
        let row_18 = frame.row(18);

        // expected state width is nb_columns field elements
        debug_assert_eq!(TRACE_WIDTH, row_0.len());
        debug_assert_eq!(TRACE_WIDTH, row_1.len());
        debug_assert_eq!(TRACE_WIDTH, row_2.len());
        debug_assert_eq!(TRACE_WIDTH, row_3.len());
        debug_assert_eq!(TRACE_WIDTH, row_4.len());
        debug_assert_eq!(TRACE_WIDTH, row_5.len());
        debug_assert_eq!(TRACE_WIDTH, row_6.len());
        debug_assert_eq!(TRACE_WIDTH, row_7.len());
        debug_assert_eq!(TRACE_WIDTH, row_8.len());
        debug_assert_eq!(TRACE_WIDTH, row_9.len());
        debug_assert_eq!(TRACE_WIDTH, row_10.len());
        debug_assert_eq!(TRACE_WIDTH, row_11.len());
        debug_assert_eq!(TRACE_WIDTH, row_12.len());
        debug_assert_eq!(TRACE_WIDTH, row_13.len());
        debug_assert_eq!(TRACE_WIDTH, row_14.len());
        debug_assert_eq!(TRACE_WIDTH, row_15.len());
        debug_assert_eq!(TRACE_WIDTH, row_16.len());
        debug_assert_eq!(TRACE_WIDTH, row_17.len());
        debug_assert_eq!(TRACE_WIDTH, row_18.len());

        // Flag definitions
        let f_0 = row_0[0] - two*row_1[0];
        let f_1 = row_1[0] - two*row_2[0];
        let f_2 = row_2[0] - two*row_3[0];
        let f_3 = row_3[0] - two*row_4[0];
        let f_4 = row_4[0] - two*row_5[0];
        let f_5 = row_5[0] - two*row_6[0];
        let f_6 = row_6[0] - two*row_7[0];
        let f_7 = row_7[0] - two*row_8[0];
        let f_8 = row_8[0] - two*row_9[0];
        let f_9 = row_9[0] - two*row_10[0];
        let f_10 = row_10[0] - two*row_11[0];
        let f_11 = row_11[0] - two*row_12[0];
        let f_12 = row_12[0] - two*row_13[0];
        let f_13 = row_13[0] - two*row_14[0];
        let f_14 = row_14[0] - two*row_15[0];
        let f_15 = row_15[0] - two*row_16[0];

        let instruction_size = f_2 + one;

        // Instruction unpacking constraints
        result[0] = are_equal(row_1[2], row_0[1] + two_to_16*row_4[1] + row_8[1]*two_to_32 + row_0[0]*two_to_48); //c_inst

        result[1] = f_0 * (f_0 - one);
        result[2] = f_1 * (f_1 - one);
        result[3] = f_2 * (f_2 - one);
        result[4] = f_3 * (f_3 - one);
        result[5] = f_4 * (f_4 - one);
        result[6] = f_5 * (f_5 - one);
        result[7] = f_6 * (f_6 - one);
        result[8] = f_7 * (f_7 - one);
        result[9] = f_8 * (f_8 - one);
        result[10] = f_9 * (f_9 - one);
        result[11] = f_10 * (f_10 - one);
        result[12] = f_11 * (f_11 - one);
        result[13] = f_12 * (f_12 - one);
        result[14] = f_13 * (f_13 - one);
        result[15] = f_14 * (f_14 - one);
        result[16] = f_15 * (f_15 - one);

        result[17] = row_15[0];

        // Operand constraints
        result[18] = are_equal(row_4[2], f_0*row_2[1] + (one - f_0)*row_1[1] + row_0[1]);
        result[19] = are_equal(row_8[2], f_1*row_2[1] + (one - f_1)*row_1[1] + row_4[1]);
        result[20] = are_equal(row_12[2], f_2*row_0[2] + f_4*row_1[1] + f_3*row_2[1] + (one - f_2 - f_4 - f_3)*row_9[2] + row_8[1]);

        // ap and fp registers
        result[21] = are_equal(row_17[1], row_1[1] + f_10*row_7[1] + f_11 + f_12*two);
        result[22] = are_equal(row_18[1], f_13*row_5[2] + f_12*(row_1[1] + two) + (one - f_13 - f_12)*row_2[1]);

        // pc register
        result[23] = are_equal(row_5[1], f_9*row_5[2]);
        result[24] = are_equal(row_6[1], row_5[1]*row_7[1]);
        result[25] = (row_6[1] - f_9)*(row_16[2] - (row_0[2] + instruction_size));
        result[26] = row_5[1]*(row_16[2] - (row_0[2] + row_13[2])) + (one - f_9)*row_16[2] - ((one - f_7 - f_8 - f_9)*(row_0[2] + instruction_size) + f_7*row_7[1] + f_8*(row_0[2] + row_7[1]));

        // Opcodes and res
        result[27] = are_equal(row_3[1], row_9[2]*row_13[2]);
        result[28] = are_equal((one - f_9)*row_7[1], f_5*(row_9[2] + row_13[2]) + f_6*row_3[1] + (one - f_5 - f_6 - f_9)*row_13[2]);
        result[29] = f_12*(row_5[2] - row_2[1]);
        result[30] = f_12*(row_9[2] - (row_0[2] + instruction_size));
        result[31] = f_14*(row_5[2] - row_7[1]);

    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        vec![]
    }
}
