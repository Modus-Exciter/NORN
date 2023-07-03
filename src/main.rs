mod io_plus; use crate::io_plus::*;
mod prime_numbers; use crate::prime_numbers::*;

fn main() {
	let mut x:u32 = 0;
	
    cout("x = ");
	
	while !cin(&mut x)
	{
		println!("Число не распознано, попробуйте ещё раз");
	}
	
	if is_prime(x)
	{
		println!("Число {} простое", x);
	}
	else
	{
		eratosthenes(x, |n| println!("->{}", n));
	}
}
