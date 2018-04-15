#[derive(Debug)]
pub struct Z2Vector {
    elements: Vec<usize>,
    lowest  : Option<usize>,
}

impl Z2Vector {
    pub fn new() -> Z2Vector {
        Z2Vector {
	    elements: Vec::new(),
	    lowest  : None,
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
	    } else if left_slice[0] < right_slice[0] {
	        result.push(left_slice[0]);
		left_slice = &left_slice[1..];
	    } else if left_slice[0] > right_slice[0] {
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
        vec.sort();
        let low =
	    if vec.len() == 0 {
	        None
	    } else { 
	        Some(vec[vec.len()-1])
	    };
        Z2Vector {
	    elements: vec,
	    lowest  : low,
	}
    }
}
