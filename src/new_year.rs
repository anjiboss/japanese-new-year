#![allow(non_snake_case)]
#[path = "./funcs/mod.rs"] mod funcs;
#[path = "./jinja.rs"] mod jinja;
#[path = "./mochi.rs"] mod mochi;
#[path = "./osechi.rs"] mod osechi;
#[path = "./lucky_money.rs"] mod lucky_money;
#[path = "./omikuji.rs"] mod omikuji;

pub fn 新年() {
	//  -------------------- 神社に行く
	let 神社 = jinja::Jinja;
	const 初詣 :bool = true;
	if 初詣 {
		神社.goto();
		神社.拝む();
	}

	// ---------------------- 鏡餅を食べる
	let 鏡餅 = mochi::KagamiMochi;
	鏡餅.eat();

	let おせち = osechi::OSeChi;
	おせち.eat();

	//  ---------------------- お年玉
	let お年玉 = lucky_money::LuckyMoney;
	let _age = funcs::prompt("今年何歳ですか？ ");
	let age: i32 = _age.parse::<i32>().unwrap();
	if age > 15 {
		お年玉.give()
	}else {
		お年玉.receive()
	}

	// ----------------------- おみくじ
	let おみくじ = omikuji::OmiKuji;
	if おみくじ.引く() == "大吉" {
		println!("Lucky Lucky 💞")
	}
	
	println!("Happy New Year 🎇🎆")
}