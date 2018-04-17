use std::fmt;

pub trait Z2Vector<T = Self> {
    fn lowest(&self) -> Option<usize>;
    fn add(&self, right: &T) -> T;
}

pub trait Chain {
    fn is_cycle(&self) -> bool;
    fn chain_dim(&self) -> usize;
}

#[derive(Debug)]
pub struct Z2VectorRaw {
    elements: Vec<usize>, /* elements must be sorted by descending order */
}

impl Z2VectorRaw {
    pub fn zero() -> Z2VectorRaw {
        Z2VectorRaw {
            elements: Vec::new(),
        }
    }
}

impl Z2Vector for Z2VectorRaw {
    fn lowest(&self) -> Option<usize> {
        if self.elements.len() == 0 {
            None
        } else {
            Some(self.elements[0])
        }
    }

    fn add(&self, right: &Z2VectorRaw) -> Z2VectorRaw {
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

        Z2VectorRaw::from(result)
    }
}

impl From<Vec<usize>> for Z2VectorRaw {
    fn from(vec_: Vec<usize>) -> Z2VectorRaw {
        let mut vec = vec_;
        vec.sort_by(|a, b| b.cmp(a));
        Z2VectorRaw { elements: vec }
    }
}

impl fmt::Display for Z2VectorRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ref v = &self.elements;
        write!(f, "{{")?;
        for x in v.iter() {
            write!(f, "{}, ", x)?;
        }
        write!(f, "}}")
    }
}

#[derive(Debug)]
pub struct Z2Boundary {
    vector: Z2VectorRaw,
    chain_dim: usize, // dimension of the preimage chain
}

impl Z2Boundary {
    pub fn new(vec: Z2VectorRaw, dim: usize) -> Z2Boundary {
        Z2Boundary {
            vector: vec,
            chain_dim: dim,
        }
    }
}

impl Chain for Z2Boundary {
    fn chain_dim(&self) -> usize {
        self.chain_dim
    }
    fn is_cycle(&self) -> bool {
        self.vector.lowest().is_none()
    }
}

impl Z2Vector for Z2Boundary {
    fn lowest(&self) -> Option<usize> {
        self.vector.lowest()
    }

    fn add(&self, right: &Z2Boundary) -> Z2Boundary {
        Z2Boundary {
            vector: self.vector.add(&right.vector),
            chain_dim: self.chain_dim,
        }
    }
}

impl fmt::Display for Z2Boundary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, dim={})", self.vector, self.chain_dim)
    }
}
