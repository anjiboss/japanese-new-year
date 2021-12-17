pub fn vars() {
	// let name = "dragon";
	// let mut age = 10; 
	// age = 23;
	// println!("My name is {} and I'm {} year old", name, age)
	println!("{}", std::i32::MAX);
	println!("{}", std::i64::MAX);

	let mut hello = String::from("Heloo"); 
	println!("{}" ,hello);
	hello.push_str(" world");
	println!("{}", hello)
}