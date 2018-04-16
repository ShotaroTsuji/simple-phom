use z2vector::Z2Vector;

#[derive(Debug)]
pub struct Z2BoundaryMatrix {
    columns: Vec<Z2Vector>,
}

impl Z2BoundaryMatrix {
    pub fn new() -> Z2BoundaryMatrix {
        Z2BoundaryMatrix { columns: Vec::new() }
    }

    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    pub fn push(&mut self, col: Z2Vector) {
        self.columns.push(col);
    }

    pub fn column(&self, index: usize) -> &Z2Vector {
        self.columns.get(index).unwrap()
    }

    pub fn lowest(&self, index: usize) -> Option<usize> {
        self.columns.get(index).unwrap().lowest()
    }
}

#[derive(Debug)]
pub struct Z2ReducedMatrix {
    columns: Vec<Z2Vector>,
}

impl Z2ReducedMatrix {
    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    pub fn column(&self, index: usize) -> &Z2Vector {
        self.columns.get(index).unwrap()
    }

    pub fn lowest(&self, index: usize) -> Option<usize> {
        self.columns.get(index).unwrap().lowest()
    }

    fn add_left_to_right(&mut self, j1: usize, j2: usize) {
        assert!(j1 < j2, "index j1 must be less than index j2");

	let new = {
	    let left  = self.columns.get(j1).unwrap();
	    let right = self.columns.get(j2).unwrap();
	    right.add(left)
	};

	let right = self.columns.get_mut(j2).unwrap();
	*right = new;
    }

    /* return the index of the column that has the same lowest value of
       the given index */
    fn find_same_lowest(&self, index: usize) -> Option<usize> {
        if let Some(low_i) = self.lowest(index) {
            for j in 0..index {
	        match self.lowest(j) {
		    Some(low_j) if low_i == low_j => { return Some(j); },
		    _ => {},
		}
	    }
	}
	None
    }
}

pub fn reduce(boundary_matrix: Z2BoundaryMatrix) -> Z2ReducedMatrix {
    let mut rmat = Z2ReducedMatrix {
        columns: boundary_matrix.columns,
    };
    for j in 0..rmat.ncols() {
        while let Some(k) = rmat.find_same_lowest(j) {
            rmat.add_left_to_right(k, j);
	}
    }
    rmat
}

