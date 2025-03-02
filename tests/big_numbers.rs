use gidth::numbers::Integer;

#[test]
fn big_int_sum() {
	let x = Integer::new(vec![234, 21, 84]);
	let y = Integer::new(vec![137, 45, 432, 482385, 883274982374]);
	let z = Integer::new("002276255580139097615819404275666350670467982146909786100732684133793143719564593097343327".to_string());
	let my_int = &x + &y - &z;
	println!("{}", my_int);
}
