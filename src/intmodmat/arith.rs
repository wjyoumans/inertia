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


use std::mem::MaybeUninit;
use std::ops::*;

use libc::{c_long, c_ulong};
use rug::ops::*;

use crate::traits::*;
use crate::integer::src::Integer;
use crate::intmodmat::src::IntModMat;

impl_cmp_unsafe! {
    eq
    IntModMat
    flint_sys::fmpz_mod_mat::fmpz_mod_mat_equal
}

impl_unop_unsafe! {
    matrix_mod
    IntModMat
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpz_mod_mat::fmpz_mod_mat_neg
}
