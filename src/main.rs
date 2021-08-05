mod analyzer;
mod compiler;
mod executor;
mod parser;
mod symbol_table;

fn main() {
    let mut args = std::env::args();
    let current_program_path = args.next().unwrap();
    let source_path = args.next();

    if source_path.is_none() {
        run_interpreter();
    } else {
        process_file(&current_program_path, &source_path.unwrap());
    }
}

fn process_file(current_program_path: &str, source_path: &str) {
    const CALC_SUFFIX: &str = ".calc";
    if !source_path.ends_with(CALC_SUFFIX) {
        eprintln!(
            "{}: Invalid argument '{}': It must end with {}",
            current_program_path, source_path, CALC_SUFFIX
        );
        return;
    }
    let source_code = std::fs::read_to_string(&source_path);
    if source_code.is_err() {
        eprintln!(
            "Failed to read from file {}: ({})",
            source_path,
            source_code.unwrap_err()
        );
        return;
    }
    let source_code = source_code.unwrap();

    let parsed_program;
    match parser::program_parser(&source_code) {
        Ok((rest, syntax_tree)) => {
            let trimmed_rest = rest.trim();
            if trimmed_rest.len() > 0 {
                eprintln!(
                    "Invalid remaining code in '{}': {}",
                    source_path, trimmed_rest
                );
                return;
            }
            parsed_program = syntax_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {:?}", source_path, err);
            return;
        }
    }

    let analyzed_program;
    let mut variables = symbol_table::SymbolTable::new();
    match analyzer::analyze_program(&mut variables, &parsed_program) {
        Ok(analyzed_tree) => {
            analyzed_program = analyzed_tree;
        }
        Err(err) => {
            eprintln!("Invalid code in '{}': {}", source_path, err);
            return;
        }
    }

    //  Remove file if it exists
    if std::path::Path::new("./calc_target/output.rs").exists() {
        std::fs::remove_file("./output.rs").unwrap();
    }

    //  Create dir if it doesn't exist
    if !std::path::Path::new("./calc_target/").exists() {
        std::fs::create_dir("./calc_target").unwrap();
    }

    match std::fs::write(
        "./calc_target/output.rs",
        compiler::to_program(&variables, &analyzed_program),
    ) {
        Ok(_) => {
            eprintln!("Translation successful.");
        }
        Err(err) => {
            eprintln!("Translation failed with error: {}", err);
        }
    }
}

fn run_interpreter() {
    eprintln!("Calc interactive shell");
    let mut variables = symbol_table::SymbolTable::new();
    loop {
        let command = input_command();
        if command.len() == 0 {
            break;
        }
        match command.trim() {
            "q" => {
                eprintln!("Exiting");
                break;
            }
            "c" => {
                variables = symbol_table::SymbolTable::new();
                eprintln!("Cleared variables");
            }
            "v" => {
                eprintln!("Variables");
                for variable in variables.iter() {
                    eprintln!("  {}: {}", variable.0, variable.1);
                }
            }
            trimmed_command => match parser::program_parser(&trimmed_command) {
                Ok((rest, parsed_program)) => {
                    if rest.len() > 0 {
                        eprintln!("Unparsed input: `{}`.", rest)
                    }
                    match analyzer::analyze_program(&mut variables, &parsed_program) {
                        Ok(analyzed_program) => {
                            executor::execute_program(&mut variables, &analyzed_program)
                        }
                        Err(err) => eprintln!("Error: {}", err),
                    }
                }
                Err(err) => eprintln!("Error: {:?}", err),
            },
        }
    }
}

fn input_command() -> String {
    let mut text = String::new();
    eprint!("> ");
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line");
    text
}
