/*
 *  Copyright (C) 2021 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(non_snake_case)]

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Rem;
use std::sync::Arc;

use flint_sys::fmpz_poly::fmpz_poly_struct;
use libc::{c_int, c_long, c_ulong};

use crate::*;

// IntPoly //

/// The ring of polynomials with [Integer] coefficients that can be used as an integer polynomial
/// "actory".
pub type IntPolyRing = PolyRing<IntegerRing>;

impl Parent for IntPolyRing {
    type Element = IntPoly;
    type Context = ();

    #[inline]
    fn default(&self) -> IntPoly {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_init(z.as_mut_ptr());
            IntPoly { data: IntPolyData { x: Arc::clone(&self.var), elem: z.assume_init() } }
        }
    }
}

impl Additive for IntPolyRing {
    #[inline]
    fn zero(&self) -> IntPoly {
        self.default()
    }
}

impl Multiplicative for IntPolyRing {
    #[inline]
    fn one(&self) -> IntPoly {
        let mut res = self.default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for IntPolyRing {}

impl Ring for IntPolyRing {}

impl PolynomialRing for IntPolyRing {
    type BaseRing = IntegerRing;

    #[inline]
    fn base_ring(&self) -> IntegerRing {
        IntegerRing {}
    }

    #[inline]
    fn gens(&self) -> Vec<IntPoly> {
        vec![IntPoly::from(vec![0,1].as_slice())]
    }

}

impl InitParent1<&str> for IntPolyRing {
    #[inline]
    fn init(x: &str) -> Self {
        IntPolyRing { phantom: PhantomData::<IntegerRing>, ctx: (), var: Arc::new(x.to_owned()) }
    }
}

impl NewElement<&IntPoly> for IntPolyRing {
    #[inline]
    fn new(&self, x: &IntPoly) -> IntPoly {
        x.clone()
    }
}

impl<T> NewElement<T> for IntPolyRing where 
    T: Into<IntPoly>
{
    #[inline]
    fn new(&self, x: T) -> IntPoly {
        x.into()
    }
}

// IntPoly //

/// A polynomial with [Integer] coefficients. The field `data` is a FLINT
/// [fmpz_poly][flint_sys::fmpz_poly::fmpz_poly_struct].
pub type IntPoly = Elem<IntPolyRing>;

#[derive(Debug)]
pub struct IntPolyData {
    pub elem: fmpz_poly_struct,
    pub x: Arc<String>,
}

impl Drop for IntPolyData {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_clear(&mut self.elem);}
    }
}

impl Element for IntPoly {
    type Data = IntPolyData;
    type Parent = IntPolyRing;

    #[inline]
    fn parent(&self) -> IntPolyRing {
        IntPolyRing { phantom: PhantomData::<IntegerRing>, ctx: (), var: Arc::clone(&self.data.x) }
    }
}

impl AdditiveElement for IntPoly {
    #[inline]
    fn is_zero(&self) -> bool {
        self == &0
    }
}

impl MultiplicativeElement for IntPoly {
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for IntPoly {}

impl RingElement for IntPoly {}

impl PolynomialRingElement for IntPoly {
    type BaseRingElement = Integer;

    /// Return the length of the polynomial, equivalently, the degree plus one.
    #[inline]
    fn len(&self) -> c_long {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_length(self.as_ptr())}
    }
    
    /// Return the degree of the polynomial.
    #[inline]
    fn degree(&self) -> c_long {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_degree(self.as_ptr())}
    }
   
    fn var(&self) -> String {
        (*self.data.x).clone()
    }

    /// Get the i-th coefficient of an integer polynomial.
    #[inline]
    fn get_coeff(&self, i: usize) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_get_coeff_fmpz(res.as_mut_ptr(), self.as_ptr(), i as i64);
            res
        }
    }
    
    /// Set the i-th coefficient of an integer polynomial to an [Integer].
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: &Integer) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set_coeff_fmpz(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.as_ptr()
            );
        }
    }
    
    /// Return a pretty-printed [String] representation of an integer polynomial.
    #[inline]
    fn get_str_pretty(&self) -> String {
        let v = CString::new((*self.data.x).clone()).unwrap();
        unsafe {
            let s = flint_sys::fmpz_poly::fmpz_poly_get_str_pretty(self.as_ptr(), v.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
}

impl IntPoly {

    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz_poly_struct {
        &self.data.elem
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz_poly_struct {
        &mut self.data.elem
    }

    /// Return a [String] representation of an integer polynomial.
    #[inline]
    pub fn get_str(&self) -> String {
        unsafe {
            let s = flint_sys::fmpz_poly::fmpz_poly_get_str(self.as_ptr());
            match CStr::from_ptr(s).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }
    
    /// Return true if the polynomial is invertible as a rational function. False is returned only
    /// if the polynomial is zero.
    #[inline]
    pub fn is_invertible(&self) -> bool {
        !self.is_zero()
    }
 
    /// Set the i-th coefficient of an integer polynomial to an unsigned integer.
    #[inline]
    pub fn set_coeff_ui<T>(&mut self, i: usize, coeff: T) where 
        T: Into<c_ulong> 
    {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set_coeff_ui(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.into()
            );
        }
    }
    
    /// Set the i-th coefficient of an integer polynomial to a signed integer.
    #[inline]
    pub fn set_coeff_si<T>(&mut self, i: usize, coeff: T) where 
        T: Into<c_long> 
    {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_set_coeff_si(
                self.as_mut_ptr(), 
                i as c_long, 
                coeff.into()
            );
        }
    }
    
    /// Return true if the polynomial is the unit +/-1.
    #[inline]
    pub fn is_unit(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_unit(self.as_ptr()) == 1}
    }
    
    /// Return true if the polynomial if the generator `x`, a degree one polynomial with
    /// coefficient one and no constant term.
    #[inline]
    pub fn is_gen(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_gen(self.as_ptr()) == 1}
    }
    
    /// Return true if the polynomial has no factors with multiplicity greater than one.
    #[inline]
    pub fn is_squarefree(&self) -> bool {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_is_squarefree(self.as_ptr()) == 1}
    }

    /// Return true if the polynomial is monic.
    #[inline]
    pub fn is_monic(&self) -> bool {
        self.get_coeff(self.degree() as usize).is_one()
    }

    /// Return true if the polynomial is constant.
    pub fn is_constant(&self) -> bool {
        self.len() <= 1
    }

    /// Returns the maximum number of limbs required to store the absolute value of the
    /// coefficients of an integer polynomial.
    #[inline]
    pub fn max_limbs(&self) -> c_ulong {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_max_limbs(self.as_ptr())}
    }
    
    /// Computes the maximum number of bits required to store the absolute value of the
    /// coefficients of an integer polynomial.
    #[inline]
    pub fn max_bits(&self) -> c_long {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_max_bits(self.as_ptr())}
    }

    /// Return the polynomial whose coefficients are the absolute value of the coefficients of the
    /// input polynomial.
    #[inline]
    pub fn abs(&self) -> IntPoly {
        let mut res = self.parent().default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_scalar_abs(res.as_mut_ptr(), self.as_ptr()); }
        res
    }

    /// Computes the height of an integer polynomial, defined as the largest of the absolute value
    /// of its coefficients. Equivalently, this gives the infinity norm of the coefficients.
    #[inline]
    pub fn height(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_height(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }

    /// Return the l2-norm of the coefficients of an integer polynomial.
    #[inline]
    pub fn l2_norm(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_2norm(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    /// Returns the discriminant of the polynomial.
    #[inline]
    pub fn discriminant(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_discriminant(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    /// Returns the content of the polynomial.
    #[inline]
    pub fn content(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_content(res.as_mut_ptr(), self.as_ptr());
        }
        res
    }
    
    /// Returns the primitive part of the polynomial, equivalent to dividing the polynomial by its
    /// content, normalized to have non-negative leading coefficient.
    #[inline]
    pub fn primitive_part(&self) -> IntPoly {
        let mut res = self.parent().default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_primitive_part(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    /// Reverse the polynomial, so the coefficients are in reverse order.
    #[inline]
    pub fn reverse(&mut self) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_reverse(self.as_mut_ptr(), self.as_ptr(), self.len());
        }
    }
    
    /// Truncate the polynomial to have length `n`.
    #[inline]
    pub fn truncate(&mut self, n: c_long) {
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_truncate(self.as_mut_ptr(), n);
        }
    }
    
    // no cdiv in flint

    /// Return the polynomial whose coefficients are the coefficients of the input polynomial
    /// divided by `other` rounded down.
    #[inline]
    pub fn fdiv(&self, other: &Integer) -> IntPoly {
        assert!(!other.is_zero());
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_fdiv_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            res
        }
    }
    
    /// Return the polynomial whose coefficients are the coefficients of the input polynomial
    /// divided by `other` and truncated.
    #[inline]
    pub fn tdiv(&self, other: &Integer) -> IntPoly {
        assert!(!other.is_zero());
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_tdiv_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            res
        }
    }
 
    /// Return the polynomial whose coefficients are the coefficients of the input polynomial
    /// divided by `other` exactly. If the division isn't exact an [Err] is returned.
    #[inline]
    pub fn divexact(&self, other: &Integer) -> Result<IntPoly, ()> {
        assert!(!other.is_zero());
        
        let coeffs = self.coefficients();
        for coeff in coeffs {
            if coeff.rem(other) != 0 {
                return Err(());
            }
        }

        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_divexact_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            Ok(res)
        }
    }
   
    /// Compute the polynomial whose coefficients are the symmetric remainder of the input
    /// polynomial coefficients modulo `other`.
    #[inline]
    pub fn srem(&self, other: &Integer) -> IntPoly {
        assert!(!other.is_zero());
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_scalar_smod_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            res
        }
    }
    
    /// Return the quotient and remainder of division of an integer polynomial by `other`.
    #[inline]
    pub fn divrem(&self, other: &IntPoly) -> (IntPoly, IntPoly) {
        assert!(!other.is_zero());
        let mut q = IntPoly::default();
        let mut r = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_divrem(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            (q, r)
        }
    }
  
    /// Square an integer polynomial.
    #[inline]
    pub fn square(&self) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_sqr(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    
    /// Return the polynomial with coefficients of the input polynomial shifted left by `n` terms.
    #[inline]
    pub fn shift_left(&mut self, n: c_long) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_shift_left(self.as_mut_ptr(), self.as_ptr(), n);}
    }
    
    /// Return the polynomial with coefficients of the input polynomial shifted right by `n` terms.
    #[inline]
    pub fn shift_right(&mut self, n: c_long) {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_shift_right(self.as_mut_ptr(), self.as_ptr(), n);}
    }

    /// Return the greatest common divisor of two integer polynomials.
    #[inline]
    pub fn gcd(&self, other: &IntPoly) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }

    /// Returns the least common multiple of two integer polynomials.
    #[inline]
    pub fn lcm(&self, other: &IntPoly) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_lcm(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            res
        }
    }

    /// Computes the extended gcd of two integer polynomials `f` and `g`. We return `(d, a, b)` where 
    /// `gcd(f, g) = d` and `d = a*f + b*g`.
    #[inline]
    pub fn xgcd(&self, other: &IntPoly) -> (Integer, IntPoly, IntPoly) {
        unsafe {
            let mut d = Integer::default();
            let mut a = IntPoly::default();
            let mut b = IntPoly::default();
            flint_sys::fmpz_poly::fmpz_poly_xgcd(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ptr()
            );
            (d, a, b)
        }
    }
   
    /// Return the resultant of two integer polynomials.
    #[inline]
    pub fn resultant(&self, other: &IntPoly) -> Integer {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_resultant(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
            res
        }
    }
    
    #[inline]
    pub fn divides(&self, other: &IntPoly) -> bool {
        let mut res = IntPoly::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_divides(
            res.as_mut_ptr(), 
            other.as_ptr(), 
            self.as_ptr()) == 1 
        }
    }
    
    #[inline]
    pub fn remove(&mut self, other: &IntPoly) -> c_int {
        assert!(!other.is_zero());
        assert!(other.abs() != 1);
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_divides(
                self.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr())
        }
    }
    
    #[inline]
    pub fn inv_series(&self, n: c_long) -> IntPoly {
        assert!(self.get_coeff(0).abs() == 1);
        assert!(n >= 1);

        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_inv_series(res.as_mut_ptr(), self.as_ptr(), n);
            res
        }
    }
    
    #[inline]
    pub fn div_series(&self, other: &IntPoly, n: c_long) -> IntPoly {
        assert!(other.get_coeff(0).abs() == 1);
        assert!(n >= 1);

        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_div_series(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr(), 
                n);
            res
        }
    }
    
    #[inline]
    pub fn derivative(&self) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_derivative(res.as_mut_ptr(), self.as_ptr());}
        res
    }

    // TODO: Flint inexact error thrown if output is rational polynomial (use RatPoly::interpolate).
    #[inline]
    pub fn interpolate(x: &[Integer], y: &[Integer]) -> IntPoly {
        assert_eq!(x.len(), y.len());
        let n = x.len();

        let vx = Vec::from_iter(x.iter().map(|x| x.as_ptr().clone()));
        let vy = Vec::from_iter(y.iter().map(|y| y.as_ptr().clone()));

        let mut res = IntPoly::default();
        unsafe { 
            flint_sys::fmpz_poly::fmpz_poly_interpolate_fmpz_vec(
                res.as_mut_ptr(),
                vx.as_ptr(),
                vy.as_ptr(),
                n as c_long
            );
        }
        res
    }

    #[inline]
    pub fn compose(&self, other: &IntPoly) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_compose(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
        res
    }
    
    #[inline]
    pub fn inflate(&self, n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_inflate(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn deflate(&self, n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe { flint_sys::fmpz_poly::fmpz_poly_deflate(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn deflation(&self) -> c_ulong {
        unsafe { flint_sys::fmpz_poly::fmpz_poly_deflation(self.as_ptr())}
    }

    #[inline]
    pub fn taylor_shift(&self, c: &Integer) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_taylor_shift(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                c.as_ptr()
            );
        }
        res
    }
    
    #[inline]
    pub fn compose_series(&self, other: &IntPoly, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_compose_series(
                res.as_mut_ptr(), 
                self.as_ptr(),
                other.as_ptr(),
                n
            );
        }
        res
    }
    
    #[inline]
    pub fn revert_series(&self, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_revert_series(
                res.as_mut_ptr(), 
                self.as_ptr(),
                n);
        }
        res
    }

    #[inline]
    pub fn sqrt(&self) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            let n = flint_sys::fmpz_poly::fmpz_poly_sqrt(res.as_mut_ptr(), self.as_ptr());
            assert_eq!(n, 1);
        }
        res
    }
    
    #[inline]
    pub fn sqrt_series(&self, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        assert!(n > 0);
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_sqrt_series(res.as_mut_ptr(), self.as_ptr(), n);
        }
        res
    }
    
    #[inline]
    pub fn power_sums_naive(&self, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_power_sums_naive(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                n);
        }
        res
    }
    
    #[inline]
    pub fn power_sums(&self, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_power_sums(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                n);
        }
        res
    }
    
    #[inline]
    pub fn power_sums_to_poly(&self) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_power_sums_to_poly(
                res.as_mut_ptr(), 
                self.as_ptr()); 
        }
        res
    }
   
    #[inline]
    pub fn signature(&self) -> (c_long, c_long) {
        assert!(self.is_squarefree());

        let mut r1 = 0 as c_long;
        let mut r2 = 0 as c_long;
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_signature(
                &mut r1,
                &mut r2,
                self.as_ptr()); 
        }
        (r1, r2)
    }

    #[inline]
    pub fn hensel_lift(
        &self, 
        g: &IntPoly, 
        h: &IntPoly, 
        a: &IntPoly, 
        b: &IntPoly, 
        p: &Integer, 
        p1: &Integer
    ) -> (IntPoly, IntPoly, IntPoly, IntPoly) 
    {
        let mut G = IntPoly::default();
        let mut H = IntPoly::default();
        let mut A = IntPoly::default();
        let mut B = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_hensel_lift(
                G.as_mut_ptr(),
                H.as_mut_ptr(),
                A.as_mut_ptr(),
                B.as_mut_ptr(),
                self.as_ptr(),
                g.as_ptr(),
                h.as_ptr(),
                a.as_ptr(),
                b.as_ptr(),
                p.as_ptr(),
                p1.as_ptr());
        }
        (G, H, A, B)
    }
    
    #[inline]
    pub fn hensel_lift_only_inv(
        G: &IntPoly,
        H: &IntPoly,
        a: &IntPoly, 
        b: &IntPoly, 
        p: &Integer, 
        p1: &Integer
    ) -> (IntPoly, IntPoly)
{
        let mut A = IntPoly::default();
        let mut B = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_hensel_lift_only_inverse(
                A.as_mut_ptr(),
                B.as_mut_ptr(),
                G.as_ptr(),
                H.as_ptr(),
                a.as_ptr(),
                b.as_ptr(),
                p.as_ptr(),
                p1.as_ptr());
        }
        (A, B)
    }
    
    #[inline]
    pub fn hensel_lift_no_inv(
        &self,
        g: &IntPoly,
        h: &IntPoly, 
        a: &IntPoly, 
        b: &IntPoly, 
        p: &Integer, 
        p1: &Integer
    ) -> (IntPoly, IntPoly) where
{
        let mut G = IntPoly::default();
        let mut H = IntPoly::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_hensel_lift_without_inverse(
                G.as_mut_ptr(),
                H.as_mut_ptr(),
                self.as_ptr(),
                g.as_ptr(),
                h.as_ptr(),
                a.as_ptr(),
                b.as_ptr(),
                p.as_ptr(),
                p1.as_ptr());
        }
        (G, H)
    }

    // CRT once nmod poly implemented

    #[inline]
    pub fn bound_roots(&self) -> Integer {
        let mut res = Integer::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_bound_roots(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    #[inline]
    pub fn num_real_roots(&self) -> c_long {
        unsafe {flint_sys::fmpz_poly::fmpz_poly_num_real_roots(self.as_ptr())}
    }
    
    #[inline]
    pub fn cyclotomic(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_cyclotomic(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn cos_minpoly(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_cos_minpoly(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn swinnerton_dyer(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_swinnerton_dyer(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn chebyshev_t(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_chebyshev_t(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn chebyshev_u(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_chebyshev_u(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn legendre_pt(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_legendre_pt(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn hermite_h(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_hermite_h(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn hermite_he(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_hermite_he(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn fibonacci(n: c_ulong) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_fibonacci(res.as_mut_ptr(), n);}
        res
    }
    
    #[inline]
    pub fn eta_qexp(r: c_long, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_eta_qexp(res.as_mut_ptr(), r, n);}
        res
    }
    
    #[inline]
    pub fn theta_qexp(r: c_long, n: c_long) -> IntPoly {
        let mut res = IntPoly::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_theta_qexp(res.as_mut_ptr(), r, n);}
        res
    }
    
    #[inline]
    pub fn CLD_bound(&self, n: c_long) -> Integer {
        let mut res = Integer::default();
        unsafe {flint_sys::fmpz_poly::fmpz_poly_CLD_bound(res.as_mut_ptr(), self.as_ptr(), n);}
        res
    }
}

impl<T> Evaluate<T> for IntPoly where
    T: Into<Integer>
{
    type Output = Integer;
    #[inline]
    fn evaluate(&self, x: T) -> Self::Output {
        self.evaluate(&x.into())
    }
}

impl Evaluate<&Integer> for IntPoly {
    type Output = Integer;
    #[inline]
    fn evaluate(&self, x: &Integer) -> Self::Output {
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz_poly::fmpz_poly_evaluate_fmpz(
                res.as_mut_ptr(),
                self.as_ptr(),
                x.as_ptr()
            );
        }
        res
    }
}

impl Evaluate<Rational> for IntPoly {
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: Rational) -> Self::Output {
        RatPoly::from(self).evaluate(x)
    }
}

impl Evaluate<&Rational> for IntPoly {
    type Output = Rational;
    #[inline]
    fn evaluate(&self, x: &Rational) -> Self::Output {
        RatPoly::from(self).evaluate(x)
    }
}