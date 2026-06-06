// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    let size_kb_pdb = 1265664/1024;
    let size_mb_pdb = 1265664/1024/1024;
    println!("{} kb",size_kb_pdb);
    println!("{} mb",size_mb_pdb);
}
/*
println! is a macro 
that prints text to the console 
appended with a newline. 
The ! indicates that it is a macro, 
not a function.

A binary can be generated using the Rust compiler: rustc.

$ rustc hello.rs

rustc will produce a hello binary that can be executed.

$ ./hello
Hello World!

Activity

Click ‘Run’ above to see the expected output. Next, add a new line with a second println! macro so that the output shows:

Hello World!
I'm a Rustacean!

*/
