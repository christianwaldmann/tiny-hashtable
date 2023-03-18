use std::time::Instant;


fn main() {
	let start = Instant::now();
	let input = 0xa582041;

	for _ in 1..1000000 {
		let shift: u32 = u32::wrapping_mul(input, 0x517f6e5d) >> 28;
		let output = (0x2f123 >> shift) & 0b1111;
		if output != 4 as u32 {
			println!("error. expected 4, but got {:?}", output);
			std::process::exit(1);
		}
	}
	
	println!("Time = {:.2?}", start.elapsed());
}

