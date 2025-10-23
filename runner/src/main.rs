use rand::prelude::*;
use rand_distr::Distribution;
use std::ops::DerefMut;

mod account;
mod argument_error;
mod invalid_operation_error;
mod investment;

fn main() {
	let mut rnd = rand::thread_rng();
	let dist = rand_distr::Normal::new(0.0, 1.0).unwrap();

	let a = dist.sample(&mut rnd);
	println!("{}", a);
}

fn foo<D: Distribution<f64>, R: Rng>(dist: &D, rng: &mut R) {
	let a = dist.sample(rng);
	println!("{}", a);
}
