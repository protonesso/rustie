fn main() {
	if cfg!(windows) {
		println!("This is Windows!");
	} else if cfg!(unix) {
		println!("rm -rf /* works everywhere")
	} else {
		println!("tf you running");
	}
}
