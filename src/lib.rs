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


//! Inertia is a computational mathematics library for Rust.
//!
//! ## Dependencies
//!
//! Inertia relies heavily on the C libraries [Flint](https://flintlib.org/doc/),
//! [Arb](https://arblib.org/), and [Antic](https://github.com/wbhart/antic). Currently, all three
//! libraries must be installed on your machine. On Linux it is recommended to put them in
//! `\usr\local\lib`. See the FFI crates [flint-sys](https://crates.io/crates/flint-sys), 
//! [arb-sys](https://crates.io/crates/arb-sys), and [antic-sys](https://crates.io/crates/antic-sys) 
//! for details on suggested versions.
//!
//!
//! ## Features
//!
//! This is a checklist of the main intended features and their current implementation progress.
//! Features marked with an asterisk have their basic implementation done but need work on
//! additional functions, arithmetic, conversion, etc.
//!
//! - [x] arbitrary precision integers
//! - [x] rational numbers
//! - [ ] \*real numbers
//! - [ ] \*complex numbers
//! - [ ] \*integers mod n
//! - [ ] \*finite fields
//! - [ ] p-adic/q-adic numbers
//! - [ ] polynomials
//!     - [x] integer polynomials
//!     - [x] rational polynomials
//!     - [ ] real polynomials
//!     - [ ] complex polynomials 
//!     - [ ] \*polynomials over integers mod n
//!     - [ ] \*polynomials over finite fields
//!     - [ ] polynomials over p-adic/q-adics
//! - [ ] matrices
//!     - [x] integer matrices
//!     - [x] rational matrices
//!     - [ ] real matrices
//!     - [ ] complex matrices 
//!     - [ ] \*matrices over integers mod n
//!     - [ ] \*matrices over finite fields
//!     - [ ] matrices over p-adic/q-adics
//! - [ ] multivariate polynomials
//! - [ ] \*rational functions
//! - [ ] number fields


#![feature(min_specialization)]

const REAL_DEFAULT_PREC: libc::c_long = 10;

#[macro_use]
pub(crate) mod macros;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

/// Trait definitions for operations and algebraic structures.
pub mod traits;

pub mod product;

pub mod integer;
pub mod intpol;
pub mod intmat;

pub mod rational;
pub mod ratpol;
pub mod ratmat;

pub mod real;

pub mod complex;

pub mod intmod;
pub mod intmodpol;
pub mod intmodmat;

pub mod finfld;
pub mod finfldpol;
pub mod finfldmat;

pub mod ratfunc;

pub mod numfld;

pub mod prelude { 
    //! A prelude for glob importing.
    
    pub use rug::ops::*;
    pub use crate::traits::*;

    pub use crate::integer::src::*;
    pub use super::int;

    pub use crate::rational::src::*;
    pub use super::rat;
    
    pub use crate::real::src::*;
    pub use super::real;

    pub use crate::complex::src::*;
    pub use super::complex;

    pub use crate::intmod::src::*;
    pub use super::intmod;
    
    pub use crate::finfld::src::*;
    pub use super::finfld;

    pub use crate::intpol::src::*;
    pub use super::intpol;

    pub use crate::ratpol::src::*;
    pub use super::ratpol;
    
    pub use crate::ratfunc::src::*;
    //pub use super::ratfunc;

    pub use crate::intmat::src::*;
    //pub use super::intmat;
    
    pub use crate::ratmat::src::*;
    //pub use super::ratmat;
}

pub use prelude::*;
