use std::io::{self, Write};
use std::str::FromStr;

pub fn cin<T>(res: &mut T) -> bool
where T: FromStr,
<T as FromStr>::Err: std::fmt::Display
{
	let mut buffer = String::new();
	
	match std::io::stdin().read_line(&mut buffer)
	{
		Err(e) => {
			println!("{}", e);
			return false 
		},
		_ => { },
	}
	
	match buffer.trim().parse::<T>()
	{
		Ok(value) => *res = value,
		Err(e) => {
			println!("{}", e);
			return false 
		},
	}
	
	return true;
}

pub fn cout(text: &str)
{
	io::stdout().write_all(text.as_bytes()).ok();
	io::stdout().flush().ok();
}