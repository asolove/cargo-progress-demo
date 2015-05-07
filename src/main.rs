extern crate rand;

use std::fmt;
use std::io;
use std::io::Write;
use std::thread;
use rand::Rng;


static UP_LINE: &'static str = "\x1b[1F";
static CLEAR_LINE: &'static str = "\x1b[K";

#[derive(Clone, Copy, Eq, PartialEq)] 
enum CompileStatus {
    NotStarted,
    Parsing,
    Configuring,
    Analyzing,
    Translating,
    Linking,
    Done
}

impl CompileStatus {
	fn name(&self) -> &'static str {
		match *self {
		  CompileStatus::NotStarted => "Not started",
		  CompileStatus::Parsing => "Parsing",
		  CompileStatus::Configuring => "Configuring and expanding",
		  CompileStatus::Analyzing => "Analyzing",
		  CompileStatus::Translating => "Translating to LLVM",
		  CompileStatus::Linking => "Linking",
          CompileStatus::Done => "Done"
		}
	}

	fn next(&self) -> CompileStatus {
		match *self {
		  CompileStatus::NotStarted => CompileStatus::Parsing,
		  CompileStatus::Parsing => CompileStatus::Configuring,
		  CompileStatus::Configuring => CompileStatus::Analyzing,
		  CompileStatus::Analyzing => CompileStatus::Translating,
		  CompileStatus::Translating => CompileStatus::Linking,
		  CompileStatus::Linking => CompileStatus::Done,
          CompileStatus::Done => CompileStatus::Done
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
	check: usize
}

const SPINNER_STATES: [&'static str; 4] = ["/", "-", "\\", "|"];
const DONE_STATE: &'static str = "*";

impl Compilation {
	fn advance(self) -> Compilation {
        let check = (self.check+1) % 4;
        let status = if rand::random::<f32>() > 0.95 { self.status.next() } else { self.status };
		Compilation{check: check, status: status, .. self}
	}
}

impl fmt::Display for Compilation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let check = if self.status == CompileStatus::Done {
            DONE_STATE
        } else {
            SPINNER_STATES[self.check]
        };
		write!(f, " [{}] {} {}", check, self.status, self.file)
	}
}

fn main() {
	let mut compiles = vec![
		Compilation{file: "alligators.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: 0},
		Compilation{file: "bears.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: 0},
		Compilation{file: "cats.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: 0},
		Compilation{file: "dogs.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: 0},
		Compilation{file: "elephants.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: 0},
		Compilation{file: "fauns.rs".to_string(), status: CompileStatus::NotStarted, time: 0, check: 0},
	];

	for i in 0..200 {
		display(&compiles);
		thread::sleep_ms(100);
		compiles = step(&mut compiles);
	}
}

fn display(compiles: &Vec<Compilation>) {
	for c in compiles.iter() {
        print!("{}", CLEAR_LINE);
		println!("{}", c);
	}
	for c in compiles.iter() {
		print!("{}", UP_LINE);
	}
}

fn step(compiles: &mut Vec<Compilation>) -> Vec<Compilation> {
    let mut next = vec![];
    while let Some(compile) = compiles.pop() {
        next.push(compile.advance());
    }
    next.reverse();
    next
}






