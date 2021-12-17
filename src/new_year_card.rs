pub struct NewYearCard;
impl NewYearCard {
	pub fn write(&self, smth: &str){
		println!("{}",smth);
	}
	pub fn with_envelope(&self) -> String {
		let msg = String::from("封筒しました。");
		return msg;
	}
}