use symbol::Symbol;

use crate::model::model_check;

pub mod connectives;
pub mod model;
pub mod sentence;
pub mod symbol;

fn harry() {
	let rain = Symbol("rain");
	let hagrid = Symbol("hagrid");
	let dumbledore = Symbol("dumbledore");

	let knowledge = and!(
		implication!(not!(rain), hagrid),
		or!(hagrid, dumbledore),
		not!(and!(hagrid, dumbledore)),
		dumbledore
	);

	println!("Result {}", model_check(&knowledge, &rain))
}

fn main() {
	harry();
}
