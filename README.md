# low-level-language
Low Level Language is an intermediate representation for higher level languages.

# Goal

Goal is to create as simple as possible programming language that could serve as intermediate representation for other, higher level programming languages. This is a way to demonstrate that all (or almost all) of languages features could be replaced by simple, but powerfull mechanisms.

Second, and more impotant, is to study different possibilities of optimization and safety

# Principal rules

+ Only one file as code input (one translation unit)
+ No infix operators (only functions)
+ All functions are global
+ No hidden control flow (a program does only what we intended and nothing more)

This rules could change as language will evolve.

# Current status

# Language description

Every program is composed of global values (functions, structs, unions and global variables). 

## Hello World

```
fn main() -> Int{
    c.stdio.printf("Hello World!\n");
    return(0);
}
```
Function "main" is a startpoint of the program.

## Variables

### Booleans

There are two possible boolean values: true and false;

### Integers

### Floats

Floating-point numbers are represented using the single-precision and double-precision formats of the IEEE 754 standard.

### Chars

### Strings

String is an array of characters.

### Pointers

Pointers are simply integers sufficiently wide to represent all memory addresses. Pointers remembers type of variable they're pointing to, but this information is available only in compile time.
```
// C
int a = 5;
int* b = &a;
*b = 7;

# LLL
a = I64(5);
b = Ptr.addr(a); #:Ptr(I64)
Ptr.set(b, 7);
```
Null pointer (equal to 0) is a perfectly valid pointer value. Behaviour of dereferencing it is platform-specific (although I don't know the example where it can be usefull and it doesn't result in something unwanted).

### Arrays

Arrays index start at 0.
```
array = Array(Int, 10); # 10-element array of ints, array is a pointer to the first element
Ptr.array_elem(array, 0); # read the first element
```
Multidimensional array are nothing more than array of arrays.

## Types

Types are nothing more than information what is the minimum size of the variable in bits.
```
boolean: 1 = True;
pointer: 64 = Ptr.deref(boolean)
```

### Undefined

Every l-value variable has to be initialized. If we don't care for starting value we can use undefined value. Using undefined value in expression has undefined behaviour.
```
x: I32 = undefined;
```

## Match

Match is build with argument and list of possible cases. Values from the left side of an arrow from list of cases are compared with match argument from the top until two compared values are equal. Match arument and all of the values from the left side of an arrow has to have the same type (but only integers, chars and booleans are allowed).

Flow.default will always match the argument.
```
x = some_computations();
match x { # 'x' is an match argument
    0 => {} # first case
    1 => {} # second case
    2 => {}
    Flow.default => { Flow.nop; }
    3 => { Flow.unreachable; }
}
```
Match is an expression, it can return a value. Types of values returned from all of the cases has to be the same.
```
//C
int x = 5, y = 6;
int z = x > y ? x : y;

# LLL
x = Int(5);
y = Int(6);
z = match gr(x, y) {true => {x} false => {y}}
```

## Flow

### Jumps

Jumps works only within the same function.

### Return

Function return accepts only one argument which has to be the same type as the one declared 

### Drop

When function returns some value which we don't need we can drop it by assigning to a drop build-in value.

## Global variables

Every global variable needs to be initialized (setting to undefined is also ok). All global variables are availble all the time from any place in code.

## Functions

Everything in passed to function by value (copyied). If you want to move by reference you have to explicitly pass pointer. Functions can not be nested.

### Command line arguments

### Function pointers

Function pointers are treated like any other pointers.
```
fn add_5(a: Int)->Int{
    Flow.return(add(a, 5));
}
fn main()->Int{
    func = add_5;
    result: Int = func(3);
    Flow.return(0);
}
```

## Structs and unions

Structs are collection of variables.
```
Point = struct{x: Int, y: Int}
Circle = struct{}
```

Structs can contain arrays.
```
//C
typedef struct{
    int a;
    int b;
} Hehe;
int main(){
    Hehe arr[5];
    arr[3].b = 12;
}

# LLL
Hehe = struct{a : I32, b : I32};
fn main(){
	arr = Array(5, Hehe);
	Ptr.setElem(Ptr.getElem(arr, 3), Hehe.b, 12);
}
```
And arrays can contain structs.
```
//C
typedef struct{
    int arr[3];
} Hehe;
int main(){
    Hehe arr[5];
    arr[3].arr[2] = 12;
}

# LLL
Hehe = struct{arr : ArrayType(3, I32)}
int main(){
	arr = Array(5, Hehe);
	Ptr.setElem(Ptr.getElem(Ptr.getElem(arr, 3), Hehe.arr), 2, 12)
}
```

## STD

STD is divided into two main parts: build-ins and platform-dependent API. Build-in are available on all supported architectures (also for standalone programs).

### List of build-in functions

List of all build-in functions. In this section we're going to use following convenction:
```
function_name(arg1:ParametrizedType(Param1, Param2), arg2:Type) ReturnedType \ possible_side_effect
```

+ Array(Type, Size)
+ Flow.goto(Label) \ infinite_loop # if we jumping backwards
+ Flow.label() -> Label
+ Flow.return(Any) 
+ Ptr.addr(Any)
+ Ptr.array_elem(Ptr, Size) \ oob
+ Ptr.getElem(Ptr, Offset) Ptr \ oob, nullptr_deref
+ Ptr.set(a:Ptr(A), b:A) Unit \ nullptr_deref # if(eq(a, 0))
+ Ptr.setElem(Ptr(A), Offset, A) Unit \ oob, nullptr_deref

### C ABI

+ c.stdio.printf() \ io

### List of build-in types

+ I32 # 32-bit integer
+ Int # c-like int type
+ Offset
+ Ptr # pointer, integer with type it's pointing to
+ Size # unsigned integer
+ Unit # type with only one possible value. Used when function doesn't return any value. Similar to C's void

### List of build-in values

+ drop
+ Flow.default
+ Flow.nop
+ Flow.unreachable \ panic
+ Ptr.null
+ undefined

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
+ oob
+ panic
+ undefined_val

# Supported backends

LLL is designed as a backend language. When it's finished it will generate executables, but for now it will use other language as backend (LLVM IR).

# What and why this language doesn't support?

List of not supported features:
- casting
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
- preprocessor
- variadic functions
- std: io, files, threads, networking, containers, ...
- garbage collector
- classes and OOP stuff: inheritance, polymorphism, ...

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

# Syntax

```
Top = Global+
Global = Function | GlobalAssignment
GlobalAssignment = Name "=" RightGlobalAssignment ";"
RightGlobalAssignment = Literal | Struct | Union
Struct = "struct" "{" ArgumentsWithType? "}"
Union = "union" "{" ArgumentsWithType? "}" 
Function = "fn" Name "(" ArgumentsWithType? ")" "-" ">" Type "{" Statement+ "}"
Statement = Assignment ";" | FunctionCall ";" | Match
Match = "match" Arguments "{" Case+ "}"
Assignment = Name "=" Argument
Case = Argument "=" ">" "{" Statement+ "}"
Functioncall = Name "(" Arguments? ")"
ArgumentsWithType = (AgrumentWithType ",")* AgrumentWithType
AgrumentWithType = Name ":" Type
Arguments = (Argument ",")* Argument
Argument = FunctionCall | Match | Literal
Literal = "true" | "false" | Name | Int | Float | Char | Str
Type = Name
Int = "[0-9]+"
Name = "[_a-zA-Z][_a-zA-Z0-9'.']*"
Float = "[0-9]+.[0-9]+"
Char
Str
```
