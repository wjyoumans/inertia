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

use crate::*;


impl AsRef<IntMat> for IntMat {
    fn as_ref(&self) -> &IntMat {
        self
    }
}

impl fmt::Display for IntMatSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The space of {}x{} matrices over the integers", self.nrows(), self.ncols())
    }
}

impl Clone for IntMat {
    fn clone(&self) -> Self {
        let mut z = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_mat::fmpz_mat_init_set(z.as_mut_ptr(), self.as_ptr());
            IntMat { data: z.assume_init() }
        }
    }
}

impl fmt::Display for IntMat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Hash for IntMat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entries().hash(state);
    }
}