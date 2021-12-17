pub struct SmartPhone;
impl SmartPhone {
 pub fn open(&self) {
	 println!("Opened")
 }
 pub fn send_message(&self, message: &str, to: i32) {
	 println!("Sent: {} to {}th Friend", message, to)
 }
}
