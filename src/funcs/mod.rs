use std::io;
use rand::Rng;
use std::io::Write;


/// Pass a {String} to show on the screen
/// @return {String} 
pub fn prompt(prompt: &str) -> String {
	 let mut line = String::new();
   print!("{}", prompt);
	 io::stdout().flush().unwrap();
   std::io::stdin().read_line(&mut line).unwrap();
	 return line;
}

pub fn rand_bool() -> bool {
	let result = rand::thread_rng().gen_range(0..2);
	if result == 1{
		return true
	}else {
		return false
	}
}

pub fn rand_int() -> i32 {
	let result = rand::thread_rng().gen_range(0..100);
	return result
}

pub fn say(smth: &str) {
	println!("{}", smth)
}