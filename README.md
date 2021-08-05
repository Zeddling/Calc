# Calc
Calc is an intepreted toy programming language created to learn compiler construction basics with rustup.
<br>The language can be used to perform basic arithmetic operations.
<br>References: <a href="https://github.com/PacktPublishing/Creative-Projects-for-Rust-Programmers/tree/master/Chapter08">Creative Projects for Rust Programmers</a>

## Documentation
### Translation and using the interactive shell
#### Interactive Shell
To run the interactive shell, enter `cargo run`.<br>
\> is the code input mode.<br>
? is the input prompt.

##### Basic commands
`q` - quits the shell.<br>
`c` - Clears all variables from memory.<br>
`v` - Displays all variables and their respective values.<br>

#### Translator
To run the translator, enter `cargo run /path/to/file`.<br>
<br>The .calc file will be translated to an output.rs file which will be found in the calc\_target/ directory.
<br>The file can be compiled using `rustc /path/to/file`.

### Syntax
@ declares a variable. Only the f64 data type is supported.<br>
\> is an input prompt.<br>
< is an output prompt.<br>
A variable is assigned using the ":=" operator.
Addition, subtraction, multiplication and division currently are the supported operations..<br>
Whitespaces and newline characters are not needed<br>

#### Example
```
@a
@b
>a
>b
@total := a * ( b - 2 * (1 / a))
<total
```
The expression `x(y)` is not valid.<br>
