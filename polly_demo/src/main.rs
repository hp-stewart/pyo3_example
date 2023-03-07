use std::io::BufReader;
use std::io::prelude::*;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::fs::File;
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::types::PyTuple;
use pyo3::exceptions::PySyntaxError;

// Input text
const INPUT_TEXT: &str = "Happy Tuesday";
const PY_FILE: &str = "py/polly.py";

fn main() {
    // test of play_mp3_audio() function
    let p = Path::new("output.mp3");
    play_mp3_audio(&p);

    // test of call_polly() function
    let s = String::from(INPUT_TEXT);
    match call_polly(s) {
        Ok(path) => {
            println!("\nPy Function was successful!! \nThe audio file was saved at: {path:?} \n");
            play_mp3_audio(&path);
        },
        Err(e) => println!("\nPy Function failed because {e}...\n"),
    };
}

fn call_polly(text: String) -> Result<PathBuf, Error> {
    // Initialize Python interpreter and acquire Global Interpreter Lock
    println!("\nInitializing py interpreter...");
    Python::with_gil(|py| {
        // first we need to grab the python code from a local file
        let code = get_py_file_contents(PY_FILE)?;
        println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");

        // attempt create PyModule from contents of file
        let functions_pymodule: Result<&PyModule, PyErr> = PyModule::from_code(py, &code, "functions.py", "functions");
        let args = PyTuple::new(py, &[text]);
        println!("\nEvaluating python code using args: {args:?}...\n-----start of py output-----\n");

        match functions_pymodule?.getattr("polly_demo")?.call1(args) {
            Ok(p) => { // python function was completed successfully
                println!("\n-----end of py output-----\npolly_demo() function call succeeded");
                match p.extract::<String>() {
                    Ok(p) => {return Ok(PathBuf::from(is_str_valid_filepath(&p)?));},
                    Err(e) => {return Err(Error::new(ErrorKind::Other, e));},
                };
            }
            Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                println!("\nResult: ERR (InvalidInput) \nPython module could not be created due to syntax error");
                return Err(Error::new(ErrorKind::InvalidInput, pyerr));
            }
            Err(e) => {return Err(Error::new(ErrorKind::Other, e));}
        };
    })
}

// helper functions
fn is_str_valid_filepath(s: &str) -> Result<&Path, Error> {
    match Path::new(s).try_exists() {
        Ok(true) => Ok(Path::new(s)),
        Ok(false) => Err(Error::new(ErrorKind::Other,"Could not access a file--check for broken symbolic link")),
        Err(e) => Err(e),
    }
}

fn get_py_file_contents(file_name: &str) -> Result<String, Error> {
    match File::open(is_str_valid_filepath(file_name)?) {
        Ok(mut file) => {
            // create a new String and read the file contents into it
            let mut s = String::new();
            file.read_to_string(&mut s)?;

            if s.is_empty() { // make sure string is not empty
                Err(Error::new(ErrorKind::UnexpectedEof,String::from("Py file was empty...")))
            } else {
                Ok(s)}
        },
        Err(e) => {
            println!("Failed to open file");
            // can potentially use match to create different behavior depending on what kind of error occcured
            match e.kind() {
                ErrorKind::NotFound => println!("File not Found"),
                ErrorKind::PermissionDenied => println!("Permission Denied"),
                _ => println!("Unknown Error Occured: {}", e),
            }
            Err(e)
        }
    }
}

fn play_mp3_audio(mp3_path:&Path) {
    println!("\nPlaying audio from {mp3_path:?}");
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let file = std::fs::File::open(mp3_path).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();  
}