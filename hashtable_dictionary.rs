use std::time::Instant;
use std::collections::HashMap;


fn main() {
	let start = Instant::now();
	let input = 0xa582041;
	let outputs: HashMap<u32, u32> = HashMap::from([
		(0xa582041, 4),
		(0xa592041, 8),
		(0xa5a2041, 3),
		(0xa582042, 1),
		(0xa592042, 5),
		(0xa5a2042, 9),
		(0xa582043, 7),
		(0xa592043, 2),
	]);


	for _ in 1..1000000 {
		let output: u32 = outputs[&input];
		if output != 4 as u32 {
			println!("error. expected 4, but got {:?}", output);
			std::process::exit(1);
		}
	}
	
	println!("Time = {:.2?}", start.elapsed());
}

