use gidth::number::{BigInt, Square};

// #[test]
// fn big_int_sum() {
// 	let x = BigInt::from("102276255580139097615819404275666350670467982146909786100732684133793143719564593097353327");
// 	let z = BigInt::from("-102276255580139097615819404275666350670467982146909786100732684133793143719564593097343327");
// 	let my_int = &x + &z + BigInt::from("22222222222222");
// 	println!("{}", my_int);
// 	println!("{}", my_int.sq());
// }
//
#[test]
fn big_int_multiply() {
	let x = 8234324234324u64;
	let y = 79832869324234324u64;
	let z: BigInt = BigInt::from(x) * BigInt::from(y);
	println!("{} = {}", z, x as u128 * y as u128);
}
//
// #[test]
// fn big_int_divide() {
// 	let int_y = BigInt::from("233422342343243242334234233");
// 	let int_x = BigInt::from(83243);
// 	let z = BigInt::divmod(&int_y, &int_x);
// 	println!("{} * {} + {} = {}", &z.0, &int_x, &z.1, &int_y);
// }

// #[test]
// fn big_int_exponentiation() {
// 	let y = BigInt::from("2334");
// 	let x = BigInt::from(8);
// 	let z = BigInt::exp(&y, &x);
// 	println!("{}", z);
// }

// #[test]
// fn big_int_fast_square() {
// 	let x = BigInt::from(vec![u32::MAX,u32::MAX,u32::MAX,1]);
// 	let mut r = BigInt::from(0);
// 	r.clear();
// 	fast_square(&x, &mut r);
// 	let res = &x * &x;
// 	println!("{} = {}", res, r);
// }
