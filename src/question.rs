#[derive(Debug)]
struct Point {
	x: f64,
	y: f64,
}

fn main() {
	println!("Hello world");

	let mut a_option = Some(Point { x: 10.0, y: 32.0 });
	let mut b_option = Some(Point { x: 12.0, y: 2.0 });

	println!("a_option: {:?}", a_option);
	println!("b_option: {:?}", b_option);

	// let mut q = (a_option, b_option);

	let (a_option, b_option) =
		match (a_option, b_option) {
			(Some(ref mut a), Some(ref mut b)) => {
				a.x += b.x;
				a.y += b.y;

				(Some(a), Some(b))
			},
			(a, b) => (a, b)
		};

	// a_option = q.0;
	// b_option = q.1;

	println!("a_option: {:?}", a_option);
	println!("b_option: {:?}", b_option);
}
