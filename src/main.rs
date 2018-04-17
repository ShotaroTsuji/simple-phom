extern crate simple_phom;

use simple_phom::filteredcomplex::FilteredComplex;
use simple_phom::simplex::Simplex;
use simple_phom::z2reduction;
use simple_phom::z2reduction::Z2BoundaryMatrix;

fn print_boundary(simplex: &Simplex) {
    print!("The boundary of simplex {} = {{ ", simplex);
    for t in simplex.boundary() {
        print!("{}, ", t);
    }
    println!("}}");
}

fn main() {
    /*
    println!("{}", Simplex::from(vec![0]) == Simplex::from(vec![0]));
    println!("{}", Simplex::from(vec![0]) == Simplex::from(vec![1]));
    println!("{}", Simplex::from(vec![0]) == Simplex::from(vec![0,1]));

    println!("{:?}", filtcomp.find_index(&Simplex::from(vec![0])));
    println!("{:?}", filtcomp.find_index(&Simplex::from(vec![2])));
    println!("{:?}", filtcomp.find_index(&Simplex::from(vec![0,1,2])));

    println!("simplex of index 4 in complex: {:?}", filtcomp.get(4));

    let v1 = Z2VectorRaw::zero();
    println!("v1 = {:?}", v1);
    println!("lowest = {:?}", v1.lowest());

    let v2 = Z2VectorRaw::from(vec![0, 1, 2]);
    println!("v2 = {:?}", v2);
    println!("lowest = {:?}", v2.lowest());

    let v3 = Z2VectorRaw::from(vec![1, 2, 3]);
    println!("v3 = {:?}", v3);
    println!("lowest = {:?}", v3.lowest());

    println!("v1 + v2 = {:?}", v1.add(&v2));
    println!("v2 + v3 = {:?}", v2.add(&v3));
    println!("v2 + v2 = {:?}", v2.add(&v2));
*/

    let mut filtcomp = FilteredComplex::new();
    let mut boundary_matrix = Z2BoundaryMatrix::new();

    let simplices = vec![
        vec![0],
        vec![1],
        vec![2],
        vec![3],
        vec![0, 1],
        vec![0, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2],
        vec![0, 1, 2],
    ];

    for simp in simplices.into_iter() {
        let boundary = filtcomp.push(Simplex::from(simp)).unwrap();
        boundary_matrix.push(boundary);
    }

    for s in filtcomp.iter() {
        print_boundary(s);
    }

    println!("Boundary Matrix");
    for i in 0..boundary_matrix.ncols() {
        println!("column[{}] = {}", i, boundary_matrix.column(i));
    }

    println!("Reduce the boundary matrix");
    let reduced_matrix = z2reduction::reduce(boundary_matrix);
    for i in 0..reduced_matrix.ncols() {
        println!("column[{}] = {}", i, reduced_matrix.column(i));
    }

    let pair_iter = z2reduction::pairing(&reduced_matrix);

    for pair in pair_iter {
        println!("{:?}", pair);
    }
}
