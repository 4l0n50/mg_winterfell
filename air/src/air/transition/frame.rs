// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{super::Air, FieldElement, Vec};
use crate::Table;
use utils::TableReader;

/// A set of execution trace rows required for evaluation of transition constraints.
/// It is passed in as one of the parameters into
/// [Air::evaluate_transition()](crate::Air::evaluate_transition) function.
pub trait EvaluationFrame<E: FieldElement> {
    
    /// Creates an empty frame
    fn new<A: Air>(air: &A) -> Self;

    /// Creates a new frame instantiated from the provided row-major table
    fn from_table(table: Table<E>) -> Self;

    /// Convert frame to a row-major table
    fn to_table(&self) -> Table<E>;

    /// Reads selected trace rows from the supplied data into the frame
    fn read_from<R: TableReader<E>>(&mut self, data: R, step: usize, blowup: usize);

    /// Returns None() (infinity) when all the cells in a row are active and Some(x) when
    /// the length of the row is x.
    fn row_len(offset: usize) -> Option<usize>;

    // TODO: It should be checked that is_active_row(offset, column_index) => row_len(offset) > 0 
    fn is_active_cell(offset: usize, column_index: usize) -> bool;

    /// Returns the specified frame row
    /// TODO: Maybe this method is not needed
    fn row<'a>(&'a self, index: usize) -> &'a [E];

    /// Returns the row evaluated at g^0*z in the OOD frame
    fn main_row<'a>(&'a self) -> &'a [E];

    /// Returns the number of frame rows
    fn num_rows() -> usize {
        Self::offsets().len()
    }

    /// Returns the offsets that make up a frame
    fn offsets() -> &'static [usize];

    /// Returns the amount of trace rows that the evaluation frame should shift across
    /// consecutive constraint evaluation steps
    fn shift() -> usize ;
}

/// Contains rows of the execution trace
#[derive(Debug, Clone)]
pub struct DefaultEvaluationFrame<E: FieldElement> {
    current: Vec<E>,
    next: Vec<E>,
}

// DEFAULT EVALUATION FRAME
// ================================================================================================

impl<E: FieldElement> DefaultEvaluationFrame<E> {
    const FRAME_OFFSETS: [usize; 2] = [0, 1];
    const FRAME_SHIFT: usize = 1;

    /// Returns a reference to the current row.
    #[inline(always)]
    pub fn current(&self) -> &[E] {
        &self.current
    }

    /// Returns a mutable reference to the current row.
    #[inline(always)]
    pub fn current_mut(&mut self) -> &mut [E] {
        &mut self.current
    }

    /// Returns a reference to the next row.
    #[inline(always)]
    pub fn next(&self) -> &[E] {
        &self.next
    }

    /// Returns a mutable reference to the next row.
    #[inline(always)]
    pub fn next_mut(&mut self) -> &mut [E] {
        &mut self.next
    }
}

impl<E: FieldElement> EvaluationFrame<E> for DefaultEvaluationFrame<E> {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    /// Returns a new evaluation frame instantiated with the specified number of registers.
    ///
    /// # Panics
    /// Panics if `num_registers` is zero.
    fn new<A: Air>(air: &A) -> Self {
        let num_cols = air.trace_layout().main_trace_width();
        DefaultEvaluationFrame {
            current: E::zeroed_vector(num_cols),
            next: E::zeroed_vector(num_cols),
        }
    }

    fn from_table(table: Table<E>) -> Self {
        //TODO add assertions?
        Self {
            current: table.get_row(0).to_vec(),
            next: table.get_row(1).to_vec()
        }
    }

    // ROW MUTATORS
    // --------------------------------------------------------------------------------------------

    fn read_from<R: TableReader<E>>(&mut self, data: R, step: usize, blowup: usize) {
        let trace_len = data.num_rows();
        for col_idx in 0..data.num_cols() {
            self.current[col_idx] = data.get(col_idx, step);
            self.next[col_idx] = data.get(col_idx, (step + blowup) % trace_len);
        }
    }

    fn row_len(offset: usize) -> Option<usize> {
        if offset < 2 {
            None
        }
        else {
            Some(0)
        }
    }

    fn is_active_cell(offset: usize, _column_index: usize) -> bool {
        offset < 2
    }

    // ROW ACCESSORS
    // --------------------------------------------------------------------------------------------

    fn row<'a>(&'a self, row_idx: usize) -> &'a [E] {
        match row_idx {
            0 => &self.current,
            1 => &self.next,
            _ => panic!("Row index must be 0 or 1")
        }
    }

    fn main_row<'a>(&'a self) -> &'a [E] {
        &self.current
    }

    fn to_table(&self) -> Table<E> {
        Table::from_rows(vec![self.current.clone(), self.next.clone()])
    }

    fn offsets() -> &'static [usize] {
        &Self::FRAME_OFFSETS
    }

    fn shift() -> usize {
        Self::FRAME_SHIFT
    }
}


// CUSTOM EVALUATION FRAME
// ================================================================================================


/// Contains rows of the execution trace
#[derive(Debug, Clone)]
pub struct CustomEvaluationFrame<E: FieldElement> {
    table: Table<E>, // row-major indexing
}


impl<E: FieldElement> CustomEvaluationFrame<E> {
    const FRAME_OFFSETS: [usize; 2] = [0, 1];
    const FRAME_SHIFT: usize = 1;
}

impl<E: FieldElement> EvaluationFrame<E> for CustomEvaluationFrame<E> {
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    
    fn new<A: Air>(air: &A) -> Self {
        let num_cols = air.trace_layout().main_trace_width();
        let num_rows = Self::num_rows();
        CustomEvaluationFrame {
            table: Table::new(num_rows, num_cols),
        }
    }

    fn from_table(table: Table<E>) -> Self {
        Self { table }
    }

    fn is_active_cell(offset: usize, _column_index: usize) -> bool {
        // TODO: Sequential search? And what about the other loop for the columns?
        for index in Self::FRAME_OFFSETS.iter() {
            if *index == offset {return true}
        }
        false
    }

    fn row_len(offset: usize) -> Option<usize> {
        if Self::offsets().contains(&offset) {
            Some(0)
        }
        else {
            None
        }
    }

    // ROW MUTATORS
    // --------------------------------------------------------------------------------------------

    fn read_from<R: TableReader<E>>(&mut self, data: R, step: usize, blowup: usize) {
        let trace_len = data.num_rows();
        for (row, row_idx) in self.table.rows_mut().zip(Self::FRAME_OFFSETS.into_iter()) {
            for col_idx in 0..data.num_cols() {
                row[col_idx] = data.get(col_idx, (step + row_idx * blowup) % trace_len);
            }
        }
    }

    // ROW ACCESSORS
    // --------------------------------------------------------------------------------------------

    fn row<'a>(&'a self, row_idx: usize) -> &'a [E] {
        &self.table.get_row(row_idx)
    }

    fn main_row<'a>(&'a self) -> &'a [E] {
        &self.table.get_row(0)
    }

    fn to_table(&self) -> Table<E> {
        self.table.clone()
    }

    fn offsets() -> &'static [usize] {
        &Self::FRAME_OFFSETS
    }

    fn shift() -> usize {
        Self::FRAME_SHIFT
    }
}
