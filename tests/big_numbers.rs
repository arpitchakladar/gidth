use gidth::numbers::{
	Integer,
};

#[test]
fn big_int_sum() {
	let x = Integer::new(vec![234, 21, 84, 89009, 889243, 09233, 8213, 88]);
	let y = Integer::new(vec![137, 45, 432, 482385, 88327498, 23,8234, 92, 893, 8204, 8921, 8402]);
	let z = Integer::new("-102276255580139097615819404275666350670467982146909786100732684133793143719564593097343327");
	let my_int = &x + &y + &z;
	println!("{}", my_int);
}

#[test]
fn big_int_multiply() {
	let x = 84u128;
	let y = 79832869u128;
	let z = Integer::new(x) * Integer::new(y);
	println!("{}", z * Integer::new(-10));
}

#[test]
fn big_int_divide() {
	let x = 8324324u128;
	let int_y = Integer::new("233422342343243242334234233");
	let int_x = Integer::new(x);
	let z = int_y / int_x;
	println!("{}", z);
}
