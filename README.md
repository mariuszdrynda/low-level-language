# low-level-language
Low Level Language is an intermediate representation for C programming language

# Goal

Goal is to create as simple as possible programming language that could serve as intermediate representation for other, higher level function. This is a way to demonstrate that all (or almost all) of languages features could be replaced by simple, but powerfull mechanisms.

Second, and more impotant, is to study different possibilities of optimization and safety

# Principal rules

+ Only one file as code input (one translation unit)
+ No infix operators (only functions)
+ All functions are global
+ No hidden control flow (a program does only what we intended and nothing more)

This rules could change as language will evolve.

# Current status

# Language description

## Hello World

```
fn main() -> Int{
    drop = c.stdio.printf("Hello World!\n");
    return(0);
}
```
Function "main" is a startpoint of the program.

## Variables

### Booleans

### Integers

### Floats

### Chars

### Strings

### Pointers

Pointers are simply integers sufficiently wide to represent all memory addresses.

```
// C
int a = 5;
int* b = &a;
*b = 7;

# LLL
a = I64(5);
b = addr(a); #:Ptr<I64>
Ptr.set(b, 7);
```

## Types

Types are nothing more than information what is the minimum size of the variable.

```
boolean: 1 = True;
pointer: 64 = Ptr.deref(boolean)
```

### Build-in types

### Undefined

## Match

Match is an expression.

## Flow

### Jumps

Jumps works only within the same function.

### Return

Function return accepts only one argument which has to be the same type as the one declared 

### Drop

## Global variables

Every global variable needs to be initialized. All global variables are availble all the time from any place in code.

## Functions

Everything in passed to function by value (copyied). If you want to move by reference you have to explicitly pass pointer.

Functions can not be nested.

### Command line arguments

### Function pointers

Function pointers are treated like any other pointer.

```
fn add_5(a: Int)->Int{
    Flow.return(add(a, 5));
}
fn main()->Int{
    func = add_5;
    result: Int = func(3); # it's user 
    Flow.return(0);
}
```

## Structs and unions

## STD

STD is divided into two main parts: build-ins and platform-dependent API. Build-in are available on all supported architectures (also for standalone programs).

### List of build-in functions

List of all build-in functions. In this section we're going to use following convenction:
```
function_name(arg1:ParametrizedType(Param1, Param2), arg2:Type) ReturnedType \ possible_side_effect
```

+ Flow.goto(Label) \ infinite_loop # if we jumping backwards
+ Flow.label() -> Label
+ Flow.nop()
+ Flow.return(Any) 
+ Flow.unreachable() \ panic
+ Ptr.set(a:Ptr(A), b:A) Unit \ nullptr_deref # if(eq(a, 0))

### List of build-in types

+ Unit # type with only one possible value. Used when function doesn't return any value. Similar to C's void

## Platform-depended API

Every supported architecture could have set of functions and variables available only on this specific architecture, for example sys calls.

### Linux_x64

## Compiler options

Replacement for preprocessor.
There are needed for building platform-independent code/libraries.

# Effects

TLDR: takes program as an input and tells you where it can fail.
List of effects:
+ infinite_loop
+ nullptr_deref
+ panic

# Supported backends

LLL is designed as a backend language. When it's finished it will generate executables, but for now it will use other language as backend (LLVM IR, QBE, etc., still need to figure out which one)

# What and why this language doesn't support?

List of not supported features:
- function declaration
- enum
- exceptions, try-catch blocks
- const
- tuples
- modules and namespaces
- defer
- references
- tests, assertions
- loops: for, while, do..while, break, continue
- if..else
- casting
- preprocessor
- variadic functions
- std: io, files, threads, networking, containers

### Casting

```
//C
float add(float a){
    return a+5.0;
}
int main(){
    int a = 5;
    float b = add(a); // ok, a is casted to float
    printf("{%f}\n", b);
    getchar();
    return 0;
}
```
For LLL only matters that argument we're passing to "add" function has 32 bits and "add" function accepts 32 bits arguments as well (doesn't matter it's not the same type).
