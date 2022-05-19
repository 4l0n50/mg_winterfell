use super::FieldElement;
use core_utils::TableReader;
use winterfell::{Air, EvaluationFrame, Table};

/// Contains rows of the execution trace
#[derive(Debug, Clone)]
pub struct CairoCpuFrame<E: FieldElement> {
    table: Table<E>, // row-major indexing
}

// DEFAULT EVALUATION FRAME
// ================================================================================================

impl<E: FieldElement> EvaluationFrame<E> for CairoCpuFrame<E> {

    const FRAME_SHIFT: usize = 16;
    // CONSTRUCTORS
    // --------------------------------------------------------------------------------------------

    fn new<A: Air>(air: &A) -> Self {
        let num_cols = air.trace_layout().main_trace_width();
        let num_rows = Self::num_rows();
        CairoCpuFrame {
            table: Table::new(num_rows, num_cols),
        }
    }

    fn from_table(table: Table<E>) -> Self {
        Self { table }
    }

    // ROW MUTATORS
    // --------------------------------------------------------------------------------------------

    fn read_from<R: TableReader<E>>(&mut self, data: R, step: usize, blowup: usize) {
        let trace_len = data.num_rows();
        for (row, row_idx) in self.table.rows_mut().zip(Self::offsets().into_iter()) {
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

    fn to_table(&self) -> Table<E> {
        self.table.clone()
    }

    fn offsets() -> &'static [usize] {
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
    }
}
