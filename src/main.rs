use std::fmt;
use std::io;
use std::io::Write;
use std::thread;

enum CompileStatus {
    NotStarted,
    Parsing,
    Configuring,
    Analyzing,
    Translating,
    Linking,
}

impl CompileStatus {
	fn name(&self) -> &'static str {
		match *self {
		  CompileStatus::NotStarted => "Not started",
		  CompileStatus::Parsing => "Parsing",
		  CompileStatus::Configuring => "Configuring and expanding",
		  CompileStatus::Analyzing => "Analyzing",
		  CompileStatus::Translating => "Translating to LLVM",
		  CompileStatus::Linking => "Linking"
		}
	}

	fn next(&self) -> CompileStatus {
		match *self {
		  CompileStatus::NotStarted => CompileStatus::Parsing,
		  CompileStatus::Parsing => CompileStatus::Configuring,
		  CompileStatus::Configuring => CompileStatus::Analyzing,
		  CompileStatus::Analyzing => CompileStatus::Translating,
		  CompileStatus::Translating => CompileStatus::Linking,
		  CompileStatus::Linking => CompileStatus::Linking
		}
	}
}

impl fmt::Display for CompileStatus {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.name())
	}
}

struct Compilation {
	file: String,
	status: CompileStatus,
	time: u64,
	check: String,
}

impl Compilation {
	fn advance(&mut self) {
		self.check = "/".to_string();
	}
}

impl fmt::Display for Compilation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, " [{}] {} {}", self.check, self.status, self.file)
	}
}

fn main() {
	let mut c = Compilation{file: "stuff.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: "|".to_string()};

	print!("{}", c);
	io::stdout().flush();

	thread::sleep_ms(1000);
	c.advance();
	print!("\r{}", c);
	print!("\n");
}
