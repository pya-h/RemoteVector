# Remote Vector Calculator
# Lang: Both Rust & Python
Combination of two old projects, resulting in a server-client app, that supports:
* Defining variables
* Defining vectors or matrices
* Performing Scalar or Vector math

This project is first implemented in python, as a server-client app; And then after a year and some, implemented via Rust as a single interpretter for vector calculus. 
The rust section, is actually my rusty practice;

# Statements:
* Define variable:
    var_name = value
* Define Vector
    vector_name = v1 v2 v3 v4 ...
    result will be a dimension n vecor as (v1, v2, v3, v4)

    * vector components can also be a vector themselves (like matix), like:
        > x = 1 2

        > y = -1 -2
        
        > q = x y

* Using variables in vector definition:
    > x = 2

    > y = 1

    > q = x y

* Define variable resulted from an expression
    > x = v1 + v2 * v3 @ v4 ...

* Print variable/Vector Value:
    Just write the name and hit enter, like:
    v1

# Operators:
    Addition: '+'
    Subtraction: '-'
    Multiplication: '*'
    Inner Multiplication (.): '@'
    Length or Abs: '|'
    Division: '/'