use gidth::numbers::Integer;
use gidth::numbers::unsigned_integer_multiplication;

#[test]
fn big_int_sum() {
	let x = Integer::new(vec![234, 21, 84]);
	let y = Integer::new(vec![137, 45, 432, 482385, 883274982374]);
	let z = Integer::new("-102276255580139097615819404275666350670467982146909786100732684133793143719564593097343327".to_string());
	let my_int = &x + &y + &z;
	println!("{}", my_int);
}

#[test]
fn big_int_multiply() {
	let x = Integer::new(35857);
	let y = Integer::new(52);
	println!("{}", unsigned_integer_multiplication(&x, &y));
}
