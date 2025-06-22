extern crate core;

mod sequential;

use std::collections::HashMap;
use std::fs;
use std::io::{Write,BufWriter};
use std::sync::atomic::Ordering::SeqCst;
use sequential::dice::Dice;
use sequential::sequencer::Sequencer;



fn main() {
	let arr = vec![0, 1];
	let mut seq = Sequencer::<24>::create(arr.iter());
	let mut accum=HashMap::<i32,usize>::new();
	
	let mut result=[0usize;25];

 
	
	
	loop {
		let idx:i32=seq.aggregate(|arr|arr.iter().sum());
		
		if let Some(x)=accum.get_mut(&idx){
			*x+=1;
		}else { 
			accum.insert(idx,1);
		}
		
		if(seq.move_next()){
			break;
		}
	}


	for (i,n) in accum.iter() {
		result[*i as usize]=*n;
	}


	for (i,n) in result.iter().enumerate() {
		println!("{}: {}",i,*n);
	}
	
}
