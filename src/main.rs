use std::fmt;

/// A struct to represent elements of the field \( \mathbb{F}_{5^2} \)
#[derive(Clone, Copy, Debug, PartialEq)]
struct F5x2 {
    a: u8, // Coefficient for 1
    b: u8, // Coefficient for t
}

impl fmt::Display for F5x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),
            (a, 0) => write!(f, "{}", a),
            (0, b) => write!(f, "{}t", b),      
            (a, b) => write!(f, "{} + {}t", a, b), 
        }
    }
}

impl F5x2 {
    /// Create a new field element
    fn new(a: u8, b: u8) -> Self {
        F5x2 { a: a % 5, b: b % 5 }
    }

    /// Add two elements of \( \mathbb{F}_{5^2} \)
    fn add(self, other: F5x2) -> F5x2 {
        F5x2 {
            a: (self.a + other.a) % 5,
            b: (self.b + other.b) % 5,
        }
    }

    /// Multiply two elements of \( \mathbb{F}_{5^2} \)
    fn mul(self, other: F5x2) -> F5x2 {
        let a = self.a as i16;
        let b = self.b as i16;
        let c = other.a as i16;
        let d = other.b as i16;

        // Polynomial multiplication: (a + bt) * (c + dt)
        let ac = (a * c) % 5;
        let bd = (b * d) % 5;
        let ad_bc = (a * d + b * c) % 5;

        // Reduction modulo t^2 + 2, where t^2 ≡ 3 (mod 5)
        let new_a = (ac + 3 * bd) % 5;
        let new_b = ad_bc % 5;

        F5x2::new(new_a as u8, new_b as u8)
    }
}

fn main() {
    // Define all elements of \( \mathbb{F}_{5^2} \)
    let mut field_elements = Vec::new();
    for a in 0..5 {
        for b in 0..5 {
            field_elements.push(F5x2::new(a, b));
        }
    }

    // Print all elements of \( \mathbb{F}_{5^2} \)
    println!("Elements of \\mathbb{{F}}_{{5^2}}:");
    for elem in field_elements.iter() {
        println!("{}", elem);
    }

    // Example: Compute points on the elliptic curve y^2 = x^3 + x + 1 over \( \mathbb{F}_{5^2} \)
    let a = F5x2::new(1, 0); // Coefficient a = 1
    let b = F5x2::new(1, 0); // Coefficient b = 1

    println!("\nPoints on the elliptic curve y^2 = x^3 + x + 1 over \\mathbb{{F}}_{{5^2}}:");
    for x in field_elements.iter() {
        let x_cubed = x.mul(*x).mul(*x); // x^3
        let rhs = x_cubed.add(a.mul(*x)).add(b); // x^3 + ax + b

        for y in field_elements.iter() {
            if y.mul(*y) == rhs {
                println!("Point ({}, {})", x, y);
            }
        }
    }
}
