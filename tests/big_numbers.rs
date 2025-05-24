use gidth::number::Ratio;
use gidth::linear::Matrix;
// use gidth_macros::auto_wrap_ints;

// #[test]
// fn big_int_sum() {
// 	let x = BigInt::from("102276255580139097615819404275666350670467982146909786100732684133793143719564593097353327");
// 	let z = BigInt::from("-102276255580139097615819404275666350670467982146909786100732684133793143719564593097343327");
// 	let my_int = &x + &z + BigInt::from("22222222222222");
// 	println!("{}", my_int);
// 	println!("{}", my_int.sq());
// }
//
// #[test]
// fn big_int_multiply() {
// 	let x = 8234324234324u64;
// 	let y = 79832869324234324u64;
// 	let z: BigInt = BigInt::from(x) * BigInt::from(y);
// 	println!("{} = {}", z, x as u128 * y as u128);
// }
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
// 	let z = BigInt::pow(&y, &x);
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
//
// #[test]
// fn big_decimal_addition() {
// 	let x1 = BigDecimal {
// 		positive: true,
// 		digits: vec![69, 54, 420, 89432, 78, 909234, 90823423, 982340, 90234, 8923, 912, 9834],
// 		decimal_pos: 7,
// 	};
// 	let x2 = BigDecimal {
// 		positive: true,
// 		digits: vec![23, 234, 134],
// 		decimal_pos: 3,
// 	};
// 	println!("{:?} {}", x1.digits, x1.decimal_pos);
// 	println!("{:?} {}", x2.digits, x2.decimal_pos);
// 	let y = x1 - &x2;
// 	println!("{:?} {}", y.digits, y.decimal_pos);
// 	println!("{}", x2);
// }
// #[test]
// fn big_decimal_fmt() {
// 	let x1 = BigDecimal {
// 		positive: true,
// 		limbs: vec![u32::MAX],
// 		decimal_pos: 5,
// 	};
// 	println!("{}", x1);
// }

// #[test]
// fn big_decimal_conv() {
// 	let x2 = BigDecimal::from(8904238.000000001f64);
// 	println!("{}", &x2);
// }
// #[test]
// fn big_decimal_mul() {
// 	let x1 = BigDecimal::from("2342323423269.32097985340908728348735987493");
// 	let x2 = BigDecimal::from(8904238.000000001f64);
// 	println!("{}", &x1 * &x2);
// }
// #[test]
// fn big_decimal_div() {
// 	let x1 = -BigDecimal::from(vec![u32::MAX, u32::MAX, u32::MAX, 2]);
// 	let x2 = BigDecimal::from(vec![u32::MAX, 2]);
//
// 	let ans = &x1 / &x2;
// 	println!("{:?}", ans);
//
// 	println!("{} / {} = {}", &x1, &x2, &ans);
// 	// println!("{}", x4);
// }

// #[test]
// fn big_decimal_sq() {
// 	let num = 2342323423269.321f64;
// 	println!("{}", num);
// 	let x1 = BigDecimal::from(num);
// 	println!("{}", x1.sq());
// 	// println!("{}", x4);
// }

// #[test]
// fn big_decimal_pow() {
// 	let x1 = BigDecimal::from("2342323423269.32080078125");
// 	println!("{}", x1.pow(10));
// 	println!("{}", x1.sq());
// 	// println!("{}", x4);
// }

// #[test]
// fn testing() {
// 	let x: i32 = 69;
// 	println!("{:?}", x.sq());
// }
//
// #[test]
// fn test_powf() {
// 	let ln1p5 = 1.5f64;
// 	println!("{}", BigDecimal::ln(ln1p5));
// }
#[test]
fn test_matrix_det() {
	let a = Matrix::new([
		[2.0, 10.0, 7.0],
		[6.0, 3.0, 4.0],
		[5.0, -2.0, -3.0],
	]);
	let a_inv = Matrix::new([
		[1.0,  -1.0, 1.0],
		[-38.0, 41.0, -34.0],
		[27.0, -29.0, 24.0],
	]);
	println!("{}", a.det());
	println!("{}", Ratio::from(232)*Ratio::from(331));
}
//
// #[test]
// fn test_ratio() {
// 	println!("{}", Ratio::new(23432, 4534322342342i128) - &Ratio::new(23432, 4534322342342i128));
// }
