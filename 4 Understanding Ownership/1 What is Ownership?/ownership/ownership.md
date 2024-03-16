# Ownership

A set of rules that govern how a Rust program manages memory.
Some languages use garbage collectors that regularly look for no-longer-used memory during runtime.
In other languages, the programmer must explicitly allocate and deallocate the memory
Rust is unique in that memory is managed trhough a system of ownership with a set of rules that the compiler checks.
If any of these rules are violated, the program simply will not compile.
None of these features will slow the program down when running like a garbage collector, and you don't have to manually allocate and deallocate memory.

Fighting with the Borrow checker is a key feature of rust that new programmers will have to get used to.

## Stack and Heap

Rust is a systems programming language where the location of a value in memory matters in how the language behaves.

Both stack and heap are parts of memory available for the code to use at runtime, but they are structured very differently.

Pushing to the stack is faster than allocating on the heap since the allocater never has to search for a place to store new data.
Accessing data on the heap is slower than on the stack because you have to follow a pointer to get there.

When code calls a function, the values passed into the function and the function's local variables get pushed onto the stack. When the function is over, they get popped off the stack.

### Stack

The stack stores values in the order it gets them, and removes them in opposite order.
This is referred to as Last In First Out (LIFO). 
Adding data to the stack is called pushing, while removing data is called popping.
All data must have a known, fixed size.

### Heap

When putting data on the heap, we request a certain amount of space.
The memory allocater finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer to this spot in memory.
Because the pointer is a known, fixed size, this pointer can be stored on a stack.
You must access the data on the heap through the pointer.

## Ownership Rules

- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

## String Type

Stored on the heap

String literals are immutable and size must be known by compile time

```String``` strings are mutable and size can be unknown at compile time

The memory allocated for ```String``` is only valid for the scope it was called in, and it is deallocated once it goes out of scope.

When we make another variable and copy the String into it, it only copies the pointer and size data.
Also, it renders the previous variable unusable so that there is no risk of a double deallocation at the end of the variables lives.
This is simmilar to a shallow copy, but also invalidates the previous variable

the .clone method allows for us to make a deep copy of a String to another String.

## Stack Data: Copy

```Copy``` is an annotation that can be placed on stack-types.  
If a type implements the ```Copy``` trait, variables that use this type do not move, but are rather trivially copied, making them still valid after assignment to another variable

Rust wont let us use the annotation if the type of any of its parts implement the Drop trait.
If the type needs something special to happen when the value goes out of scope and the ```Copy``` annotation is added, we'll get a compile error.

As a general rule, any group of simple scalar values can implement ```Copy```, and nothing that requires allocation or is some form of resource can. 

### List of types that implement ```Copy```

- All integer types
- bool
- All float types
- char
- tuples, if they only contain types above

## Ownership and functions

Passing a value to a function are similar to when assigning a value to a variable.
Passing a variable to a function will move or copy.

## Return Values and Scope

Returning values can also transfer ownership.
