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

//! Polynomial rings.
//!
//! [PolyRing] is a convenient wrapper for working with arbitrary (potentially 
//! recursively defined) polynomial rings.
//!
//! The default behavior is to use a [GenericPolyRing] which can be specialized for
//! any ring where optimized implementations are available. If the polynomial ring 
//! needs to use some underlying storage that differs from that of
//! [GenericPolyRing] (for example, polynomial rings coming from FFI such as 
//! [IntPolyRing]) then this can be specified with the `PolyRing` associated type 
//! in the [Ring] trait implementation for the base ring.

use crate::*;
use std::fmt;
use std::rc::Rc;

mod arith;
mod generic;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PolyRing<T: Ring> {
    inner: Rc<T::PolynomialRing>,
}

impl<T: Ring> fmt::Display for PolyRing<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner.to_string())
    }
}

impl<T: Ring> Parent for PolyRing<T> {
    type Element = Poly<T>;

    #[inline]
    fn default(&self) -> Self::Element {
        PolynomialRing::default(self)
    }
}

impl<T: Ring> Ring for PolyRing<T> {
    type Element = Poly<T>;
    type PolynomialRing = GenericPolyRing<Self>;
    type MatrixSpace = GenericMatSpace<Self>;

    #[inline]
    fn default(&self) -> <Self as Ring>::Element {
        PolynomialRing::default(self)
    }
}

impl<T: Ring> PolynomialRing<T> for PolyRing<T> {
    type Element = Poly<T>;

    #[inline]
    fn default(&self) -> <Self as PolynomialRing<T>>::Element {
        Poly {
            ctx: Rc::clone(&self.inner),
            inner: PolynomialRing::default(&*self.inner),
        }
    }

    #[inline]
    fn init(ring: &T, var: &str) -> Self {
        PolyRing {
            inner: Rc::new(T::PolynomialRing::init(ring, var)),
        }
    }

    #[inline]
    fn base_ring(&self) -> T {
        self.inner.base_ring()
    }

    #[inline]
    fn var(&self) -> String {
        self.inner.var()
    }

    #[inline]
    fn set_var<S: AsRef<str>>(&self, var: S) {
        self.inner.set_var(var);
    }
}

#[derive(Clone, Debug, Eq, Hash)]
pub struct Poly<T: Ring> {
    ctx: Rc<T::PolynomialRing>,
    inner: <T::PolynomialRing as PolynomialRing<T>>::Element,
}

impl<S, T> PartialEq<Poly<S>> for Poly<T> where
    S: Ring,
    T: Ring,
    <T::PolynomialRing as PolynomialRing<T>>::Element: 
        PartialEq<<S::PolynomialRing as PolynomialRing<S>>::Element>
{
    #[inline]
    fn eq(&self, rhs: &Poly<S>) -> bool {
        self.inner == rhs.inner
    }
}

impl<S, T> PartialEq<&Poly<S>> for Poly<T> where
    S: Ring,
    T: Ring,
    <T::PolynomialRing as PolynomialRing<T>>::Element: 
        PartialEq<<S::PolynomialRing as PolynomialRing<S>>::Element>
{
    #[inline]
    fn eq(&self, rhs: &&Poly<S>) -> bool {
        self.inner == rhs.inner
    }
}

impl<S, T> PartialEq<Poly<S>> for &Poly<T> where
    S: Ring,
    T: Ring,
    <T::PolynomialRing as PolynomialRing<T>>::Element: 
        PartialEq<<S::PolynomialRing as PolynomialRing<S>>::Element>
{
    #[inline]
    fn eq(&self, rhs: &Poly<S>) -> bool {
        self.inner == rhs.inner
    }
}

impl<T: Ring> fmt::Display for Poly<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T: Ring> Element for Poly<T> {
    type Parent = PolyRing<T>;

    #[inline]
    fn parent(&self) -> Self::Parent {
        PolynomialRingElement::parent(self)
    }
}

impl<T: Ring> RingElement for Poly<T> {
    type Parent = PolyRing<T>;

    #[inline]
    fn parent(&self) -> <Self as RingElement>::Parent {
        PolynomialRingElement::parent(self)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }
}

impl<T: Ring> PolynomialRingElement<T> for Poly<T> {
    type Parent = PolyRing<T>;

    #[inline]
    fn parent(&self) -> <Self as PolynomialRingElement<T>>::Parent {
        PolyRing {
            inner: Rc::clone(&self.ctx),
        }
    }

    #[inline]
    fn base_ring(&self) -> T {
        self.inner.base_ring()
    }

    #[inline]
    fn var(&self) -> String {
        self.inner.var()
    }

    #[inline]
    fn set_var<S: AsRef<str>>(&self, var: S) {
        self.inner.set_var(var);
    }

    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn degree(&self) -> i64 {
        self.inner.degree()
    }

    #[inline]
    fn get_coeff(&self, i: usize) -> <T as Ring>::Element {
        self.inner.get_coeff(i)
    }

    #[inline]
    fn set_coeff<'a, S>(&mut self, i: usize, coeff: S)
    where
        <T as Ring>::Element: 'a,
        S: Into<ValOrRef<'a, <T as Ring>::Element>>,
    {
        self.inner.set_coeff(i, coeff);
    }

    #[inline]
    fn coefficients(&self) -> Vec<<T as Ring>::Element> {
        self.inner.coefficients()
    }
}

impl<T, A> New<A> for PolyRing<T>
where
    T: Ring,
    T::PolynomialRing: 
        New<A, Output = <T::PolynomialRing as PolynomialRing<T>>::Element>,
{
    type Output = Poly<T>;
    fn new(&self, a: A) -> Poly<T> {
        Poly {
            ctx: Rc::clone(&self.inner),
            inner: self.inner.new(a),
        }
    }
}

pub use generic::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poly() {
        let zz = IntegerRing {};
        let qq = RationalField {};

        let p1 = PolyRing::init(&zz, "x");
        let p2 = PolyRing::init(&qq, "y");
        let p3 = PolyRing::init(&p2, "z");

        println!("{}", p1);
        println!("{}", p2);
        println!("{}", p3);
    }
}
