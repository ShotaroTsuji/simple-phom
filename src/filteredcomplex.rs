use simplex::Simplex;
use std::error;
use std::fmt;
use std::marker::PhantomData;
use z2vector::Z2Boundary;
use z2vector::Z2VectorRaw;

#[derive(Debug)]
pub struct FilterError;

impl fmt::Display for FilterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilterError of Filtered Complex")
    }
}

impl error::Error for FilterError {
    fn description(&self) -> &str {
        "The complex after inserted the simplex is NOT a simplicial complex"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct FilteredComplex {
    simplices: Vec<Simplex>,
}

impl FilteredComplex {
    pub fn new() -> FilteredComplex {
        FilteredComplex {
            simplices: Vec::new(),
        }
    }

    pub fn push_unchecked(&mut self, simplex: Simplex) {
        self.simplices.push(simplex);
    }

    pub fn push_raw(&mut self, simplex: Simplex) -> Result<Z2VectorRaw, FilterError> {
        let mut indices = Vec::new();
        for b in simplex.boundary() {
            match self.find_index(&b) {
                Some(index) => indices.push(index),
                None => {
                    return Err(FilterError);
                }
            }
        }
        let boundary = Z2VectorRaw::from(indices);
        self.simplices.push(simplex);
        Ok(boundary)
    }

    pub fn push(&mut self, simplex: Simplex) -> Result<Z2Boundary, FilterError> {
        let dim = simplex.dimension();
        self.push_raw(simplex).map(|v| Z2Boundary::new(v, dim))
    }

    pub fn find_index(&self, simplex: &Simplex) -> Option<usize> {
        self.simplices
            .iter()
            .enumerate()
            .find(|pair| *pair.1 == *simplex)
            .map(|pair| pair.0)
    }

    pub fn len(&self) -> usize {
        self.simplices.len()
    }

    pub fn get(&self, index: usize) -> Option<&Simplex> {
        self.simplices.get(index)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            simplices: self.simplices.as_ref(),
            index: 0,
            phantom: PhantomData,
        }
    }
}

pub struct Iter<'a> {
    simplices: &'a [Simplex],
    index: usize,
    phantom: PhantomData<&'a Simplex>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.simplices.len() {
            let r = Some(&self.simplices[self.index]);
            self.index = self.index + 1;
            r
        } else {
            None
        }
    }
}
