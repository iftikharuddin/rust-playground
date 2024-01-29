# Rust

- developed by Firefox
- open source
- 2 ways to deploy rust code
    - cargo (kinda npm)
    - native rust compiler
- for creating new project run this cmd
    - cargo new `project name`
- using variables in console print
    - println!("Hello, world! Back to rust again ;) {} {}", var1, var2);
    - unsigned - can't be negative
    - signed - can be negative
    - the type `i8` whose range is `-128..=127`
    - the type `u8` whose range is `0..=255`
    - data types
- Arrays
    - array
    - slice (mostly slices are passed from frontend)
    - Slices are similar to arrays, but their length is not known at compile time.
    - Instead, a slice is a two-word object; the first word is a pointer to the data, 
    - the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture,
    - e.g. 64 bits on an x86-64.
    

- functions
    - all funcs are private by default
- Mutability
    - vars are immutable by default
    - for making it mutable use mut keyword
- Strings
    - string.push, push_str
    - string.replace
- If statements
- loops
    - for i in  0..6 { println!("test"); }
    - while loop
    -  let mut i = 0; while i < 4 {i++}
    - break/continue
- Match statements 
    - kinda switch in other langs
    - match i
- Struct
- Traits
    - Rust don't support inheritance! However it only supports interfaces 
    - 
- ENUM
- Vector
- Hash Map
- Options
- Results



# Rust for ETH

- RETH Client
- Ethers RS