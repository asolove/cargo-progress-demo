extern crate rand;
extern crate term;

use std::fmt;
use std::io::prelude::*;
use std::thread;

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
	check: usize
}

const SPINNER_STATES: [&'static str; 4] = ["/", "-", "\\", "|"];
const DONE_STATE: &'static str = "✓";

impl Compilation {
	fn advance(self) -> Compilation {
        let check = (self.check+1) % 4;
        let status = if rand::random::<f32>() > 0.95 { self.status.next() } else { self.status };
		Compilation{check: check, status: status, .. self}
	}
}

impl fmt::Display for Compilation {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let check = match self.status {
            CompileStatus::Done => DONE_STATE,
            CompileStatus::NotStarted => " ",
            _ => SPINNER_STATES[self.check]
        };
		write!(f, " [{}] {}: {}", check, self.file, self.status)
	}
}

fn main() {
	let mut compiles = vec![
		Compilation{file: "alligators.rs".to_string(), status: CompileStatus::Parsing, check: 0},
		Compilation{file: "bears.rs".to_string(), status: CompileStatus::Parsing, check: 0},
		Compilation{file: "cats.rs".to_string(), status: CompileStatus::NotStarted, check: 0},
		Compilation{file: "dogs.rs".to_string(), status: CompileStatus::NotStarted, check: 0},
		Compilation{file: "elephants.rs".to_string(), status: CompileStatus::NotStarted, check: 0},
		Compilation{file: "fauns.rs".to_string(), status: CompileStatus::NotStarted, check: 0},
	];

	for _ in 0..200 {
		display(&compiles);
		thread::sleep_ms(100);
		compiles = step(&mut compiles);
	}
}

fn display(compiles: &Vec<Compilation>) {
    let mut t = term::stdout().unwrap();
	for c in compiles.iter() {
        write!(t, "{}", CLEAR_LINE).unwrap();
        let color = if c.status == CompileStatus::Done {
            term::color::GREEN
        } else {
            term::color::YELLOW
        };
        t.fg(color).unwrap();
        write!(t, "{}\n", c).unwrap();
	}
	for _ in compiles.iter() {
        write!(t, "{}", UP_LINE).unwrap()
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






