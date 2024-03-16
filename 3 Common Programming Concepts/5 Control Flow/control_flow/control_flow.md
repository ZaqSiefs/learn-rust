# Control Flow

The ability to run some code depending on whether a condition is true, and to run some code repeatedly while a condition is true.

The most common constructs to control the flow of execution are ```if``` expressions and loops

## if Expressions

Allows you to branch code depending on conditions.

Provide a condition and state "If this condition is met, run this block of code.  If the condition is not met, do not run the block of code".

Blocks of code associated with the conditions of if expressions are sometimes called arms.

The else expression can chose to be used to provide the program with an alternative block of code to execute if the condition evaluates to false.

Rust does not implicitly convert non-bools to bools, so if must always be evaluated with a boolean value.

using too many else if statements clutter the code, so if there are more than one, you probably want to refactor

## Loops

The ```loop``` keyword tells Rust to execute a block of code over and over again forever or until explicitly toled to stop.

Loop labels allow you to apply break and continue to nesting loops.

```while``` will loop through a block until a condition is false.

```for``` loops through a collection of elements