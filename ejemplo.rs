//! Lee operaciones de una lista de archivos, y las aplica.
//!
//! Una vez procesados todos los archivos, imprime el resultado final.
//!
//! ## Uso
//!
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
//! ### Ejercicio 2
//!
//! En lugar de utilizar locks para sincronizar el acceso a un recurso compartido, utilizar channels.
//!
//! ### Bonus
//!
//! Medir la diferencia de performance entre los 3 enfoques.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

// A basic wrapping u8 calculator.
//
// The possible values range from [0;256).
#[derive(Default)]
struct Calculator {
    value: u8,
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Add(u8),
    Sub(u8),
    Mul(u8),
    Div(u8),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        // Try to convert the vector into a statically-sized array of 2 elements, failing otherwise.
        let [operation, operand] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

        // Parse the operand into an u8.
        let operand: u8 = operand.parse().map_err(|_| "operand is not an u8")?;

        match operation {
            "+" => Ok(Operation::Add(operand)),
            "-" => Ok(Operation::Sub(operand)),
            "*" => Ok(Operation::Mul(operand)),
            "/" => Ok(Operation::Div(operand)),
            _ => Err("unknown operation"),
        }
    }
}

impl Calculator {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn apply(&mut self, op: Operation) {
        match op {
            Operation::Add(operand) => self.value = self.value.wrapping_add(operand),
            Operation::Sub(operand) => self.value = self.value.wrapping_sub(operand),
            Operation::Mul(operand) => self.value = self.value.wrapping_mul(operand),
            Operation::Div(operand) => self.value = self.value.wrapping_div(operand),
        }
    }
}

pub fn main() {
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
