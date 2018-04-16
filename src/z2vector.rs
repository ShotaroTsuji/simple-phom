#[derive(Debug)]
pub struct Z2Vector {
    elements: Vec<usize>, /* elements must be sorted by descending order */
}

impl Z2Vector {
    pub fn zero() -> Z2Vector {
        Z2Vector {
	    elements: Vec::new(),
	}
    }

    pub fn lowest(&self) -> Option<usize> {
        if self.elements.len() == 0 {
	    None
	} else {
	    Some(self.elements[0])
	}
    }

    pub fn add(&self, right: &Z2Vector) -> Z2Vector {
        let mut left_slice = &self.elements[..];
        let mut right_slice = &right.elements[..];
	let mut result = Vec::new();

	while left_slice.len() > 0 || right_slice.len() > 0 {
	    if left_slice.len() == 0 {
	        result.push(right_slice[0]);
		right_slice = &right_slice[1..];
	    } else if right_slice.len() == 0 {
	        result.push(left_slice[0]);
		left_slice = &left_slice[1..];
	    } else if left_slice[0] > right_slice[0] {
	        result.push(left_slice[0]);
		left_slice = &left_slice[1..];
	    } else if left_slice[0] < right_slice[0] {
	        result.push(right_slice[0]);
		right_slice = &right_slice[1..];
	    } else if left_slice[0] == right_slice[0] {
		left_slice = &left_slice[1..];
		right_slice = &right_slice[1..];
	    } else {
	        panic!("UNREACHABLE!");
	    }
	}

        Z2Vector::from(result)
    }
}

impl From<Vec<usize>> for Z2Vector {
    fn from(vec_: Vec<usize>) -> Z2Vector {
        let mut vec = vec_;
        vec.sort_by(|a, b| b.cmp(a));
        Z2Vector {
	    elements: vec,
	}
    }
}
