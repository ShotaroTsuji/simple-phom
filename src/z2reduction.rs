use z2vector::Z2Vector;

#[derive(Debug)]
pub struct Z2BoundaryMatrix {
    columns: Vec<Z2Vector>,
}

impl Z2BoundaryMatrix {
    pub fn new() -> Z2BoundaryMatrix {
        Z2BoundaryMatrix { columns: Vec::new() }
    }

    pub fn push(&mut self, col: Z2Vector) {
        self.columns.push(col);
    }
}
