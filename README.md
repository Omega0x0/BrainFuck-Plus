# BrainFuck Plus (BFP/BF+)
BrainFuck Plus is a small offshoot of the original BF, with minor changes...

## What can BFP do?
***
BFP - can do almost everything that the original BF, BF has several differences from BF:
1. Its own file format `.bfp`
2. Slightly modified syntax, `[...]` - now not a loop, but is a block code
3. Its project structure
4. Interpreter in the Rust programming language
5. Several new commands (`$, !, @, *, ;`)
6. Your own plugin for VS Code What can BFP do?What can BFP do?

[Hello world](./examples/hello%20world/)

## Syntax features
***
`[...]` - can be executed just like that, i.e. without `@` and `$`. </br>
`@` and `$` will give an error if there is no `[...]` after them.
## Constructions:
***
```
[...];      Code block.
@[...];     Creating a loop.
$[...];     Creating a function.
*;          Function call.
!;          Debug call.
;;          Comment.
```
## Commands:
***
|Commands|             Description              |
|:-----:|:-------------------------------------:|
|  `@`  | Creates a loop from a block of code,</br>what follows next,</br>works about the same,</br>as in classic BF|
|  `!`  | Debug outputs the</br> cell value and printer position to the console|
|  `;`  | The usual line comment|
|  `$`  | Creates a function from a further block of code...</br>The function gets the name from the 4 bytes of the array, </br> that follow each other including the cell, </br> where the pointer is located|
|  `*`  | Calls a function, </br> addressing by name is similar to creating a name in `$`|