use std::time::Instant;


fn main() {
	let start = Instant::now();
	let input = 0xa582041;
	let lut:[u32; 8] = [4, 1, 7, 8, 5, 2, 3, 9];

	for _ in 1..1000000 {
		let output = lut[input % 13 - 3];
		if output != 4 as u32 {
			println!("error. expected 4, but got {:?}", output);
			std::process::exit(1);
		}
	}

	println!("Time = {:.2?}", start.elapsed());
}

