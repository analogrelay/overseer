extern crate getopts;

use self::getopts::{optopt, getopts};

pub struct Overseer;

pub struct Options {
	_config_file: Option<String>
}

impl Options {
	pub fn from_args(args: Vec<String>) -> Options {
		let opts = [
			optopt("c", "", "the config file to use", "CONFIG_FILE")
		];
		
		let matches = match getopts(args.tail(), opts) {
			Ok(m) => m,
			Err(f) => fail!(f.to_string())
		};

		Options {
			_config_file: matches.opt_str("c")
		}
	}

	pub fn config_file(&self) -> Option<&str> {
		match self._config_file {
			Some(ref val) => Some(val.as_slice()),
			None => None
		}
	}
}

impl Overseer {
	pub fn run(opts: Options) {
		println!("Using config file: {}", opts.config_file());
	}
}