extern crate simple_phom;

use simple_phom::simplex::Simplex;
use simple_phom::filteredcomplex::FilteredComplex;
use simple_phom::z2vector::Z2Vector;
use simple_phom::z2reduction::Z2BoundaryMatrix;

fn print_boundary(simplex: &Simplex) {
    print!("The boundary of simplex {} = {{ ", simplex);
    for t in simplex.boundary() {
        print!("{}, ", t);
    }
    println!("}}");
}

fn main() {
    println!("Hello, world!");

    let simplices = vec![
        vec![0],   vec![1],   vec![2],   vec![3],
	vec![0,1], vec![0,2], vec![1,3], vec![2,3], vec![1,2],
	vec![0,1,2]
    ];

    let mut filtcomp = FilteredComplex::new();
    let mut boundary_matrix = Z2BoundaryMatrix::new();

    for simp in simplices.into_iter() {
        let boundary = filtcomp.push(Simplex::from(simp)).unwrap();
	boundary_matrix.push(boundary);
    }

    for s in filtcomp.iter() {
        print_boundary(s);
    }

    println!("{:?}", boundary_matrix);

    println!("{}", Simplex::from(vec![0]) == Simplex::from(vec![0]));
    println!("{}", Simplex::from(vec![0]) == Simplex::from(vec![1]));
    println!("{}", Simplex::from(vec![0]) == Simplex::from(vec![0,1]));

    println!("{:?}", filtcomp.find_index(&Simplex::from(vec![0])));
    println!("{:?}", filtcomp.find_index(&Simplex::from(vec![2])));
    println!("{:?}", filtcomp.find_index(&Simplex::from(vec![0,1,2])));

    println!("simplex of index 4 in complex: {:?}", filtcomp.get(4));

    let v1 = Z2Vector::new();
    println!("v1 = {:?}", v1);

    let v2 = Z2Vector::from(vec![0, 1, 2]);
    println!("v2 = {:?}", v2);

    let v3 = Z2Vector::from(vec![1, 2, 3]);
    println!("v3 = {:?}", v3);

    println!("v1 + v2 = {:?}", v1.add(&v2));
    println!("v2 + v3 = {:?}", v2.add(&v3));
    println!("v2 + v2 = {:?}", v2.add(&v2));
}