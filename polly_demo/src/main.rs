use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

use pyo3::exceptions::PySyntaxError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::types::PyTuple;

fn main() {
    let s = String::from("Hello Polly.");

    println!("\nAWS Polly Example:");
    let _r7c = match call_polly(s) {
        Ok(n) => println!("\nPy Function was successful!! \nThe result was Ok({n:?}) \n"),
        Err(e) => println!("\nPy Function failed because {e}...\n"),
    };
}

// example function

fn call_polly(text: String) -> Result<Option<bool>, Error> {
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // first we need to grab the python code from a local file
        let code = get_py_file_contents("py/polly.py")?;
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");

        // attempt create PyModule from contents of file
        // this module can be used to access individual functions separately
        let functions_pymodule: Result<&PyModule, PyErr> =
            PyModule::from_code(py, &code, "functions.py", "functions");

        let args = PyTuple::new(py, &[text]);
        println!(
            "\nEvaluating python code using args: {args:?}...\n-----start of py output-----\n"
        );
        match functions_pymodule?.getattr("polly_demo")?.call1(args) {
            Ok(_) => {
                // python function completed successfully
                println!("\n-----end of py output-----\n");
                println!("polly_demo() function call succeeded");
                return Ok(Some(true));
            }
            Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                println!("\nResult: ERR (InvalidInput) \nPython module could not be created due to syntax error");
                return Err(Error::new(ErrorKind::InvalidInput, pyerr));
            }
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, e));
            }
        };
    })
}

// helper functions

// path should meet the following requirements:
//  - path is valid and exists
//  - path leads to a file with .py extension
//  - the file can be read and its contents are not empty
fn validate_py_path(path: &Path) -> Result<(), String> {
    // confirm path exists and is a file rather than a directory
    if !path.exists() {
        return Err("Path does not exist".to_owned());
    }
    if !path.is_file() {
        return Err("Path does not lead to a file (maybe a directory?)".to_owned());
    }

    // file extension must be a .py
    let expected_extension = OsStr::new("py");
    match path.extension() {
        Some(ext) => {
            println!("file ext: {:?}", ext);
            if ext == expected_extension {
                Ok(())
            } else {
                Err("Invalid file extension".to_owned())
            }
        }
        None => {
            println!("c");
            Err("path.extension() failed--maybe path does not have a period delimiting the extension?".to_owned())
        }
    }
}

fn get_py_file_contents(file_name: &str) -> Result<String, Error> {
    // Create a path to the desired file
    println!("Opening file: {}", &file_name);
    let path = Path::new(file_name);

    // validate path
    let path_validation = validate_py_path(&path);
    if path_validation.is_err() {
        return Err(Error::new(
            ErrorKind::Other,
            path_validation
                .err()
                .unwrap_or("Could not validate path".to_owned()),
        ));
    }

    // try to open the file
    let file = File::open(&path);

    // match on file to examine result of open operation
    let result: Result<String, Error> = match file {
        Ok(mut file) => {
            println!("File was opened successfully");

            // create a new String and read the file contents into it
            let mut s = String::new();
            file.read_to_string(&mut s)?;

            // make sure file is not empty
            if s.is_empty() {
                return Err(Error::new(
                    ErrorKind::Other,
                    String::from("Py file was empty..."),
                ));
            }
            // finished inner actions for successful file read--return file content string inside Result
            Ok(s.to_owned())
        }
        Err(e) => {
            println!("Failed to open file");
            // can use match to create different behavior depending on what kind of error occcured
            match e.kind() {
                ErrorKind::NotFound => println!("File not Found"),
                ErrorKind::PermissionDenied => println!("Permission Denied"),
                _ => println!("Unknown Error Occured: {}", e),
            }
            // finished inner actions for unsuccessful file read--return error inside Result
            Err(e)
        }
    };
    return result;
}
