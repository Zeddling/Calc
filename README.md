#Calc
Calc is an intepreted toy programming language created to learn compiler construction basics with rustup.
The language can be used to perform basic arithmetic operations.
References: <a href="https://github.com/PacktPublishing/Creative-Projects-for-Rust-Programmers/tree/master/Chapter08">Creative Projects for Rust Programmers</a>

##Documentation
###Translation and using the interactive shell
####Interactive Shell
To run the interactive shell, enter `cargo run`.
\> is the code input mode.
? is the input prompt.

#####Basic commands
`q` - quits the shell
`c` - Clears all variables from memory
`v` - Displays all variables and their respective values

####Translator
To run the translator, enter `cargo run /path/to/file`.
The .calc file will be translated to an output.rs file which will be found in the calc\_target/ directory.
The file can be compiled using `rustc /path/to/file`.

###Syntax
@ declares a variable. Only the f64 data type is supported.
\> is an input prompt
< is an output prompt
A variable is assigned using the ":=" operator
Addition, subtraction, multiplication and division currently are the supported operations.
Whitespaces and newline characters are not needed

####Example
```
@a
@b
>a
>b
@total := a * ( b - 2 * (1 / a))
<total
```
The expression `x(y)` is not valid.
