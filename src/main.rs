use std::fs::{File, Metadata};
use std::error::Error;
use std::io;

fn write_yaml(meta: Metadata) {
	println!("ftype: {:?}", meta.file_type());
}

fn get_meta(file: File) {
	let meta = file.metadata();
	match meta {
		Ok(yaml) => write_yaml(yaml),
		Err(err) => io_err(err),
	}
}

fn io_err(err: io::Error) {
	println!("err: {err}");
}

fn main() {
	let meta = File::open("some.yaml");
	match meta {
		Err(err) => io_err(err),
		Ok(yaml) => get_meta(yaml),
	};
/*
	let meta = yaml.metadata()?;
	println!("is directory: {:?}", meta.is_dir());
	if let Ok(yaml) = tryopen {
		if let Ok(meta) = yaml.metadata() {
		}
	}
*/
	println!("Hello, world!");
    println!("some shit");
}
