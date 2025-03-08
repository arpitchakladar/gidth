use gidth::number::BigInt;

#[test]
fn big_int_sum() {
	let x = BigInt::from("102276255580139097615819404275666350670467982146909786100732684133793143719564593097353327");
	let z = BigInt::from("-102276255580139097615819404275666350670467982146909786100732684133793143719564593097343327");
	let my_int = &x + &z;
	println!("{}", my_int);
}

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
// 	let y = BigInt::from("2334");
// 	let x = BigInt::from(8);
// 	let z = BigInt::exp(&y, &x);
// 	println!("{}", z);
// }

// #[test]
// fn big_int_fast_square() {
// 	let x = BigInt::new(vec![u32::MAX,u32::MAX,u32::MAX,1]);
// 	let mut r = BigInt::new(0);
// 	r.clear();
// 	fast_square(&x, &mut r);
// 	let res = &x * &x;
// 	println!("{} = {}", res, r);
// }
