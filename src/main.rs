
//! Para procesar todos los archivos de la carpeta `data/`, ejecutar:
//! ```bash
//! cargo run -- data/*
//! ```
//! El resultado esperado de una ejecución secuencial es 26.
//!
//! ## Ejercicios
//!
//! ### Ejercicio 1
//!
//! Utilizar threads y locks para procesar los archivos de forma concurrente.
//!
//! NOTA: Una ejecución concurrente daría un resultado distinto.
//!


use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};



fn main() {
    // `Args` is an iterator over the program arguments.
    let mut inputs = std::env::args();

    // We skip the first argument, as its traditionally the path to the executable.
    inputs.next();

    // We maintain a *global* calculator for the entire program.
    let mut calculator = Calculator::default();

    for input in inputs {
        // Open the input file.
        let file = File::open(input).expect("failed to open input file");

        // We need to create a BufReader for the file.
        //
        // It can be excessively inefficient to work directly with a reader,
        // as each read results in a system call. A buffered readered performs
        // large, infrequent reads on the underlying reader and maintains an
        // in-memory buffer of the results.
        let file_reader = BufReader::new(file);

        // A buffered reader also implements useful methods, like `lines()`
        for line in file_reader.lines() {
            // The underlying reader (file) may fail. In that case, we print the
            // error and skip the current file.
            let line = match line {
                Ok(line) => line,
                Err(error) => {
                    eprintln!("failed to read line {}", error);
                    break;
                }
            };

            // The operation may be invalid. In that case, we print the error
            // and skip the current *line*.
            let operation = match Operation::from_str(&line) {
                Ok(operation) => operation,
                Err(error) => {
                    eprintln!("failed to parse line {}", error);
                    continue;
                }
            };

            calculator.apply(operation);
        }
    }

    println!("{}", calculator.value())
}