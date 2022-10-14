use std::env::args;

use interpreter::Interpreter;

mod interpreter;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 {
        let filepath = args.get(1).unwrap();
        let mut brainfuck = Interpreter::new(128, true);
        brainfuck.interpret(&filepath);
    } else {
        println!(
            "
        Program: brainfuck
        
        Usage:
            <filepath>  :   executes given file.
        "
        );
        std::process::exit(1);
    }
}
