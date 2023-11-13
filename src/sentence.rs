use crate::model::Model;
use crate::symbol::SymbolName;

pub trait Sentence {
	fn symbols(&self) -> Vec<SymbolName>;
	fn evaluate(&self, model: &Model) -> bool;
}

#[macro_export]
macro_rules! merge_symbols {
	($x:expr) => {{
		let mut symbols: Vec<$crate::symbol::SymbolName> = vec![];

		for sentence in $x {
			for symbol in sentence.symbols() {
				if !symbols.contains(&symbol) {
					symbols.push(symbol);
				}
			}
		}

		symbols
	}};
	($($x:expr),+ $(,)?) => (merge_symbols!([$($x),*]));
}
