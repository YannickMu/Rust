use std::{io::stdout, io::Write, thread, sync::{Arc, Mutex}};
use sha2::{Sha512, Digest};
use rand::RngCore;
use regex::Regex;

fn main() {
	let mut count : u64 = 0;
	let th = thread::spawn(move ||{
		loop {
			let hs = Regex::new(r"^AAAAAAAAAA").unwrap();
			let mut data = [0u8; 128];
			let mut hasher = Sha512::new();
			rand::thread_rng().fill_bytes(&mut data);
			hasher.update(data);
			let res = format!("{:X}", hasher.finalize());
			if hs.is_match(&res) {
				println!("Found hash: {res:?}\nData: {data:?}");
				std::process::exit(0);
			}
			count += 1;
			write!(stdout().lock(), "Bytes tested: {}\r", count).unwrap();
		}
	});
	let count = Arc::new(Mutex::new(0u64));
	for _i in 1..11 {
		let countse = Arc::clone(&count);
		thread::spawn(move ||{
			loop {
				let hs = Regex::new(r"^AAAAAAAAAA").unwrap();
				let mut data = [0u8; 128];
				let mut hasher = Sha512::new();
				rand::thread_rng().fill_bytes(&mut data);
				hasher.update(data);
				let res = format!("{:X}", hasher.finalize());
				if hs.is_match(&res) {
					println!("Found hash: {res:?}\nData: {data:?}");
					std::process::exit(0);
				}
				let mut countse = countse.lock().unwrap();
				*countse += 1;
				write!(stdout().lock(), "Bytes tested: {}\r", *countse).unwrap();
			}
		});
	}
	th.join().unwrap();
}
