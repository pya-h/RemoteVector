# Remote Vector Calculator
# Lang: Python
Combination of two old projects, resulting in a server-client app, that supports:
* Defining variables
* Defining vectors or matrices
* Performing Scalar or Vector math

Client app will send th expression and receives the result

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
* Print variabke/Vector Value:
    Just write the name and hit enter, like:
    v1
# Operators:
    Addition: '+'
    Substraction: '-'
    Multiplication: '*'
    Division: '@'
    Length or Abs: '|'
    Division: '/'

# Run:
Set your server configs, in config.py and then run it by: python3 server.py
Then open any number of clients you need with: python client.py