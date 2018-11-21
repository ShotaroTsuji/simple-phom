use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Simplex {
    vertices: Vec<usize>,
}

impl Simplex {
    pub fn dimension(&self) -> usize {
        self.vertices.len() - 1
    }

    pub fn boundary(&self) -> BoundaryIter {
        BoundaryIter {
            vertices: self.vertices.as_ref(),
            index: 0,
            phantom: PhantomData,
        }
    }
}

impl From<Vec<usize>> for Simplex {
    fn from(vec_: Vec<usize>) -> Simplex {
        let mut vec = vec_;
        assert!(vec.len() > 0, "vector has no element");
        vec.sort();
        Simplex { vertices: vec }
    }
}

pub struct BoundaryIter<'a> {
    vertices: &'a [usize],
    index: usize,
    phantom: PhantomData<&'a usize>,
}

impl<'a> Iterator for BoundaryIter<'a> {
    type Item = Simplex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vertices.len() == 1 {
            None
        } else if self.index == self.vertices.len() {
            None
        } else {
            let head = self.vertices.iter().take(self.index);
            let tail = self.vertices.iter().skip(self.index + 1);
            let vec: Vec<usize> = head.chain(tail).map(|&x| x).collect();
            self.index = self.index + 1;
            Some(Simplex::from(vec))
        }
    }
}

impl fmt::Display for Simplex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ref v = &self.vertices;
        write!(f, "|")?;
        for x in v.iter() {
            write!(f, "{}", x)?;
        }
        write!(f, "|")
    }
}

impl PartialEq for Simplex {
    fn eq(&self, other: &Simplex) -> bool {
        if self.vertices.len() != other.vertices.len() {
            return false;
        }

        let iter = self.vertices.iter().zip(other.vertices.iter());
        for (x, y) in iter {
            if *x != *y {
                return false;
            }
        }
        true
    }
}
