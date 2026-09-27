use reda_unit::{num, Complex};

fn main() {
    let re = num!(3.);
    let im = num!(4.);

    let c = Complex::new(re, im);

    println!("Complex number: {}", c);

    let conj = c.conjugate();
    println!("Conjugate: {}", conj);

    let norm_sq = c.norm_sqr();
    println!("Norm squared: {}", norm_sq);
}
