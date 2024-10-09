use integers::factorization::get_factors;

mod integers;

fn main() {
    let factors = get_factors(625).unwrap();

    for f in factors {
        println!("{f}");
    }
}
