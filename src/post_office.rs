
pub struct PostOffice;
impl PostOffice {
	pub fn send(&self, stuff: String){
		println!("sent {}", stuff)
	}
}