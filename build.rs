use std::fs::{
	copy,
	read_dir,
	create_dir,
	remove_file,
};
use std::path::PathBuf;


fn recursive_copy(from: PathBuf, to: PathBuf) -> Result<(), std::io::Error> {
	let items = read_dir(from)?;
	for item in items {
		let path: PathBuf = item?.path();
		
		let name = path.file_name().expect("File has name");
		let new_to = to.join(name);
			
		if path.is_dir() {
			if !new_to.exists() {
				create_dir(new_to.clone())?;
			}
			recursive_copy(path, new_to)?;
		} else {
			println!("{:?}", &new_to);
			if new_to.exists() {
				remove_file(&new_to)?;
			}
			copy(path, new_to)?;
		}
	}
	
	Ok(())
}

fn main() {
	let to: PathBuf = "pkg".into();
	if !to.exists() {
		create_dir(to.clone()).unwrap();
	}
	recursive_copy("html".into(), to).unwrap();
	println!("cargo::rerun-if-changed=html/");
	println!("cargo::rerun-if-changed=build.rs");
}
