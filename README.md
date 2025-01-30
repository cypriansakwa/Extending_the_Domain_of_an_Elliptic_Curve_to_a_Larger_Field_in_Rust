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

## Prerequisites

- Rust installed. If not, install it using [rustup](https://rustup.rs/).
- Cargo package manager (comes with Rust).

## Installation

Clone this repository:

```sh
git clone https://github.com/cypriansakwa/Extending_the_Domain_of_an_Elliptic_Curve_to_a_Larger_Field_in_Rust.git
cd Extending_the_Domain_of_an_Elliptic_Curve_to_a_Larger_Field_in_Rust
```

## Running the Program

Use the following command to compile and run the program:

```sh
cargo run
```

Example output (for **𝔽₅²**):

```sh
Elements of \mathbb{F}_{5^2}:
0 
1t
2t
3t
4t
1
1 + 1t
1 + 2t
1 + 3t
1 + 4t
2
2 + 1t
2 + 2t
2 + 3t
2 + 4t
3
3 + 1t
3 + 2t
3 + 3t
3 + 4t
4
4 + 1t
4 + 2t
4 + 3t
4 + 4t

Points on the elliptic curve y^2 = x^3 + x + 1 over \mathbb{F}_{5^2}:
Point (0, 1)
Point (0, 4)
Point (1, 1t)
Point (1, 4t)
Point (1 + 2t, 1 + 1t)
Point (1 + 2t, 4 + 4t)
Point (1 + 3t, 1 + 4t)
Point (1 + 3t, 4 + 1t)
Point (2, 1)
Point (2, 4)
Point (2 + 2t, 1t)
Point (2 + 2t, 4t)
Point (2 + 3t, 1t)
Point (2 + 3t, 4t)
Point (3, 1)
Point (3, 4)
Point (3 + 1t, 1 + 3t)
Point (3 + 1t, 4 + 2t)
Point (3 + 2t, 2)
Point (3 + 2t, 3)
Point (3 + 3t, 2)
Point (3 + 3t, 3)
Point (3 + 4t, 1 + 2t)
Point (3 + 4t, 4 + 3t)
Point (4, 2)
Point (4, 3)
```

