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
	check: &'static str
}

const SPINNER_STATES: [&'static str; 4] = ["/", "-", "\\", "|"];

impl Compilation {
	fn advance(self) -> Compilation {
        let check = "|";
		Compilation{check: check, .. self}
	}
}

impl fmt::Display for Compilation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, " [{}] {} {}", self.check, self.status, self.file)
	}
}

fn main() {
	let mut compiles = vec![
		Compilation{file: "stuff.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: "/"},
		Compilation{file: "things.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: "/"},
	];

	for i in 0..20 {
		display(&compiles);
		thread::sleep_ms(100);
		compiles = step(&mut compiles);
	}
}

fn display(compiles: &Vec<Compilation>) {
	for c in compiles.iter() {
		println!("{}", c)
	}
	// go back up to where we started?
}

fn step(compiles: &mut Vec<Compilation>) -> Vec<Compilation> {
    let mut next = vec![];
    while let Some(compile) = compiles.pop() {
        next.push(compile.advance());
    }
    next.reverse();
    next
}






