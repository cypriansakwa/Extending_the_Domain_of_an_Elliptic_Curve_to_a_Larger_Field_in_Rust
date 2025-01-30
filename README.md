# Elliptic Curve Points over Finite Field Extensions

This project computes and lists the elements of the finite field extension $\mathbb{F}_{p^k}$ and finds valid points on an elliptic curve of the form:

$y^2 = x^3 + ax + b$

where $x, y \in \mathbb{F}_{p^k}$.

## Features
- **Finite Field Extension**: Implements arithmetic in $\mathbb{F}_{p^k}$ using polynomial representation.
- **Element Listing**: Generates and displays all elements of $\mathbb{F}_{p^k}$.
- **Elliptic Curve Point Computation**: Finds valid points $(x, y)$ that satisfy the elliptic curve equation.

## Example: Field Extension $\mathbb{F}_{5^2}$
This code specifically works with $\mathbb{F}_{5^2}$, where elements are written as $a + bt$ and satisfy:

$t^2 \equiv 3 \pmod{5}$

All 25 elements of $\mathbb{F}_{5^2}$ are generated and printed.

## Example: Elliptic Curve Over $\mathbb{F}_{5^2}$
The elliptic curve equation used in this example is:

$y^2 = x^3 + x + 1$

For each $x$ in $\mathbb{F}_{5^2}$, the corresponding $y$ values are computed and valid points $(x, y)$ are listed.

## Running the Code
To run the program, use:

