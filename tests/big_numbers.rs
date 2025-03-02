use gidth::numbers::Integer;

#[test]
fn big_int_sum() {
	let x = Integer::new(vec![234, 21, 84, 74,22, 89, 72, 75, 65, 10, 78, 89, 84, 85]);
	let y = Integer::new(vec![137, 45, 203, 78, 156, 241, 12, 89, 34, 220, 89, 32, 23, 81]);
	let my_int = &x + &y;
	println!("{}", my_int);
}
