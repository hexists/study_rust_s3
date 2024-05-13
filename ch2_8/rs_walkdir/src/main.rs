use walkdir::WalkDir;


fn main() {
	for entry in WalkDir::new(".") {
		println!("{:?}", entry);
	}
}
