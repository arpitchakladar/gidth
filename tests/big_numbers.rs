use gidth::numbers::Integer;

#[test]
fn big_int_sum() {
	let x = Integer::new(vec![234, 21, 84]);
	let y = Integer::new(vec![137, 45]);
	let my_int = &x + &y;
	println!("{}", my_int);
}
