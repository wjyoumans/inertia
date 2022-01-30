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

use crate::*;

impl Eq for IntMatSpace {}
impl PartialEq for IntMatSpace {
    fn eq(&self, other: &IntMatSpace) -> bool {
        if self.nrows == other.nrows && self.nrows == other.nrows {
            true
        } else {
            false
        }
    }
}

impl_cmp_unsafe! {
    eq
    IntMat
    flint_sys::fmpz_mat::fmpz_mat_equal
}

impl_cmp! {
    eq
    IntMat, RatMat
    {
        fn eq(&self, rhs: &RatMat) -> bool {
            RatMat::from(self).eq(rhs)
        }
    }
}

impl_unop_unsafe! {
    matrix
    IntMat
    Neg {neg}
    NegAssign {neg_assign}
    flint_sys::fmpz_mat::fmpz_mat_neg
}

impl_unop! {
    IntMat, RatMat
    Inv {inv}
    {
        fn inv(self) -> RatMat {
            let rr = RatMatSpace::init(self.nrows(), self.ncols());
            let mut res = rr.default();
            unsafe {
                flint_sys::fmpq_mat::fmpq_mat_set_fmpz_mat(res.as_mut_ptr(), self.as_ptr());
                flint_sys::fmpq_mat::fmpq_mat_inv(res.as_mut_ptr(), res.as_ptr());
            }
            res
        }
    }
}

impl_binop_unsafe! {
    matrix
    IntMat, IntMat, IntMat

    Add {add}
    AddAssign {add_assign}
    AddFrom {add_from}
    AssignAdd {assign_add}
    flint_sys::fmpz_mat::fmpz_mat_add;
    
    Sub {sub}
    SubAssign {sub_assign}
    SubFrom {sub_from}
    AssignSub {assign_sub}
    flint_sys::fmpz_mat::fmpz_mat_sub;
    
    Mul {mul}
    MulAssign {mul_assign}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    flint_sys::fmpz_mat::fmpz_mat_mul;
}

impl_binop_unsafe! {
    rhs_scalar
    op_assign
    IntMat, Integer, IntMat

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz_mat::fmpz_mat_scalar_mul_fmpz;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    flint_sys::fmpz_mat::fmpz_mat_scalar_mod_fmpz;
}

impl_binop_unsafe! {
    rhs_scalar
    IntMat, Integer, RatMat

    Div {div}
    AssignDiv {assign_div}
    flint_sys::fmpq_mat::fmpq_mat_set_fmpz_mat_div_fmpz;
}

impl_binop_unsafe! {
    lhs_scalar
    op_from
    Integer, IntMat, IntMat

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mat_fmpz_scalar_mul;
}

impl_binop_unsafe! {
    rhs_scalar
    op_assign
    IntMat, u64 {u64 u32 u16 u8}, IntMat

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz_mat::fmpz_mat_scalar_mul_ui;
    
    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_mat_scalar_mod_ui;

    Pow {pow}
    PowAssign {pow_assign}
    AssignPow {assign_pow}
    flint_sys::fmpz_mat::fmpz_mat_pow;
}

/* TODO: RatMat
impl_binop_unsafe! {
    rhs_scalar
    IntMat, Integer, RatMat

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mat_scalar_div_fmpz;
}*/

impl_binop_unsafe! {
    rhs_scalar
    op_assign
    IntMat, i64 {i64 i32 i16 i8}, IntMat

    Mul {mul}
    MulAssign {mul_assign}
    AssignMul {assign_mul}
    flint_sys::fmpz_mat::fmpz_mat_scalar_mul_si;

    Rem {rem}
    RemAssign {rem_assign}
    AssignRem {assign_rem}
    fmpz_mat_scalar_mod_si;
}

/* TODO: RatMat
impl_binop_unsafe! {
    rhs_scalar
    IntMat, Integer, RatMat

    Div {div}
    DivAssign {div_assign}
    AssignDiv {assign_div}
    fmpz_mat_scalar_div_fmpz;
}*/

impl_binop_unsafe! {
    lhs_scalar
    op_from
    u64 {u64 u32 u16 u8}, IntMat, IntMat

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mat_ui_scalar_mul;
}

impl_binop_unsafe! {
    lhs_scalar
    op_from
    i64 {i64 i32 i16 i8}, IntMat, IntMat

    Mul {mul}
    MulFrom {mul_from}
    AssignMul {assign_mul}
    fmpz_mat_si_scalar_mul;
}

#[inline]
unsafe fn fmpz_mat_fmpz_scalar_mul(
    res: *mut flint_sys::fmpz_mat::fmpz_mat_struct,
    f: *const flint_sys::fmpz::fmpz,
    g: *const flint_sys::fmpz_mat::fmpz_mat_struct)
{
    flint_sys::fmpz_mat::fmpz_mat_scalar_mul_fmpz(res, g, f);
}

#[inline]
unsafe fn fmpz_mat_ui_scalar_mul(
    res: *mut flint_sys::fmpz_mat::fmpz_mat_struct,
    f: c_ulong,
    g: *const flint_sys::fmpz_mat::fmpz_mat_struct)
{
    flint_sys::fmpz_mat::fmpz_mat_scalar_mul_ui(res, g, f);
}

#[inline]
unsafe fn fmpz_mat_si_scalar_mul(
    res: *mut flint_sys::fmpz_mat::fmpz_mat_struct,
    f: c_long,
    g: *const flint_sys::fmpz_mat::fmpz_mat_struct)
{
    flint_sys::fmpz_mat::fmpz_mat_scalar_mul_si(res, g, f);
}

#[inline]
unsafe fn fmpz_mat_scalar_mod_ui(
    res: *mut flint_sys::fmpz_mat::fmpz_mat_struct,
    f: *const flint_sys::fmpz_mat::fmpz_mat_struct,
    g: c_ulong)
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_ui(z.as_mut_ptr(), g);
    flint_sys::fmpz_mat::fmpz_mat_scalar_mod_fmpz(res, f, z.as_ptr());
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
}

#[inline]
unsafe fn fmpz_mat_scalar_mod_si(
    res: *mut flint_sys::fmpz_mat::fmpz_mat_struct,
    f: *const flint_sys::fmpz_mat::fmpz_mat_struct,
    g: c_long)
{
    let mut z = MaybeUninit::uninit();
    flint_sys::fmpz::fmpz_init_set_si(z.as_mut_ptr(), g);
    flint_sys::fmpz_mat::fmpz_mat_scalar_mod_fmpz(res, f, z.as_ptr());
    flint_sys::fmpz::fmpz_clear(z.as_mut_ptr());
}
