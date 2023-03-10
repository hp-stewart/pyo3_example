

    use std::ffi::OsStr;
    use std::io::prelude::*;
    use std::io;
    use std::io::Error;
    use std::io::ErrorKind;
    use std::collections::HashMap;
    use std::path::Path;
    use std::fs::File;
    
    use std::process::Command; // used to do "$pip list" from inside Rust
    
    use pyo3::exceptions::PyModuleNotFoundError;
    use pyo3::exceptions::PySyntaxError;
    use pyo3::exceptions::PyTypeError;
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;
    use pyo3::types::PyModule;
    use pyo3::types::PyTuple;
    use pyo3::types::IntoPyDict;
    
    // Example functions are defined below
    
    // Example 1
    // Simplest method-- write a python snippet as a rust string and execute it using .eval()
    // The function returns a PyResult which is a public type representing the result of a python call
    //    pub type PyResult<T> = Result<T, PyErr>
    // where PyErr  represents a python exception 
    pub fn simple_inline_python_code() -> PyResult<()> {
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
            // define the python code to be evaluated
            let code = "print('Hello, World!')";
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code:?}\n\n-----end of py code-----");
    
            // execute the code using .eval()
            // the method returns the result evaluated from the expression as an &PyAny object
            // this method accepts (code: &str, globals: Option<&PyDict>, locals: Option<&PyDict>) and returns PyResult<&'py PyAny>
            // if globals is None, it defauilts to python module __main__
            // If locals is None, it defaults to the value of globals
            // you can optionally use .extract()? to assign the a value to a new variable
            println!("\nEvaluating...\n-----start of py output-----\n");
            py.eval(code, None, None)?;
            println!("\n-----end of py output-----\nEvaluation completed\n");
    
            // Finished--> PyResult = Ok
            Ok(())
        })
    }
    
    
    // Example 2
    // This example shows two ways to import a package 
    // that has been installed for the active python interpreter
    // the python snippet in this example depends on import sys and 
    // import os, two default packages
    pub fn print_python_version() -> PyResult<()> {
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
            // You can get certain data from py modules without writing any inline code 
            // The first example will retrieve the current version of python from the "sys" module
            
            // import the library to the active interpreter session
            // .getattr() can be used to retrieve an attribute vale, in this case "version"
            //     self.getattr(attr_name) 
            // in Rust is equivalent to the python expression 
            //     self.attr_name
            // .extract() is used to get the corresponding rust type binding from a python object 
            let sys = py.import("sys")?;
            let version: String = sys.getattr("version")?.extract()?;
    
            // The second example will add an imported library to some inline python code
            // import the library or libraries to be used
            // create a vector of tuples representing each library with a name & pointer
            // then convert the vector to a python dictionary
            let os = py.import("os")?; 
            let locals = [("os", os)].into_py_dict(py);
            
            // define the code to be executed
            let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
    
            // evaluate the python expression using .eval()
            // remember to pass in the libraries via "locals"
            // and convert the result from a python type to rust type using .extract()
            println!("\nEvaluating...\n-----start of py output-----\n");
            let user: String = py.eval(code, None, Some(&locals))?.extract()?;
            println!("\n-----end of py output-----\nEvaluation completed");
    
            // the values we got from python can now be used in Rust
            println!("\nRust Output:\n\tUser: {}, \n\tVersion: {}\n", user, version);
    
            // Finished--> PyResult = Ok
            Ok(())
        })
    }
    
    // Example 3
    // This example shows some ways to pass one or more args from Rust to Python
    // It also shows how to evaluate a python code containing multiple expressions
    pub fn python_function_with_args() -> PyResult<()> {
        // Define the arguments that will be sent to the python function
        let arg1 = "arg1";
        let arg2 = "arg2";
        let arg3 = "arg3";
    
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
            // Previous examples executed a single expression using .eval()
            // this time a longer snippet will be created--a function definition
            let code = "def example(*args, **kwargs):
            if args != ():
                print('called with args', args)
            if kwargs != {}:
                print('called with kwargs', kwargs)
            if args == () and kwargs == {}:
                print('called with no arguments')";
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
    
            // for longer code we can create a pyModule instead of using .eval()
            // it can contain one or more function definitions
            // the functions can then be accessed as attributes of the pymodule
            let example_function: Py<PyAny> = PyModule::from_code(
                py,
                &code,
                "",
                "",
            )? // create a PyModule from the provided snippet of code
            .getattr("example")? // extract the function called "example"
            .into(); // type conversion
    
            // Now we can use the python function "example" using the example_function object 
            // To call the function without any arguments use .call0()
            println!("\nDemo#3.1 Call function without arguments\nEvaluating...\n-----start of py output-----\n");
            example_function.call0(py)?;
            println!("\n-----end of py output-----\nDemo#3.1 completed\n");
    
            // args can be provided via PyTuple
            // To call the function with one or more arguments, use .call1()
            println!("\nDemo#3.2 Call function with PyTuple arguments\nEvaluating...\n-----start of py output-----\n");
            let args = PyTuple::new(py, &[arg1, arg2, arg3]);
            example_function.call1(py, args)?;
            println!("\n-----end of py output-----\nDemo#3.2 completed\n");
    
            // args can also be provided via Rust Tuple
            // To call the function with one or more arguments, use .call1()
            println!("\nDemo#3.3 Call function with Rust Tuple arguments\nEvaluating...\n-----start of py output-----\n");
            let args = (arg1, arg2, arg3);
            example_function.call1(py, args)?;
            println!("\n-----end of py output-----\nDemo#3.3 completed\n");
    
            // Finished--> PyResult = Ok
            Ok(())
        })
    }
    
    // Example 4
    // This example shows some ways to pass one or more kwargs from Rust to Python
    pub fn python_function_with_kwargs() -> PyResult<()> {
        // Define the keywords arguments that will be sent to the python function
        let key1 = "key1";
        let val1 = 1;
        let key2 = "key2";
        let val2 = 2;
        
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
            // define the python code to be run
            let code = "def example(*args, **kwargs):
            if args != ():
                print('called with args', args)
            if kwargs != {}:
                print('called with kwargs', kwargs)
            if args == () and kwargs == {}:
                print('called with no arguments')";
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
            
            // create a new PyModule containing the python code and extract the function using .getattr()
            let example_function: Py<PyAny> = PyModule::from_code(
                py,
                code,
                "",
                "",
            )?
            .getattr("example")?
            .into();
    
            // pass kwargs with PyDict
            println!("\nDemo#4.1 Call function with kwargs from PyDict\nEvaluating...\n-----start of py output-----\n");
            let kwargs = [(key1, val1)].into_py_dict(py);
            example_function.call(py, (), Some(kwargs))?;        
            println!("\n-----end of py output-----\nDemo#4.1 completed\n");
    
    
            // pass kwargs with Vec
            println!("\nDemo#4.2 Call function with kwargs from vec!\nEvaluating...\n-----start of py output-----\n");
            let kwargs = vec![(key1, val1), (key2, val2)];
            example_function.call(py, (), Some(kwargs.into_py_dict(py)))?;
            println!("\n-----end of py output-----\nDemo#4.2 completed\n");
    
            // pass arguments as HashMap
            println!("\nDemo#4.3 Call function with kwargs from HashMap\nEvaluating...\n-----start of py output-----\n");
            let mut kwargs = HashMap::<&str, i32>::new();
            kwargs.insert(key1, 1);
            example_function.call(py, (), Some(kwargs.into_py_dict(py)))?;
            println!("\n-----end of py output-----\nDemo#4.3 completed\n");
    
            Ok(())
        })
    }
    
    // Example 5
    // In this example the python code is provided by a separate .py file
    // the contents of the file are read and the functions defined inside
    // can be used similar to previous examples
    // This example based in part on 
    // https://python.plainenglish.io/using-python-in-rus-and-trust-in-python-ac5cf77d5ece
    pub fn python_function_from_file() -> PyResult<i32> {
        
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
            // first we need to grab the python code from a local file
            // Create a path to the desired file
            let code = get_py_file_contents("py/functions.py")?; 
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
            
            // create PyModule from contents of file
            // this is used to access individual functions separately
            let functions = PyModule::from_code(
                py,
                &code,
                "functions.py",
                "functions"
            ).unwrap();
    
            // grab the desired function using .getattr("function_name")
            // call the function using .call0() exe without args (or in this case use default args)
            println!("\nDemo#5.1 Call function without args\nEvaluating...\n-----start of py output-----\n");
            let add_function = functions.getattr("add_numbers").unwrap();
            let function_result = add_function.call0().unwrap(); // instead of unwrap, try to handle the pyresult directly?
            println!("\n-----end of py output-----\n");
            // the values we got from python can now be used in Rust
            println!("\nRust Output:\n\tThe sum is {}", function_result);
    
            // grab the desired function using .getattr("function_name")
            // then create some args and call the function using .call1(args)
            println!("\nDemo#5.2 Call function with PyTuple args\nEvaluating...\n-----start of py output-----\n");
            let add_function = functions.getattr("add_numbers").unwrap();
            let args = PyTuple::new(py, &[11,23]);
            let function_result = add_function.call1(args).unwrap().extract()?; // instead of unwrap, try to handle the pyresult directly?
            println!("\n-----end of py output-----\n");
            // the values we got from python can now be used in Rust
            println!("\nRust Output:\n\tThe sum is {}", function_result);
    
            //return the string result
            Ok(function_result)
        })
    }
    
    
    // Example 6
    // 
    pub fn python_function_err_handling()-> PyResult<i32> {
        
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
            // first we need to grab the python code from a local file
            // Create a path to the desired file
            let code = get_py_file_contents("py/functions.py")?; 
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
            
            // create PyModule from contents of file
            // this is used to access individual functions separately
            let functions = PyModule::from_code(
                py,
                &code,
                "functions.py",
                "functions"
            ).unwrap();
    
            // wrong type (float) args
            println!("\nDemo#6.1 Call function with wrong type args(f32)\nEvaluating...\n-----start of py output-----\n");
            let add_function = functions.getattr("add_numbers").unwrap();
            let args = PyTuple::new(py, &['a','b']);
            let function_result = add_function.call1(args).unwrap().extract()?; // instead of unwrap, try to handle the pyresult directly?
            println!("\n-----end of py output-----\n");
            
    
            // the values we got from python can now be used in Rust
            println!("\nRust Output:\n\tThe sum is {}", function_result);
    
            //return the result
            Ok(function_result)
        })
    }
    
    // Example 7a
    // Python functions that require packages installed on a virtual environment
    pub fn python_function_venv_a()-> PyResult<i32> {
        
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
    
            // first we need to grab the python code from a local file
            let code = get_py_file_contents("py/functions_venv.py")?; 
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
            
            // attempt create PyModule from contents of file
            // this module can be used to access individual functions separately
            let functions_pymodule: PyResult<&PyModule> = PyModule::from_code(
                py,
                &code,
                "functions.py",
                "functions"
            );
            
            // The next action to take depends on whether result of creating PyModule was OK or Err
            // if Ok, then load a function with .getattr(), create some args (if needed), and execute the function using .call0() or .call1()
            // if Err, then display the reason and return the relevant PyErr to main()
            // Err may occur for reasons such as:
            //  - Python cant compile because of syntax error in code
            //  - Python cant compile because imported libraries cant be found
            // ...etc. 
            match functions_pymodule {
                Ok(functions) => {
                    println!("\nResult: OK\nPython module was successfully created");
    
                    // Example 1: display emoji 
                    println!("\nDemo#7a 1 - Emoji\n");
                    
                    println!("Accessing function 'emoji_test()'");
                    let load_emoji_function = functions.getattr("emoji_test");
                    
                    // unpack Result of loading function and decide what to do next
                    // if Ok, call the function (nothing is returned)
                    // if Err, display the reason why (nothing is returned)
                    match load_emoji_function {
                        Ok(emoji_function) => {
                            // loaded successfully -> call the function and print whether the result was Ok or Err
                            println!("Successfully accessed function 'emoji_test()'");
                            println!("Evaluating...\n-----start of py output-----\n");
                            match emoji_function.call0() {
                                Ok(_) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("emoji_test() function call succeeded");
                                },
                                Err(pyerr) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("emoji_test() function call failed because: {}", pyerr);
                                },
                            };
                        },
                        Err(pyerr) => {
                            // failed to load function
                            println!("Failed to access function 'emoji_test' because: {}", pyerr);     
                        }
                    };
                  
                    // Example 2: Random Number 
                    println!("\n\nDemo#7a 2 - Random\n");
    
                    println!("Accessing function 'random_number()'");
                    let load_rand_function = functions.getattr("random_number");
                    
                    
                    // unpack Result of loading function and decide what to do next
                    // if Ok, create some args and call the function with them. If the call succeeds, the value is returned
                    // if Err, display the reason why and return the relevent PyErr 
                    // Err may occur for reasons such as:
                    //  - no function by that name (ex. incorrect spelling)
                    match load_rand_function {
                        Ok(rand_function) => {
                            println!("Successfully accessed function 'random_number()'");
                            
                            // random number test 1
                            let args = PyTuple::new(py, &[10, 20]);
                            println!("\nTesting function using args {:?}", args);
                            
                            println!("Evaluating...\n-----start of py output-----\n");
                            
                            // call the function and decide what to do next based on Result
                            // if Ok, convert the new PyAny value to i32 and return it
                            // if Err, display the reason why and return the relevent PyErr 
                            // Err may occur for reasons such as:
                            //  - Supplied arguments are the wrong type
                            //  - Supplied arguments have invalid value(s) (ex. lower_limt > higher_limit)
                            // ...etc. 
                            match rand_function.call1(args) {
                                Ok(num) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call succeeded");
                                    println!("A random number from Python: {}\n", num);
                                    let num = num.extract()?;
                                    return Ok(num);
                                },
                                Err(pyerr) if pyerr.is_instance_of::<PyTypeError>(py) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call failed because PyTypeError: {}", pyerr);
                                    return Err(pyerr);
                                },
                                Err(pyerr) if pyerr.is_instance_of::<PyValueError>(py) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call failed because PyValueError: {}", pyerr);
                                    return Err(pyerr);
                                },
                                Err(pyerr) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call failed because unspecified Python error: {}", pyerr);
                                    return Err(pyerr);
                                },
                            };
                        },
                        Err(pyerr) => {
                            println!("Failed to access function 'random_number()' because: {}", pyerr);   
                            return Err(pyerr);   
                        }
                    }
                },
    
                Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                    println!("\nResult: ERR (Syntax Error) \nPython module could not be created"); 
                    return Err(pyerr);
                },
                Err(pyerr) if pyerr.is_instance_of::<PyModuleNotFoundError>(py) => {
                    println!("\nResult: ERR (ModuleNotFoundError) \nPython module could not be created"); 
                    return Err(pyerr);
                },
                Err(pyerr) => {
                    println!("\nResult: ERR (Unspecified Error)\nPython module could not be created"); 
                    return Err(pyerr);
                },
            };     
        })
    }
    
    
    // Example 7b
    // Python functions that require packages installed on a virtual environment
    pub fn python_function_venv_b()-> Result<i32, Error> {
        
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
    
            // first we need to grab the python code from a local file
            let code = get_py_file_contents("py/functions_venv.py")?; 
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
            
            // attempt create PyModule from contents of file
            // this module can be used to access individual functions separately
            let functions_pymodule: Result<&PyModule, PyErr> = PyModule::from_code(
                py,
                &code,
                "functions.py",
                "functions"
            );
            
            // The next action to take depends on whether result of creating PyModule was OK or Err
            // if Ok, then load a function with .getattr(), create some args (if needed), and execute the function using .call0() or .call1()
            // if Err, then display the reason and return the relevant PyErr to main()
            // Err may occur for reasons such as:
            //  - Python cant compile because of syntax error in code
            //  - Python cant compile because imported libraries cant be found
            // ...etc. 
            match functions_pymodule {
                Ok(functions) => {
                    println!("\nResult: OK\nPython module was successfully created");
    
                    // Example 1: display emoji 
                    println!("\nDemo#7b 1 - Emoji\n");
                    
                    println!("Accessing function 'emoji_test()'");
                    let load_emoji_function = functions.getattr("emoji_test");
                    
                    // unpack Result of loading function and decide what to do next
                    // if Ok, call the function (nothing is returned)
                    // if Err, display the reason why (nothing is returned)
                    match load_emoji_function {
                        Ok(emoji_function) => {
                            // loaded successfully -> call the function and print whether the result was Ok or Err
                            println!("Successfully accessed function 'emoji_test()'");
                            println!("Evaluating...\n-----start of py output-----\n");
                            match emoji_function.call0() {
                                Ok(_) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("emoji_test() function call succeeded");
                                },
                                Err(pyerr) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("emoji_test() function call failed because: {}", pyerr);
                                },
                            };
                        },
                        Err(pyerr) => {
                            // failed to load function
                            println!("Failed to access function 'emoji_test' because: {}", pyerr);     
                        }
                    };
                  
                    // Example 2: Random Number 
                    println!("\n\nDemo#7b 2 - Random\n");
    
                    println!("Accessing function 'random_number()'");
                    let load_rand_function = functions.getattr("random_number");
                    
                    
                    // unpack Result of loading function and decide what to do next
                    // if Ok, create some args and call the function with them. If the call succeeds, the value is returned
                    // if Err, display the reason why and return the relevent PyErr 
                    // Err may occur for reasons such as:
                    //  - no function by that name (ex. incorrect spelling)
                    match load_rand_function {
                        Ok(rand_function) => {
                            println!("Successfully accessed function 'random_number()'");
                            
                            // random number test 1
                            let args = PyTuple::new(py, &[10, 20]);
                            println!("\nTesting function using args {:?}", args);
                            
                            println!("Evaluating...\n-----start of py output-----\n");
                            
                            // call the function and decide what to do next based on Result
                            // if Ok, convert the new PyAny value to i32 and return it
                            // if Err, display the reason why and return the relevent PyErr 
                            // Err may occur for reasons such as:
                            //  - Supplied arguments are the wrong type
                            //  - Supplied arguments have invalid value(s) (ex. lower_limt > higher_limit)
                            // ...etc. 
                            match rand_function.call1(args) {
                                Ok(num) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call succeeded");
                                    println!("A random number from Python: {}\n", num);
                                    let num:i32 = num.extract()?;
                                    return Ok(num);
                                },
                                Err(pyerr) if pyerr.is_instance_of::<PyTypeError>(py) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call failed because PyTypeError: {}", pyerr);
                                    return Err(Error::new(ErrorKind::InvalidInput, pyerr));
                                },
                                Err(pyerr) if pyerr.is_instance_of::<PyValueError>(py) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call failed because PyValueError: {}", pyerr);
                                    return Err(Error::new(ErrorKind::InvalidInput, pyerr));
                                },
                                Err(pyerr) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("random_number() function call failed because unspecified Python error: {}", pyerr);
                                    return Err(Error::new(ErrorKind::Other, pyerr));
                                },
                            };
                        },
                        Err(pyerr) => {
                            println!("Failed to access function 'random_number()' because: {}", pyerr); 
                            return Err(Error::new(ErrorKind::Other, pyerr));
        
                        }
                    }
                },
                Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                    println!("\nResult: ERR (InvalidInput) \nPython module could not be created due to syntax error"); 
                    return Err(Error::new(ErrorKind::InvalidInput, pyerr));
                },
                Err(pyerr) if pyerr.is_instance_of::<PyModuleNotFoundError>(py) => {
                    println!("\nResult: ERR (NotFound) \nPython module could not be created because something wasnt found"); 
                    return Err(Error::new(ErrorKind::NotFound, pyerr));
                },
                Err(pyerr) => {
                    println!("\nResult: ERR (Unspecified Error)\nPython module could not be created"); 
                    return Err(Error::new(ErrorKind::Other, pyerr));
                },
            };     
        })
    }
    
    
    // Example 7c
    // Python functions that require packages installed on a virtual environment
    // fn python_function_venv_c()-> Result<Option<String>, Error> {
    pub fn python_function_venv_c(color_name:&str)-> Result<Option<char>, Error> {
        // Initialize Python interpreter and acquire Global Interpreter Lock
        println!("\nInitializing py interpreter...");
        Python::with_gil(|py| {
    
            // first we need to grab the python code from a local file
            let code = get_py_file_contents("py/functions_venv.py")?; 
            println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
            
            // attempt create PyModule from contents of file
            // this module can be used to access individual functions separately
            let functions_pymodule: Result<&PyModule, PyErr> = PyModule::from_code(
                py,
                &code,
                "functions.py",
                "functions"
            );

            let args = PyTuple::new(py, &[color_name]);
            let out:Option<char> = match functions_pymodule?.getattr("color_emoji")?.call1(args) {
                Ok(n) => {
                    // python function completd successfully 
                    // need to check if Some or None
                    let cha:char = n.extract()?;
                    if cha=='0' {
                        return Ok(None);
                    } else {
                        return Ok(Some(cha));
                    }
                },
                Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                    println!("\nResult: ERR (InvalidInput) \nPython module could not be created due to syntax error"); 
                    return Err(Error::new(ErrorKind::InvalidInput, pyerr));
                },
                Err(e) => {return Err(Error::new(ErrorKind::Other, e));}
            };

     
            /* 
            // The next action to take depends on whether result of creating PyModule was OK or Err
            // if Ok, then load a function with .getattr(), create some args (if needed), and execute the function using .call0() or .call1()
            // if Err, then display the reason and return the relevant PyErr to main()
            // Err may occur for reasons such as:
            //  - Python cant compile because of syntax error in code
            //  - Python cant compile because imported libraries cant be found
            // ...etc. 
            match functions_pymodule {
                Ok(functions) => {
                    println!("\nResult: OK\nPython module was successfully created");
    
                    // Example 1: display emoji 
                    println!("\nDemo#7c 1 - Emoji\n");
                    
                    println!("Accessing function 'color_emoji()'");
                    let load_emoji_function = functions.getattr("color_emoji");
                    
                    // unpack Result of loading function and decide what to do next
                    // if Ok, call the function and return the value as an Option
                    // if Err, display the reason why and return the Error 
                    match load_emoji_function {
                        Ok(emoji_function) => {
                            // loaded successfully -> call the function and print whether the result was Ok or Err
                            println!("Successfully accessed function 'color_emoji()'");
                            
                            // create args to supply to function
                            let args = PyTuple::new(py, &[color_name]);
                            println!("\nTesting function using args {:?}", args);
                            
                            // call the function and choose next actions based on Result Ok or Err
                            println!("Evaluating...\n-----start of py output-----\n");
                            match emoji_function.call1(args) {
                                Ok(output) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("color_emoji() function call succeeded");
    
                                    // python result is returned as an option which pyo3 doesn't recognize
                                    // need to extract the value manually
                                    println!("function returned: {:?}\n", output);
                                    let out = extract_py_emoji_optional_output(output);
                                
                                    return Ok(out);
                                },
                                Err(pyerr) => {
                                    println!("\n-----end of py output-----\n");
                                    println!("color_emoji() function call failed because: {}", pyerr);
                                    return Err(Error::new(ErrorKind::Other, pyerr));
                                },
                            };
                        },
                        Err(pyerr) => {
                            // failed to load function
                            println!("Failed to access function 'color_emoji' because: {}", pyerr); 
                            return Err(Error::new(ErrorKind::Other, pyerr));    
                        }
                    };
                  
                },
                Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                    println!("\nResult: ERR (InvalidInput) \nPython module could not be created due to syntax error"); 
                    return Err(Error::new(ErrorKind::InvalidInput, pyerr));
                },
                Err(pyerr) if pyerr.is_instance_of::<PyModuleNotFoundError>(py) => {
                    println!("\nResult: ERR (NotFound) \nPython module could not be created because something wasnt found"); 
                    return Err(Error::new(ErrorKind::NotFound, pyerr));
                },
                Err(pyerr) => {
                    println!("\nResult: ERR (Unspecified Error)\nPython module could not be created"); 
                    return Err(Error::new(ErrorKind::Other, pyerr));
                },
            }; */    
        })
    }
    

    
    
    // Example 7d
    // Python functions that require packages installed on a virtual environment
    // fn python_function_venv_c()-> Result<Option<int>, Error> {
        pub fn python_function_venv_d(upper_limit:i32, lower_limit:i32)-> Result<Option<i32>, Error> {
            // Initialize Python interpreter and acquire Global Interpreter Lock
            println!("\nInitializing py interpreter...");
            Python::with_gil(|py| {
        
                // first we need to grab the python code from a local file
                let code = get_py_file_contents("py/functions_venv.py")?; 
                println!("\nPython code to evaluate:\n-----start of py code-----\n\n{code}\n\n-----end of py code-----");
                
                // attempt create PyModule from contents of file
                // this module can be used to access individual functions separately
                let functions_pymodule: Result<&PyModule, PyErr> = PyModule::from_code(
                    py,
                    &code,
                    "functions.py",
                    "functions"
                );

                let args = PyTuple::new(py, &[lower_limit, upper_limit]);
                println!("\nEvaluating python code using args {args:?}...\n-----start of py output-----\n");
                let out:Option<i32> = match functions_pymodule?.getattr("random_number")?.call1(args) {
                    Ok(n) => {
                        // python function completed successfully 
                        println!("\n-----end of py output-----\n");
                        println!("random_number() function call succeeded");
                        // need to decide if value correponds to Some or None
                        let num:i32 = n.extract()?; //change python int to rust i32
                        if num & 1 == 1 {
                            println!("The random number {num} is ODD - return Some");
                            Some(num)
                        } else {
                            println!("The random number {num} is EVEN - return None");
                            None
                        }
    
                    },
                    Err(pyerr) if pyerr.is_instance_of::<PySyntaxError>(py) => {
                        println!("\nResult: ERR (InvalidInput) \nPython module could not be created due to syntax error"); 
                        return Err(Error::new(ErrorKind::InvalidInput, pyerr));
                    },
                    Err(e) => {return Err(Error::new(ErrorKind::Other, e));}
                };
    
                // do some math on out, you could use match out {} or out.map()
                // return the result after this math instead of in the previous match
                println!("\nProcessing Option(num) from Python in Rust...");
                println!("Double number if it is ODD, do nothing if number is EVEN...");
                let out = out.map(|n| n*2);
                println!("\nProcessed result: {:?}", &out);

                Ok(out)
             
            })
        }
        
    
        
    // Misc Helper functions
    
    pub fn display_package_info() {
        println!("\nRunning PIP to see packages...");
    
        println!("\nLocal Packages:");
        let output = Command::new("bash")
            .arg("-c")
            .arg("pip3 freeze --local")
            .output()
            .expect("bash command failed");
        println!("pip3 freeze --local:\tStatus: {:?}",output.status);
        match output.stdout.len() {
            0 =>println!("No virtual environment detected\nNo packages to display"),
            _=> {
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    
    }
    
    
    // path should meet the following requirements:
    //  - path is valid and exists
    //  - path leads to a file with .py extension
    //  - the file can be read and its contents are not empty
    fn validate_py_path(path:&Path) -> Result<(), String> {
        // confirm path exists and is a file rather than a directory
        if !path.exists() { 
            return Err("Path does not exist".to_owned());    }
        if !path.is_file() { 
            return Err("Path does not lead to a file (maybe a directory?)".to_owned());   }
    
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
            },
            None => {
                println!("c");
                Err("path.extension() failed--maybe path does not have a period delimiting the extension?".to_owned())
            },
    
        }
    }
    
    fn get_py_file_contents(file_name:&str) -> Result<String, Error> {
    
        // Create a path to the desired file
        println!("Opening file: {}", &file_name);
        let path = Path::new(file_name);
    
        // validate path
        let path_validation =  validate_py_path(&path);
        if path_validation.is_err() {
            return Err(Error::new(ErrorKind::Other, path_validation.err().unwrap_or("Could not validate path".to_owned())));
        }
    
        // try to open the file
        let file = File::open(&path);
        
        // match on file to examine result of open operation
        let result:Result<String, Error> = match file {
            Ok(mut file) => {
                println!("File was opened successfully");
    
                // create a new String and read the file contents into it
                let mut s = String::new();
                file.read_to_string(&mut s)?;
                
                // make sure file is not empty
                if s.is_empty() {
                    return Err(Error::new(ErrorKind::Other, String::from("Py file was empty...")));
                }
                // finished inner actions for successful file read--return file content string inside Result
                Ok(s.to_owned())
            },
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
    
    // convert python type "Optional" into Rust type "Option"
    fn extract_py_emoji_optional_output(py_option:&PyAny) -> Option<char> {
        println!("Converting Py Option to Rust...");
        
        // TO-DO: replace unwrap()s used to unpack getattr() and call0()
        match py_option.getattr("is_present").unwrap().call0().unwrap().to_string().as_str() {
            "True" => {
                // is_present == True
                println!("Sucess -> 'SOME'");
                let value = py_option.getattr("get").unwrap().call0().unwrap().extract().unwrap();
               
                return Some(value);
            },
            "False" => {
                // is_present == False
    
                println!("Sucess -> 'NONE'");
                return None;
            },
            _ => {
                println!("Failure -> Something went wrong");
                return None;
            },
        }
    }
    
    pub fn get_user_input() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_owned()
    }

    pub fn get_user_confirmation() -> bool {
        loop {
            println!("Yes or No?");
            
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            
            match input.trim().to_ascii_uppercase().as_str() {
                "YES" | "Y" | "TRUE" | "t" => {
                    println!("Continue");
                    break true;
                },
                "NO" | "N" | "FALSE" | "F" => {
                    println!("Do not Continue");
                    break false;
                },
                _ => {
                    println!("Error, invalid response. Please try again");
                    continue;
                },
            };
        }
    } // end of fn get_user_confirmation()