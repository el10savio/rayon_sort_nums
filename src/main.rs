use rayon::prelude::*;

fn main() {
    let mut vec = vec![-1, -6, 5, 11, 0, 0, -1];

    sort(&mut vec);

    println!("{:?}", vec)
}

fn sort(vec: &mut Vec<i32>) {
    vec.par_sort_unstable();
}
