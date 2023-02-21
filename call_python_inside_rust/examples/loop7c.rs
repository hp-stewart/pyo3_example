/*
This example runs function 7c in a loop so you can quickly try out different inputs and see the result

Expected behavior:
- Prompt the user to type a color
- call the function from example 7c in lib, while supplying the users text input 
- inside the example 7c function, the pyo3 crate is used to create a Python session inside Rust
    - The return value of this function is in the form Result<Option<char>, Error>
    - the Python Module is created from the contents of py/functions_venv.py file
    - the Python function "def color_emoji()" is loaded from the PyModule
    - color_emoji() is executed using the users text input
        - an error is raised if the text input contains a number
        - the supplied color name is converted into the appropriate CLDR shortname to display a colored circle
            - ex: "Red" into ":red_circle:"
        - attempt to convert the CLDR shortname into an emoji using the python emoji module
            - If the CLDR shortname matches a known emoji, the operation completes and color_emoji() returns Some(emoji)
            - If the CLDR shortname does not match any emoji, the operation completes and color_emoji() returns None  
    - the rust example function receives a Result containing either OK(PyAny) (if the operation completed) or else PyErr
    - this result is converted into Rust-compatible types and then returned to main()
        - If OK: The PyAny is a python Optional which can manually converted to Rust Option Enum and returned inside Rust Result Enum
        - If PyErr: convert into Rust Err and return inside Rust Result Enum
    - main() receives the Result<Option<char>,Error> returned by the example fnuction and displays the results
    - the user is prompted to either try again or quit
    - based on user input, either the loop restarts or ends
 */

use examples;
fn main() {
    loop {
        println!("\n\n---------------------------------------------------------------------------\n---------------------------------------------------------------------------\n");
        
        // user input selects a color to be displayed
        println!("Enter a color to be Displayed: ");
        let input = examples::get_user_input();

    
        // run the example
        println!("\nExample 7c: functions requiring packages installed on venv -- returns Result<Option<char>, Error>");
        let _r7c = match examples::python_function_venv_c(&input) {
            Ok(n) =>     println!("\nPy Function 7c success!! \nThe result was Ok({n:?}) \n"),
            Err(e) =>     println!("\nPy Function 7c failed because {e}...\n"),
        };
    
        println!("\nEnd\n--------------------------------------------------\n");
    
        // try again or quit
        println!("\nExample complete...do you want to start again?");
        if examples::get_user_confirmation() {
            println!("Restarting");
            continue;
        } else {
            println!("Goodbye");
            break;
        }

    }
    
}


