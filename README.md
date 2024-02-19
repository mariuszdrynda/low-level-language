# low-level-language
Low Level Language is an intermediate representation for C programming language

# Goal

Goal is to create as simple as possible programming language that could serve as intermediate representation for other, higher level function. This is a way to demonstrate that all (or almost all) of languages features could be replaced by simple, but powerfull mechanisms.

Second, and more impotant, is to study different possibilities of optimization and safety

# Principal rules

+ Only one file as code input
+ No infix operators (only functions)
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

## Types

### Build-in types

### Undefined

## Match

Match is an expression.

## Flow

### Jumps

### Return

Function return accepts only one argument which has to be the same type as the one declared 

### Drop

## Global variables

## Functions

Everything in passed to function by value (copyied). If you want to move by reference you have to explicitly pass pointer.

### Command line arguments

### Function pointers

Function pointers are treated like any other pointer.

## Structs and unions

## STD

### List of build-in functions

## Platform-depended API

### Linux

## Compiler options

Replacement for preprocessor.
There are needed for building platform-independent code/libraries.

# Effects

TLDR: takes program as an input and tells you where it can fail

# Supported backends

LLL is designed as a backend language. When it's finished it will generate executables, but for now it will use other language as backend (LLVM IR, QBE, etc., still need to figure out which one)

# What and why this language doesn't support?

List of not supported features:
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
