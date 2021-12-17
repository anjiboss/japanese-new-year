#[path = "./funcs/mod.rs"] mod funcs;
#[path = "./smart_phone.rs"] mod smart_phone;
#[path = "./post_office.rs"] mod post_office;
#[path = "./new_year_card.rs"] mod new_year_card;
#[path = "./toshikoshi_soba.rs"] mod toshikoshi_soba;


pub fn 年末() {
	let 郵便 = post_office::PostOffice;
	let スマホ = smart_phone::SmartPhone;

 	let 友達 = funcs::rand_int();
	//  ---------------------------
	let 出かける = funcs::rand_bool();

	if 出かける == true {
		let 会う = funcs::rand_bool();
			if 会う == true {
				//  ---------------------------- 年末の挨拶
				funcs::say("よいお年を")
			}
	}else {
		let 連絡 = funcs::rand_bool();
		if 連絡 == true {
			for 親友 in 1..友達 {
				// ------------------------------  友達にメッセージを送る
				スマホ.open();
				let message = "良いお年を";
				let to = 親友;
				スマホ.send_message(message, to);
			}
		}
	}

	let 親族 = funcs::rand_int();
	for _親族 in 0..親族 {
		let 年賀状 = new_year_card::NewYearCard;
		年賀状.write("新年あけましておめでとうございます。
								皆様おすこやかに新春をお迎えのことと存じます。
								昨年は何かとお世話になりましてありがとうございました。
								本年もどうぞよろしくお願い申し上げます。"
							);
		郵便.send(年賀状.with_envelope());
	}

	let 年越しそば = toshikoshi_soba::ToshikoshiSoba;
	//  ---- ズーズ〜　ズーズ〜
	年越しそば.eat()
}