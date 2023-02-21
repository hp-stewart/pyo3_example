/*
This example runs function 7d in a loop so you can quickly try out different inputs and see the result

Expected behavior:
- call the function from example 7d in lib
- inside the example 7d function, the pyo3 crate is used to create a Python session inside Rust
    - The return value of this function is in the form Result<Option<i32>, Error>
    - the Python Module is created from the contents of py/functions_venv.py file
    - the Python function "def random_number()" is loaded from the PyModule
    - random_number() is executed using the range 1, 10
        - a random number from 1 to 10 is generated and returned  
    - the rust example function receives a Result containing either OK(PyAny) (if the operation completed) or else PyErr
        - If OK: The PyAny is converted into Rust i32 and returned inside an Option
            - The option is Some if the number is Odd and None if the number is even
        - If PyErr: convert into corresponding Rust Err and return inside Rust Result Enum
    - Some simple arithmatic is performed on the value inside the Option (if it is not none) and it is returned to main() inside a Result 
    - main() receives the Result<Option<int>,Error> returned by the example fnuction and displays the results
    - the user is prompted to either try again or quit
    - based on user input, either the loop restarts or ends
 */

 use std::io;

 use examples;
 fn main() {
     loop {
         println!("\n\n---------------------------------------------------------------------------\n---------------------------------------------------------------------------\n");
         
 
         // run the example
         println!("\nExample 7d:  returns Result<Option<i32>, Error>");
         let _r7c = match examples::python_function_venv_d(10, 1) {
             Ok(n) =>     println!("\nPy Function 7c success!! \nThe result was Ok({n:?}) \n"),
             Err(e) =>     println!("\nPy Function 7c failed because {e}...\n"),
         };
     
         println!("\nEnd\n--------------------------------------------------\n");
     
         // try again or quit
         println!("\nExample complete...do you want to start again?");
         if get_user_confirmation() {
             println!("Restarting");
             continue;
         } else {
             println!("Goodbye");
             break;
         }
 
     }
     
 }
 
 
 fn get_user_confirmation() -> bool {
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
 