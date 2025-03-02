use gidth::numbers::Integer;

#[test]
fn big_int_works() {
	let my_int = &Integer::new(699348) + &Integer::new(92385);
	println!("{}", my_int);
}
