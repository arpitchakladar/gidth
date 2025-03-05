use gidth::number::{
	BigInt,
	fast_square,
};

// #[test]
// fn big_int_sum() {
// 	let x = BigInt::new(vec![234, 21, 84, 89009, 889243, 09233, 8213, 88]);
// 	let y = BigInt::new(vec![137, 45, 432, 482385, 88327498, 23,8234, 92, 893, 8204, 8921, 8402]);
// 	let z = BigInt::new("-102276255580139097615819404275666350670467982146909786100732684133793143719564593097343327");
// 	let my_int = &x + &y + &z;
// 	println!("{}", my_int);
// }

// #[test]
// fn big_int_multiply() {
// 	let x = 8234324234324u64;
// 	let y = 79832869324234324u64;
// 	let z: BigInt = BigInt::new(x) * BigInt::new(y);
// 	println!("{} = {}", z, x as u128 * y as u128);
// }
//
// #[test]
// fn big_int_divide() {
// 	let int_y = BigInt::new("233422342343243242334234233");
// 	let int_x = BigInt::new(83243);
// 	let z = BigInt::divmod(&int_y, &int_x);
// 	println!("{} * {} + {} = {}", &z.0, &int_x, &z.1, &int_y);
// }

// #[test]
// fn big_int_exponentiation() {
// 	let y = BigInt::new("2334");
// 	let x = BigInt::new(82);
// 	let z = BigInt::exp(&y, &x);
// 	println!("{}", z);
// }

#[test]
fn big_int_exponentiation() {
	let x = BigInt::new(vec![1, 2, 3, 4, 5]);
	let z = fast_square(&x);
	println!("{} = {}", &x * &x, z);
}
