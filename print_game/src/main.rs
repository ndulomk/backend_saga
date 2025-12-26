/*
THE PRINTLN MACRO

*Macros expand into additional code
*println "Prints" (displays) information to the terminal
*Useful for debugging
*/

static life: i32 = 42;
println!("hello");
println!("{:?}", life); 
println!("{:?} {:?}", life, life);
println!("the meaning is {:?}", life)




// {:?} is for debugging without it is just for showing uses