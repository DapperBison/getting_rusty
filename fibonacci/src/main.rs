fn main() {
	fn fib(n: u32) -> u32 {
		let mut a = 0;
		let mut b = 1;
		for _ in 0..n {
			let tempvalue = a +b;
			a = b;
			b = tempvalue;
		}
	a
	}
	for i in 0..10 {
		println!("F({}) = {}" , i, fib(i));
	}
}
