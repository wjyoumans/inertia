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
//use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::sync::Arc;

use arb_sys::acb::acb_struct;
use libc::c_long;
//use rug::ops::Pow;
//use rustc_hash::FxHashMap;

use crate::traits::*;
//use crate::product::src::Product;
use crate::complex::src::{Complex, ComplexField};

// RealField //

impl Parent for ComplexField {
    type Data = Arc<c_long>;
    type Element = Complex;
}

// Real //

impl Element for Complex {
    type Data = acb_struct;
    type Parent = ComplexField;
}

impl Clone for Complex {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe { 
            arb_sys::acb::acb_init(z.as_mut_ptr());
            arb_sys::acb::acb_set(z.as_mut_ptr(), self.as_ptr());
            Complex { ctx: Arc::clone(&self.ctx), data: z.assume_init() }
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            arb_sys::acb::acb_init(z.as_mut_ptr());
            Complex { ctx: Arc::new(crate::REAL_DEFAULT_PREC), data: z.assume_init() }
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Drop for Complex {
    fn drop(&mut self) {
        unsafe { arb_sys::acb::acb_clear(self.as_mut_ptr());}
    }
}

/*
impl Hash for Real {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_ui_vector().hash(state);
    }
}

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
}

impl EvaluateProductMod<&Integer> for Product<Integer> {
    type Output = Result<Integer, ()>;
    fn evaluate_mod(&self, modulus: &Integer) -> Result<Integer, ()> {
        let mut x = Integer::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.powm(k, modulus)?;
            x %= modulus;
        }
        Ok(x)
    }
}
*/
