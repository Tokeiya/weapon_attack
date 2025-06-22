extern crate core;

mod sequential;

use std::collections::{HashMap, HashSet};
//use std::io::{Write,BufWriter};
use sequential::sequencer::Sequencer;



fn main() {

}

fn sort(source:&HashMap<i32,usize>)->Vec<usize>{
	let mut vec=(0..source.len()).collect::<Vec<usize>>();

	for (key,value) in source {
		vec[*key as usize]=*value;
	}
	
	vec
}
