use std::marker::PhantomData;
use z2vector::Chain;
use z2vector::Z2Vector;

#[derive(Debug)]
pub struct Z2BoundaryMatrix<C>
where
    C: Z2Vector,
{
    columns: Vec<C>,
}

impl<C> Z2BoundaryMatrix<C>
where
    C: Z2Vector,
{
    pub fn new() -> Z2BoundaryMatrix<C> {
        Z2BoundaryMatrix {
            columns: Vec::new(),
        }
    }

    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    pub fn push(&mut self, col: C) {
        self.columns.push(col);
    }

    pub fn get_column(&self, index: usize) -> Option<&C> {
        self.columns.get(index)
    }

    pub fn lowest(&self, index: usize) -> Option<usize> {
        self.columns.get(index).unwrap().lowest()
    }
}

#[derive(Debug)]
pub struct Z2ReducedMatrix<C>
where
    C: Z2Vector,
{
    columns: Vec<C>,
}

impl<C> Z2ReducedMatrix<C>
where
    C: Z2Vector,
{
    pub fn ncols(&self) -> usize {
        self.columns.len()
    }

    pub fn get_column(&self, index: usize) -> Option<&C> {
        self.columns.get(index)
    }

    pub fn lowest(&self, index: usize) -> Option<usize> {
        self.columns.get(index).and_then(|c| c.lowest())
    }

    fn add_left_to_right(&mut self, j1: usize, j2: usize) {
        assert!(j1 < j2, "index j1 must be less than index j2");

        let new = {
            let left = self.columns.get(j1).unwrap();
            let right = self.columns.get(j2).unwrap();
            right.add(left)
        };

        let right = self.columns.get_mut(j2).unwrap();
        *right = new;
    }

    /// Searches for a column that has the same lowest to the column of the given index.
    fn position_of_same_lowest(&self, index: usize) -> Option<usize> {
        if let Some(low_i) = self.lowest(index) {
            for j in 0..index {
                match self.lowest(j) {
                    Some(low_j) if low_i == low_j => {
                        return Some(j);
                    }
                    _ => {}
                }
            }
        }
        None
    }

    /// Searches for a column that has the given lowest.
    fn position_by_lowest(&self, low: usize) -> Option<usize> {
        for k in low..self.ncols() {
            match self.lowest(k) {
                Some(low_k) if low_k == low => {
                    return Some(k);
                }
                _ => {}
            }
        }
        None
    }
}

pub fn reduce<C>(boundary_matrix: Z2BoundaryMatrix<C>) -> Z2ReducedMatrix<C>
where
    C: Z2Vector,
{
    let mut rmat = Z2ReducedMatrix {
        columns: boundary_matrix.columns,
    };
    for j in 0..rmat.ncols() {
        while let Some(k) = rmat.position_of_same_lowest(j) {
            rmat.add_left_to_right(k, j);
        }
    }
    rmat
}

#[derive(Debug)]
pub struct Persistence {
    dimension: usize,
    birth: usize,
    death: Option<usize>,
}

pub fn pairing<'a, C>(reduced_matrix: &'a Z2ReducedMatrix<C>) -> PersistencePairing<'a, C>
where
    C: 'a + Z2Vector + Chain,
{
    PersistencePairing {
        matrix: reduced_matrix,
        index: 0,
        phantom: PhantomData,
    }
}

pub struct PersistencePairing<'a, C>
where
    C: 'a + Z2Vector + Chain,
{
    matrix: &'a Z2ReducedMatrix<C>,
    index: usize, // index of column that will be processed in next calling
    phantom: PhantomData<&'a Z2ReducedMatrix<C>>,
}

impl<'a, C> Iterator for PersistencePairing<'a, C>
where
    C: 'a + Z2Vector + Chain,
{
    type Item = Persistence;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(chain) = self.matrix.get_column(self.index) {
            if chain.is_cycle() {
                let j = self.index;
                self.index += 1;
                match self.matrix.position_by_lowest(j) {
                    Some(k) => {
                        return Some(Persistence {
                            dimension: chain.chain_dim(),
                            birth: j,
                            death: Some(k),
                        });
                    }
                    None => {
                        return Some(Persistence {
                            dimension: chain.chain_dim(),
                            birth: j,
                            death: None,
                        });
                    }
                }
            }
            self.index += 1;
        }
        None
    }
}
