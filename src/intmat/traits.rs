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


use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;

use flint_sys::fmpz_mat::fmpz_mat_struct;
use libc::c_long;

use crate::traits::*;
use crate::intmat::src::{IntMat, IntMatSpace};

// IntMatSpace //

impl Parent for IntMatSpace {
    type Data = ();
    type Element = IntMat;
}

// IntMat //

impl Element for IntMat {
    type Data = fmpz_mat_struct;
    type Parent = IntMatSpace;
}

impl Clone for IntMat {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_init_set(z.as_mut_ptr(), &self.data);
            IntMat { ctx: (), data: z.assume_init() }
        }
    }
}

impl fmt::Display for IntMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for IntMat {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz_mat::fmpz_mat_clear(self.as_mut_ptr()); }
    }
}

impl Hash for IntMat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Vec::from(self).hash(state);
    }
}

/* TODO: RatMat 
impl EvaluateProduct for Product<Integer> {
    type Output = Rational;
    fn evaluate(&self) -> Rational {
        let mut x = Rational::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.pow(k);
        }
        x
    }
}

impl EvaluateProductMod<Integer> for Product<Integer> {
    type Output = Result<Integer, ()>;
    #[inline]
    fn evaluate_mod(&self, modulus: Integer) -> Result<Integer, ()> {
        self.evaluate_mod(&modulus)
    }
}*/
