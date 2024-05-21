use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn read_json() -> io::Result<String>{
	let config_path = dirs::home_dir()
		.map(|p| p.join(".rcont/main.json"))
		.expect("Failed to find home directory");

	if config_path.exists() {
		let mut file = File::open(config_path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		Ok(contents)
	} else {
		create_json()?;
		return read_json();
	}
}

fn create_json() -> io::Result<()>{
	let config_path = dirs::home_dir()
			.map(|p| p.join(".rcont/main.json"))
			.expect("Failed to find home directory");
	let file = File::create(config_path)?;
	Ok(())
}
