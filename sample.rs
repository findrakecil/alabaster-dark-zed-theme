fn main() {
	let n = 5;

	if n < 0 {
		print!("{} is negative", n);
	} else {
		print!("{} is zero", n)
	}

	let big_n =
		if n < 10 && n > -10 {
			println!(", and is a small number, increase ten-fold");

			// This expression returns an `i32`.
			10 * n
		} else {
			println!(", and is a big number, halve the number");

			// This expression returns an `i32`.
			n / 2
			// TODO ^ Try suppressing this expression with a semicolon.
		};
	//   ^ Don't forget to put a semicolon here! All `let` bindings need it.

	println!("{} -> {}", n, bign_n);
}
