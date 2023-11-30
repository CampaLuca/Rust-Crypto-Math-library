use crate::numbers::numbers::Instance;
use crate::numbers::numbers::Operand;
use crate::poly::monomial::Monomial;




// POLYNOMIAL
#[derive(Clone)]
pub struct Polynomial<T> {
    monomials: Vec<Monomial<T>>
}

impl<T> Polynomial<T> where T: Instance + Operand + Clone {
    pub fn new(monomials: Vec<Monomial<T>>) -> Polynomial<T> {
        Polynomial { monomials: monomials }
    }

    // pub fn ToUnivariatePolynomial(self) -> UnivariatePolynomial<T> {
    //     // write the code to convert a polynomial to univariate polynomial
    // }
}

// add function

// mul function

// div function

// sub function

// pow to BigInt




impl<T> std::fmt::Display for Polynomial<T> where T: Instance + Operand + Clone +  std::fmt::Display {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut printable = format!("");

        for i in &self.monomials {
            let monomial = format!("{i}");
            printable = format!("{printable}{monomial}")
        }

        write!(f, "{0}", printable )
    }
}