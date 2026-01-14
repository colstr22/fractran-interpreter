An interpreter for the esoteric programming language Fractran.
The input is multiplied by the first item of the program, and if the result is a natural number, the value is updated, otherwise it remains the same.
Whenever a multiplication is 'successful' restart the process at the beginning of the program.
The program ends when a full pass without a 'successful' multiplication is made.
Register values are encoded in the powers of prime numbers.
Includes a very naive implementation of rational numbers as a struct using the Euclidean algorithm to simplify the fractions.
