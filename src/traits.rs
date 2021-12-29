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

use std::fmt::{self, Debug};
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

use libc::c_long;

/// Traits for operations and algebraic structures.


/// Inverse as a unary operation.
pub trait Inv {
    type Output;
    fn inv(self) -> Self::Output;
}

/// Inverse with assignment.
pub trait InvAssign {
    fn inv_assign(&mut self);
}

/// Bitwise `and` of two items with assignment into a third.
pub trait AssignBitAnd<T, U> {
    fn assign_bitand(&mut self, lhs: T, rhs: U);
}

/// Bitwise `or` of two items with assignment into a third.
pub trait AssignBitOr<T, U> {
    fn assign_bitor(&mut self, lhs: T, rhs: U);
}

/// Bitwise `xor` of two items with assignment into a third.
pub trait AssignBitXor<T, U> {
    fn assign_bitxor(&mut self, lhs: T, rhs: U);
}

/// Addition of two items with assignment into a third.
pub trait AssignAdd<T, U> {
    fn assign_add(&mut self, lhs: T, rhs: U);
}

/// Subtraction of two items with assignment into a third.
pub trait AssignSub<T, U> {
    fn assign_sub(&mut self, lhs: T, rhs: U);
}

/// Multiplication of two items with assignment into a third.
pub trait AssignMul<T, U> {
    fn assign_mul(&mut self, lhs: T, rhs: U);
}

/// Division of two items with assignment into a third.
pub trait AssignDiv<T, U> {
    fn assign_div(&mut self, lhs: T, rhs: U);
}

/// Exponentiation of two items with assignment into a third.
pub trait AssignPow<T, U> {
    fn assign_pow(&mut self, lhs: T, rhs: U);
}

/// Remainder of two items with assignment into a third.
pub trait AssignRem<T, U> {
    fn assign_rem(&mut self, lhs: T, rhs: U);
}

/// Evaluation of an expression at `x`.
pub trait Evaluate<T> {
    type Output;
    fn evaluate(&self, x: T) -> Self::Output;
}

/// Modular evaluation of an expression at `x`.
pub trait EvaluateMod<T, U> {
    type Output;
    fn evaluate_mod(&self, x: T, modulus: U) -> Self::Output;
}

/// Evaluation of a `Product`.
pub trait EvaluateProduct {
    type Output;
    fn evaluate(&self) -> Self::Output;
}

/// Modular evaluation of a `Product`.
pub trait EvaluateProductMod<T> {
    type Output;
    fn evaluate_mod(&self, modulus: T) -> Self::Output;
}

/// Factorization.
pub trait Factorizable {
    type Output;
    fn factor(&self) -> Self::Output;
}

/// A generic parent, for example an algebraic structure like a ring.
pub trait Parent {
    type Element: Element;
    type Context: Clone + Debug + Hash;

    fn default(&self) -> Self::Element;
}

pub trait InitParent: Parent {
    fn init() -> Self;
}

pub trait InitParent1<A>: Parent {
    fn init(a: A) -> Self;
}

pub trait InitParent2<A, B>: Parent {
    fn init(a: A, b: B) -> Self;
}

pub trait InitParent3<A, B, C>: Parent {
    fn init(a: A, b: B, c: C) -> Self;
}

pub trait InitParent4<A, B, C, D>: Parent {
    fn init(a: A, b: B, c: C, d: D) -> Self;
}

pub trait InitParent5<A, B, C, D, E>: Parent {
    fn init(a: A, b: B, c: C, d: D, e: E) -> Self;
}

pub trait NewElement<T>: Parent {
    fn new(&self, x: T) -> <Self as Parent>::Element;
}

/// An generic element of a `Parent`.
pub trait Element {
    type Data: fmt::Debug;
    type Parent: Parent;

    fn parent(&self) -> Self::Parent;
}

pub trait Additive: Parent {
    fn zero(&self) -> <Self as Parent>::Element;
}
pub trait AdditiveElement: Element {
    fn is_zero(&self) -> bool;
}

pub trait Multiplicative: Parent {
    fn one(&self) -> <Self as Parent>::Element;
}
pub trait MultiplicativeElement: Element {
    fn is_one(&self) -> bool;
}

pub trait AdditiveGroup: Additive {
    #[inline]
    fn identity(&self) -> <Self as Parent>::Element {
        self.zero()
    }
}
pub trait AdditiveGroupElement: AdditiveElement {
    #[inline]
    fn is_identity(&self) -> bool {
        self.is_zero()
    }
}

pub trait MultiplicativeGroup: Multiplicative {
    #[inline]
    fn identity(&self) -> <Self as Parent>::Element {
        self.one()
    }
}
pub trait MultiplicativeGroupElement: MultiplicativeElement {
    #[inline]
    fn is_identity(&self) -> bool {
        self.is_one()
    }
}

pub trait Module: AdditiveGroup {}
pub trait ModuleElement: AdditiveGroupElement {}

pub trait VectorSpace: Module {
    type BaseRing: Ring;
    fn base_ring(&self) -> Self::BaseRing;
}
pub trait VectorSpaceElement: ModuleElement {
    type BaseRingElement: RingElement;
}

pub trait MatrixSpace: VectorSpace {

    fn nrows(&self) -> c_long;
    
    fn ncols(&self) -> c_long;
}
pub trait MatrixSpaceElement: VectorSpaceElement {

    fn nrows(&self) -> c_long;
    
    fn ncols(&self) -> c_long;
    
    fn get_entry(&self, i: usize, j: usize) -> <Self as VectorSpaceElement>::BaseRingElement;
  
    fn set_entry(&mut self, i: usize, j: usize, e: &<Self as VectorSpaceElement>::BaseRingElement);

    #[inline]
    fn is_empty(&self) -> bool {
        self.nrows() == 0 || self.ncols() == 0
    }

    #[inline]
    fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }

    fn get_str(&self) -> String {
        let r = self.nrows() as usize;
        let c = self.ncols() as usize;
        let mut out = Vec::<String>::with_capacity(r);

        for i in 0usize..r {
            let mut row = Vec::<String>::with_capacity(c+2);
            row.push("[".to_string());
            for j in 0usize..c {
                row.push(format!(" {} ", self.get_entry(i, j)));
            }
            if i == r-1 {
                row.push("]".to_string());
            } else {
                row.push("]\n".to_string());
            }
            out.push(row.join(""));
        }
        out.join("")
    }

    fn entries(&self) -> Vec<<Self as VectorSpaceElement>::BaseRingElement> {
        let r = self.nrows() as usize;
        let c = self.ncols() as usize;
        let mut out = Vec::<<Self as VectorSpaceElement>::BaseRingElement>::with_capacity(r*c);

        for i in 0usize..r {
            for j in 0usize..c {
                out.push(self.get_entry(i, j));
            }
        }
        out
    }

    // is_invertible
    // submatrix (derive row/col)
    // hcat, vcat
    // trace, det, charpoly, minpoly, rank
    // rref, solve, nullspace
}

/* would work if MatrixSpaceElement<T> etc
impl<T: MatrixSpaceElement> From<&T> for Vec<<T as VectorSpaceElement>::BaseRingElement> {
    #[inline]
    fn from(x: &T) -> Vec<<T as VectorSpaceElement>::BaseRingElement> {
        x.entries()
    }
}*/

pub trait Ring: AdditiveGroup + Multiplicative {}
pub trait RingElement: AdditiveGroupElement + MultiplicativeElement + fmt::Display {}

pub trait PolynomialRing: Ring {
    type BaseRing: Ring;
    
    fn base_ring(&self) -> Self::BaseRing;

    fn gens(&self) -> Vec<<Self as Parent>::Element>;
}

pub trait PolynomialRingElement: RingElement {
    type BaseRingElement: RingElement; 
  
    fn len(&self) -> c_long;

    fn degree(&self) -> c_long;

    fn get_coeff(&self, i: usize) -> Self::BaseRingElement;
    
    fn set_coeff(&mut self, i: usize, coeff: &Self::BaseRingElement);

    #[inline]
    fn coefficients(&self) -> Vec<Self::BaseRingElement> {
        let len = self.len();

        let mut vec = Vec::<Self::BaseRingElement>::default();
        for i in 0..len {
            vec.push(self.get_coeff(i as usize));
        }
        vec
    }
}

pub trait Field: Ring {
    type BaseField: Field;
    
    fn base_field(&self) -> Self::BaseField;

    // TODO
    // gen, basis
}
pub trait FieldElement: RingElement {
    // TODO
    // norm(&self)
    // trace(&self)
}

pub trait NumberField: Field {} // + PolynomialRing (Q[x]/f)
pub trait NumberFieldElement: FieldElement {} // + PolynomialRingElement

/// An element of a `Parent`. In cases where the parent holds important context data we use the 
/// thread-safe [Arc] reference counter to avoid cleaning up the parent until all elements are 
/// dropped.
#[derive(Debug)] 
pub struct Elem<T: Parent> {
    pub data: <T::Element as Element>::Data,
}

#[derive(Debug, Hash, Clone)]
pub struct PolyRing<T: Ring + Debug + Hash + Clone> {
    pub phantom: PhantomData<T>,
    pub ctx: <T as Parent>::Context,
    pub var: Arc<String>,
}

/*
impl<T> PolyRing<T> where
    T: Ring + Debug + Hash + Clone
{
    pub fn new(base_ring: T, var: &str) -> PolyRing<T> {
        PolyRing { 
            phantom: PhantomData::<T>, 
            ctx: Arc::clone(&base_ring.ctx), 
            var: Arc::new(var.to_owned())
        }
    }
}*/

#[derive(Debug, Hash, Clone)]
pub struct MPolyRing<T: Ring + Debug + Hash + Clone> {
    pub phantom: PhantomData<T>,
    pub vars: Arc<Vec<String>>,
}

/*
impl<T> MPolyRing<T> where
    T: Ring + Debug + Hash + Clone
{
    pub fn new(_base_ring: T, vars: &[&str]) -> MPolyRing<T> {
        let mut vec = Vec::with_capacity(vars.len());
        for &var in vars.iter() {
            vec.push(var.to_owned());
        }
        MPolyRing { 
            phantom: PhantomData::<T>,
            vars: Arc::new(vec)
        }
    }
}*/

#[derive(Debug, Hash, Clone)]
pub struct MatSpace<T: Ring + Debug + Hash + Clone> {
    pub phantom: PhantomData<T>,
    pub nrows: c_long,
    pub ncols: c_long,
}

/*
impl<T> MatSpace<T> where
    T: Ring + Debug + Hash + Clone
{
    pub fn new(_base_ring: T, nrows: c_long, ncols: c_long) -> MatSpace<T> {
        MatSpace { phantom: PhantomData::<T>, nrows, ncols}
    }
}
*/
// quotient, frac field, extension
