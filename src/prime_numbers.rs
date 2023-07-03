pub fn is_prime(n: u32) -> bool
{
	if n < 4
	{
		return n > 1;
	}
	
	if n % 2 == 0 || n % 3 == 0
	{
		return false;
	}
	
	let mut k:u32 = 5;
	while k * k <= n
	{
		if n % k == 0
		{
			return false;
		}
		if n % (k + 2) == 0
		{
			return false;
		}
		
		k += 6;
	}
	
	return true;
}

pub fn eratosthenes(n: u32, out: fn(u32))
{
	if n < 2
	{
		return;
	}
	
	let n = n as usize;
	let mut values = vec![true; n - 1];
	let mut count = 0;
	
	for i in (0usize..).take_while(|i| i * i <= (n + 1))
	{
		if values[i]
		{
			let mut x = (i + 2) * (i + 2) - 2;
			
			while x < values.len()
			{
				values[x] = false;
				x += i + 2;
			}
		}
	}
	
	for i in 0..values.len()
	{
		if values[i]
		{
			out((i + 2) as u32);
			count += 1;
		}
	}
	
	out(count);
}