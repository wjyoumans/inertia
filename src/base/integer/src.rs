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
use std::mem::MaybeUninit;
use std::ops::{Rem, RemAssign};

//use flint_sys::flint::{flint_rand_s, flint_bitcnt_t};
use flint_sys::fmpz::fmpz;
use libc::{c_int, c_long, c_ulong};
use num_traits::Zero;
use rug::ops::Pow;
use rustc_hash::FxHashMap;

use crate::*;


/// An integer ring. For consistency with other types, `IntegerRing` (aka `Integers`) can be 
/// initialized and used as an [Integer] factory.
///
/// The functions and methods used here are designed so that the syntax of working with any
/// algebraic structure will always be consistent.
///
/// All algebraic structures can be initialized using the `init` function which will take a
/// different number of arguments depending on the structure. The `default` method will always
/// return zero when we have an additive structure, and `new` will always be the standard element
/// constructor.
///
/// ```
/// use inertia::prelude::*;
///
/// let zz = Integers::init();
///
/// // Initialize a new `Integer` as zero.
/// let z1 = zz.default();
///
/// // Initialize a new `Integer` and set it to zero (makes an additional call compared to `default`)
/// let z2 = zz.new(0);
///
/// assert_eq!(z1, z2);
/// ```
#[derive(Default, Debug, Hash, Clone, Copy)]
pub struct IntegerRing {}
pub type Integers = IntegerRing;

impl Parent for IntegerRing {
    type Element = Integer;
    type Context = ();

    /// Return the default value of the ring (zero whenever we have an additive structure).
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let zz = Integers::init();
    /// let z = zz.default();
    /// assert_eq!(z, 0);
    /// ```
    #[inline]
    fn default(&self) -> Integer {
        Integer::default()
    }
}

impl Additive for IntegerRing {
    /// Return the additive identity zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let zz = Integers::init();
    /// assert_eq!(zz.zero(), zz.new(0));
    /// ```
    #[inline]
    fn zero(&self) -> Integer {
        Integer::default()
    }
}

impl Multiplicative for IntegerRing {
    /// Return the multiplicative identity one.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let zz = Integers::init();
    /// assert_eq!(zz.one(), zz.new(1));
    /// ```
    #[inline]
    fn one(&self) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_one(res.as_mut_ptr()); }
        res
    }
}

impl AdditiveGroup for IntegerRing {}

impl Ring for IntegerRing {}

impl InitParent for IntegerRing {
    /// Initialize an `IntegerRing`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let zz = Integers::init();
    ///
    /// // Initialize a new `Integer` as zero.
    /// let z1 = zz.default();
    ///
    /// // Initialize a new `Integer` and set it to zero (makes an additional call compared to 
    /// // `default`)
    /// let z2 = zz.new(0);
    ///
    /// assert_eq!(z1, z2);
    /// ```
    #[inline]
    fn init() -> Self {
        IntegerRing {}
    }
}

impl NewElement<&Integer> for IntegerRing {
    #[inline]
    fn new(&self, x: &Integer) -> Integer {
        x.clone()
    }
}

impl<T> NewElement<T> for IntegerRing where
    T: Into<Integer>
{
    /// Construct a new `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let zz = Integers::init();
    ///
    /// let x = zz.new(12);
    /// assert_eq!(x, 12);
    ///
    /// let x = zz.new("101");
    /// assert_eq!(x, 101);
    /// ```
    #[inline]
    fn new(&self, x: T) -> Integer {
        x.into()
    }
}


/// An arbitrary precision integer.
///
/// Like all elements of algebraic structures in Inertia, an `Integer` can be constructed from a
/// parent using the `new` method of the [NewElement] trait.
///
/// ```
/// use inertia::prelude::*;
///
/// let zz = Integers::init();
/// let z = zz.new(123);
/// assert_eq!(z, 123);
/// ```
///
/// For convenience, we can also use the `From` and `Default` traits to avoid instantiating an
/// `IntegerRing`.
///
/// ```
/// use inertia::prelude::*;
///
/// let z1 = Integer::from(7);
/// let z2 = Integer::from("7");
/// assert_eq!(z1, z2);
///
/// let z = Integer::default();
/// assert_eq!(z, 0);
/// ```
///
/// The `int` macro is provided for making it even easier to instantiate an `Integer`:
///
/// ```
/// use inertia::prelude::*;
///
/// let z = int!();
/// assert_eq!(z, 0);
///
/// let z = int!(7);
/// assert_eq!(z, 7);
///
/// let z = int!("123");
/// assert_eq!(z, 123);
/// ```
#[derive(Debug)]
pub struct Integer {
    pub data: fmpz,
}

impl Drop for Integer {
    fn drop(&mut self) {
        unsafe { flint_sys::fmpz::fmpz_clear(&mut self.data);}
    }
}

impl Element for Integer {
    type Parent = IntegerRing;

    /// Return the parent.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!();
    /// let zz = x.parent();
    ///
    /// assert_eq!(zz, IntegerRing {});
    /// ```
    #[inline]
    fn parent(&self) -> IntegerRing {
        IntegerRing {}
    }
}

impl AdditiveElement for Integer {
    /// Determine if the `Integer` is the additive identity zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(0u32);
    /// assert!(x.is_zero());
    /// ```
    #[inline]
    fn is_zero(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_zero(self.as_ptr()) == 1 }
    }
}

impl MultiplicativeElement for Integer {
    /// Determine if the `Integer` is the multiplicative identity one.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1i16);
    /// assert!(x.is_one());
    /// ```
    #[inline]
    fn is_one(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_one(self.as_ptr()) == 1 }
    }
}

impl AdditiveGroupElement for Integer {}

impl RingElement for Integer {}

impl Integer {
    /// A reference to the underlying FFI struct. This is only needed to interface directly with 
    /// FLINT via the FFI.
    #[inline]
    pub fn as_ptr(&self) -> &fmpz {
        &self.data
    }
    
    /// A mutable reference to the underlying FFI struct. This is only needed to interface directly 
    /// with FLINT via the FFI.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> &mut fmpz {
        &mut self.data
    }

    /// Convert the `Integer` to a string in base `base`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1024);
    /// assert_eq!(x.to_str_radix(2), "10000000000")
    /// ```
    pub fn to_str_radix(&self, base: u8) -> String {
        unsafe {
            // Extra two bytes are for possible minus sign and null terminator
            let len = flint_sys::fmpz::fmpz_sizeinbase(self.as_ptr(), base as c_int) as usize + 2;

            // Allocate and write into a raw *c_char of the correct length
            let mut vector: Vec<u8> = Vec::with_capacity(len);
            vector.set_len(len);

            flint_sys::fmpz::fmpz_get_str(vector.as_mut_ptr() as *mut _, base as c_int, self.as_ptr());

            let mut first_nul = None;
            let mut index : usize = 0;
            for elem in &vector {
                if *elem == 0 {
                    first_nul = Some(index);
                    break;
                }
                index += 1;
            }
            let first_nul = first_nul.unwrap_or(len);

            vector.truncate(first_nul);
            match String::from_utf8(vector) {
                Ok(s)  => s,
                Err(_) => panic!("Flint returned invalid UTF-8!")
            }
        }
    }

    /// Check if the `Integer` is even.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(102);
    /// assert!(z.is_even());
    /// ```
    #[inline]
    pub fn is_even(&self) -> bool {
        unsafe {flint_sys::fmpz::fmpz_is_even(self.as_ptr()) == 1}
    }
    
    /// Check if the `Integer` is odd.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(103);
    /// assert!(z.is_odd());
    /// ```
    #[inline]
    pub fn is_odd(&self) -> bool {
        unsafe {flint_sys::fmpz::fmpz_is_odd(self.as_ptr()) == 1}
    }
    
    /// Returns -1 if the `Integer` is negative, +1 if the `Integer` is positive, and 0 otherwise.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(-12);
    /// assert_eq!(z.sign(), -1);
    ///
    /// let z = int!(0);
    /// assert_eq!(z.sign(), 0);
    ///
    /// let z = int!(12);
    /// assert_eq!(z.sign(), 1);
    /// ```
    #[inline]
    pub fn sign(&self) -> i32 {
        unsafe {
            flint_sys::fmpz::fmpz_sgn(self.as_ptr())
        }
    }

    /// Returns the absolute value of an `Integer`
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(-99);
    /// assert_eq!(z.abs(), int!(99));
    /// ```
    #[inline]
    pub fn abs(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_abs(res.as_mut_ptr(), self.as_ptr());
            res
        }
    }
    
    /// Set the input to its absolute value.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(-99);
    /// z.abs_assign();
    /// assert_eq!(z, int!(99));
    /// ```
    #[inline]
    pub fn abs_assign(&mut self) {
        unsafe {
            flint_sys::fmpz::fmpz_abs(self.as_mut_ptr(), self.as_ptr());
        }
    }
   
    /// Determines the size of the absolute value of an `Integer` in base `base` in terms of number
    /// of digits. The base can be between 2 and 62, inclusive.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(1000001);
    /// assert_eq!(8, z.sizeinbase(7));
    /// ```
    #[inline]
    pub fn sizeinbase(&self, base: u8) -> usize {
        unsafe { flint_sys::fmpz::fmpz_sizeinbase(self.as_ptr(), base as i32) as usize }
    }
   
    /// Returns the number of limbs required to store the absolute value of an `Integer`. Returns
    /// zero if the `Integer` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!("18446744073709551616");
    /// assert_eq!(2, z.size());
    /// ```
    #[inline]
    pub fn size(&self) -> c_long {
        unsafe { flint_sys::fmpz::fmpz_size(self.as_ptr()) }
    }
   
    /// Returns the number of bits required to store the absolute value of an `Integer`. Returns 
    /// zero if the `Integer` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(16);
    /// assert_eq!(x.bits(), 5);
    /// ```
    #[inline]
    pub fn bits(&self) -> c_ulong {
        unsafe { flint_sys::fmpz::fmpz_bits(self.as_ptr()) }
    }
   
    /// Determine if the `Integer` fits in a signed long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!("18446744073709551616");
    /// assert_eq!(z.fits_si(), false);
    /// ```
    #[inline]
    pub fn fits_si(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_fits_si(self.as_ptr()) == 1 }
    }
    
    /// Determine if the absolute value of an `Integer` fits in an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!("18446744073709551614");
    /// assert_eq!(z.abs_fits_ui(), true);
    /// ```
    #[inline]
    pub fn abs_fits_ui(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_abs_fits_ui(self.as_ptr()) == 1 }
    }
   
    /// Return an `Option` containing the input as a signed long (`libc::c_long`) if possible.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(-1234);
    /// assert_eq!(z.get_si().unwrap(), -1234);
    /// ```
    #[inline]
    pub fn get_si(&self) -> Option<c_long> {
        if self.fits_si() {
            unsafe { 
                Some(flint_sys::fmpz::fmpz_get_si(self.as_ptr()))
            }
        } else {
            None
        }
    }

    /// Return an `Option` containing the input as an unsigned long (`libc::c_ulong`) if possible. 
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(-1234);
    /// assert!(z.get_ui().is_none());
    /// ```
    #[inline]
    pub fn get_ui(&self) -> Option<c_ulong> {
        if self.sign() < 0 {
            return None;
        }
        if self.abs_fits_ui() {
            unsafe { 
                Some(flint_sys::fmpz::fmpz_get_ui(self.as_ptr())) 
            }
        } else {
            None
        }
    }

    /// Return a vector `A` of unsigned longs such that the original [Integer] can be written as 
    /// `a[0] + a[1]*x + ... + a[n-1]*x^(n-1)` where `x = 2^FLINT_BITS`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2).pow(65u8);
    /// let v = z.get_ui_vector();
    /// assert!(v == vec![0, 2]);
    ///
    /// let mut t = int!();
    /// t.set_ui_vector(v);
    /// assert_eq!(z, t);
    /// ```
    #[inline]
    pub fn get_ui_vector(&self) -> Vec<c_ulong> {
        assert!(self > &0);

        let n = self.size();
        let mut out = Vec::<c_ulong>::with_capacity(n as usize);
        unsafe {
            flint_sys::fmpz::fmpz_get_ui_array(out.as_mut_ptr(), n, self.as_ptr());
            out.set_len(n as usize);
        }
        out
    }

    /// Set `self` to the nonnegative [Integer] `vec[0] + vec[1]*x + ... + vec[n-1]*x^(n-1)` 
    /// where `x = 2^FLINT_BITS`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!();
    /// z.set_ui_vector(vec![0,2]);
    /// assert_eq!(z, int!(2).pow(65u8));
    /// ```
    #[inline]
    pub fn set_ui_vector(&mut self, vec: Vec<c_ulong>) {
        unsafe {
            flint_sys::fmpz::fmpz_set_ui_array(self.as_mut_ptr(), vec.as_ptr(), vec.len() as c_long);
        }
    }

    /// Sets the bit index `bit_index` of an `Integer`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(1024);
    /// z.setbit(0);
    /// assert_eq!(1025, z);
    /// ```
    #[inline]
    pub fn setbit(&mut self, bit_index: usize) {
        unsafe { flint_sys::fmpz::fmpz_setbit(self.as_mut_ptr(), bit_index as c_ulong) }
    }

    /// Test the bit index `bit_index` of an `Integer`. Return `true` if it is 1, `false` if it is
    /// zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(1025);
    /// assert!(z.testbit(0));
    /// ```
    #[inline]
    pub fn testbit(&self, bit_index: usize) -> bool {
        unsafe { flint_sys::fmpz::fmpz_tstbit(self.as_ptr(), bit_index as c_ulong) == 1 }
    }

    /*
    // TODO: All Rand functions need work.

    /// Not implemented.
    #[inline]
    pub fn rand_bits(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randbits(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_max_bits(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_max_bits_ui(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_unsigned(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_max_bits_non_zero(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_not_zero(res.as_mut_ptr(), &st, bt);}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand<T>(st: flint_rand_s, m: T) -> Integer where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randm(res.as_mut_ptr(), &st, m.as_ref().as_ptr());}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_mod<T>(st: flint_rand_s, m: T) -> Integer where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randtest_mod(res.as_mut_ptr(), &st, m.as_ref().as_ptr());}
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_mod_si<T>(st: flint_rand_s, m: T) -> Integer where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe { 
            flint_sys::fmpz::fmpz_randtest_mod_signed(res.as_mut_ptr(), &st, m.as_ref().as_ptr());
        }
        res
    }
    
    /// Not implemented.
    #[inline]
    pub fn rand_prime(st: flint_rand_s, bt: flint_bitcnt_t) -> Integer {
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_randprime(res.as_mut_ptr(), &st, bt, 1);}
        res
    }*/

    /// Outputs `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let f = int!(-1);
    /// assert_eq!(f.mul2_uiui(10, 3), -30);
    /// ```
    #[inline]
    pub fn mul2_uiui<S>(&self, x: S, y: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let x = x.try_into().expect("Input cannot be converted to an unsigned long.");
        let y = y.try_into().expect("Input cannot be converted to an unsigned long.");

        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_mul2_uiui(res.as_mut_ptr(), self.as_ptr(), x, y);
        }
        res
    }
    
    /// Set `self` to `self * x * y` where `x, y` can be converted to unsigned longs.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut f = int!(-1);
    /// f.mul2_uiui_assign(10, 3);
    /// assert_eq!(f, -30);
    /// ```
    #[inline]
    pub fn mul2_uiui_assign<S>(&mut self, x: S, y: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let x = x.try_into().expect("Input cannot be converted to an unsigned long.");
        let y = y.try_into().expect("Input cannot be converted to an unsigned long.");

        unsafe {
            flint_sys::fmpz::fmpz_mul2_uiui(self.as_mut_ptr(), self.as_ptr(), x, y);
        }
    }

    /// Output `self * 2^exp`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let g = int!(2);
    /// assert_eq!(g.mul_2exp(3), 16);
    /// ```
    #[inline]
    pub fn mul_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");

        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_mul_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
        
    }
    
    /// Compute `self * 2^exp` in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut g = int!(2);
    /// g.mul_2exp_assign(3);
    /// assert_eq!(g, 16);
    /// ```
    #[inline]
    pub fn mul_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_mul_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
        
    }

    /// Return `self + (x * y)`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.addmul(int!(3), int!(4)), 14);
    /// ```
    #[inline]
    pub fn addmul<T>(&self, x: T, y: T) -> Integer where 
        T: AsRef<Integer>
    {
        let mut res = self.clone();
        unsafe {
            flint_sys::fmpz::fmpz_addmul(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Compute `self + (x * y)` in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(2);
    /// z.addmul_assign(int!(3), int!(4));
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_assign<T>(&mut self, x: T, y: T) where 
        T: AsRef<Integer>
    {
        unsafe {
            flint_sys::fmpz::fmpz_addmul(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
    }
    
    /// Return `self + (x * y)` where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.addmul_ui(int!(3), 4), 14);
    /// ```
    #[inline]
    pub fn addmul_ui<S, T>(&self, x: T, y: S) -> Integer where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = self.clone();
        unsafe {
            flint_sys::fmpz::fmpz_addmul_ui(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
        res
    }
    
    /// Compute `self + (x * y)` in place where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(2);
    /// z.addmul_ui_assign(int!(3), 4);
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn addmul_ui_assign<S, T>(&mut self, x: T, y: S) where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_addmul_ui(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
    }
   
    /*
    /// Return `self + (x * y)` where `y` can be converted to a signed long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(14);
    /// assert_eq!(z.addmul_si(int!(3), -4), 2);
    /// ```
    #[inline]
    pub fn addmul_si<S, T>(&self, x: T, y: S) -> Integer where 
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to a signed long.");
        let mut res = self.clone();
        unsafe {
            flint_sys::fmpz::fmpz_addmul_si(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
        res
    }
   
    /// Compute `self + (x * y)` in place where `y` can be converted to a signed long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(14);
    /// z.addmul_si_assign(int!(3), -4);
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn addmul_si_assign<S, T>(&mut self, x: T, y: S) where 
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to a signed long.");
        unsafe {
            flint_sys::fmpz::fmpz_addmul_si(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
    }
    */
    
    /// Return `self - (x * y)`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(14);
    /// assert_eq!(z.submul(int!(3), int!(4)), 2);
    /// ```
    #[inline]
    pub fn submul<T>(&self, x: T, y: T) -> Integer where 
        T: AsRef<Integer>
    {
        let mut res = self.clone();
        unsafe {
            flint_sys::fmpz::fmpz_submul(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
        res
    }
    
    /// Compute `self - (x * y)` in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(14);
    /// z.submul_assign(int!(3), int!(4));
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn submul_assign<T>(&mut self, x: T, y: T) where 
        T: AsRef<Integer>
    {
        unsafe {
            flint_sys::fmpz::fmpz_submul(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y.as_ref().as_ptr()
            );
        }
    }
    
    /// Return `self - (x * y)` where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(14);
    /// assert_eq!(z.submul_ui(int!(3), 4), 2);
    /// ```
    #[inline]
    pub fn submul_ui<S, T>(&self, x: T, y: S) -> Integer where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = self.clone();
        unsafe {
            flint_sys::fmpz::fmpz_submul_ui(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
        res
    }
    
    /// Compute `self - (x * y)` in place where `y` can be converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(14);
    /// z.submul_ui_assign(int!(3), 4);
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn submul_ui_assign<S, T>(&mut self, x: T, y: S) where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_submul_ui(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
    }
   
    /*
    /// Return `self - (x * y)` where `y` can be converted to a signed long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.submul_si(int!(3), -4), 14);
    /// ```
    pub fn submul_si<S, T>(&self, x: T, y: S) -> Integer where 
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to a signed long.");
        let mut res = self.clone();
        unsafe {
            flint_sys::fmpz::fmpz_submul_si(
                res.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
        res
    }
   
    /// Compute `self - (x * y)` in place where `y` can be converted to a signed long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(2);
    /// z.submul_si_assign(int!(3), -4);
    /// assert_eq!(z, 14);
    /// ```
    pub fn submul_si_assign<S, T>(&mut self, x: T, y: S) where 
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let y = y.try_into().expect("Input cannot be converted to a signed long.");
        unsafe {
            flint_sys::fmpz::fmpz_submul_si(
                self.as_mut_ptr(), 
                x.as_ref().as_ptr(), 
                y
            );
        }
    }
    */

    /// Compute `(a * b) + (c * d)` in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!();
    /// z.fmma_assign(int!(1), int!(2), int!(3), int!(4));
    /// assert_eq!(z, 14);
    /// ```
    #[inline]
    pub fn fmma_assign<T>(&mut self, a: T, b: T, c: T, d: T) where
        T: AsRef<Integer>
    {
        unsafe {
            flint_sys::fmpz::fmpz_fmma(
                self.as_mut_ptr(), 
                a.as_ref().as_ptr(),
                b.as_ref().as_ptr(),
                c.as_ref().as_ptr(),
                d.as_ref().as_ptr()
            );
        }
    }
    
    /// Compute `(a * b) - (c * d)` in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!();
    /// z.fmms_assign(int!(4), int!(3), int!(2), int!(1));
    /// assert_eq!(z, 10);
    /// ```
    #[inline]
    pub fn fmms_assign<T>(&mut self, a: T, b: T, c: T, d: T) where
        T: AsRef<Integer>
    {
        unsafe {
            flint_sys::fmpz::fmpz_fmms(
                self.as_mut_ptr(), 
                a.as_ref().as_ptr(),
                b.as_ref().as_ptr(),
                c.as_ref().as_ptr(),
                d.as_ref().as_ptr()
            );
        }
    }

    /// Return the quotient and remainder of `self/other` rounding up towards infinity. Panics if
    /// `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// let (q, r) = x.cdiv_qr(int!(2));
    /// assert_eq!(q, 7);
    /// assert_eq!(r, -1);
    /// ```
    #[inline]
    pub fn cdiv_qr<T>(&self, other: T) -> (Integer, Integer) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut q = Integer::default();
        let mut r = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            )
        }
        (q, r)
    }

    /// Return the quotient `self/other` rounded up towards infinity. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.cdiv_q(int!(2)), 6);
    /// ```
    #[inline]
    pub fn cdiv_q<T>(&self, other: T) -> Integer where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
        res
    }
    
    /// Compute the quotient `self/other` in place, rounded up towards infinity. Panics if 
    /// `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.cdiv_q_assign(int!(2));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    pub fn cdiv_q_assign<T>(&mut self, other: T) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
    }

    /// Return the quotient `self/other` rounded up towards infinity where `other` can be converted 
    /// to an unsigned long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.cdiv_q_ui(2), 6);
    /// ```
    #[inline]
    pub fn cdiv_q_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q_ui(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
    
    /// Compute the quotient `self/other` in place where `other` can be converted to an unsigned 
    /// long, rounded up towards infinity. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.cdiv_q_ui_assign(2);
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    pub fn cdiv_q_ui_assign<S>(&mut self, other: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q_ui(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
    
    /// Return the quotient `self/other` rounded up towards infinity where `other` can be converted
    /// to a signed long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.cdiv_q_si(-2), -5);
    /// ```
    #[inline]
    pub fn cdiv_q_si<S>(&self, other: S) -> Integer where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to a signed long.");
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q_si(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
    
    /// Compute the quotient `self/other` in place where `other` can be converted to a signed long, 
    /// rounded up towards infinity. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.cdiv_q_si_assign(-2);
    /// assert_eq!(x, -5);
    /// ```
    #[inline]
    pub fn cdiv_q_si_assign<S>(&mut self, other: S) where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q_si(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
    
    /// Return the quotient `self/(2^exp)` rounded up towards infinity where `exp` can be
    /// converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1025);
    /// assert_eq!(x.cdiv_q_2exp(10), 2);
    /// ```
    #[inline]
    pub fn cdiv_q_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
    }
    
    /// Compute the quotient `self/(2^exp)` in place where `exp` can be converted to an unsigned 
    /// long, rounded up towards infinity.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(1025);
    /// x.cdiv_q_2exp_assign(10);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    pub fn cdiv_q_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_q_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
    }
    
    /// Return the remainder of `self/(2^exp)` where the remainder is non-positive.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1025);
    /// assert_eq!(x.cdiv_r_2exp(10), -1023);
    /// ```
    #[inline]
    pub fn cdiv_r_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_r_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
    }
    
    /// Compute the remainder of `self/(2^exp)` where the remainder is non-positive in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(1025);
    /// x.cdiv_r_2exp_assign(10);
    /// assert_eq!(x, -1023);
    /// ```
    #[inline]
    pub fn cdiv_r_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_cdiv_r_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
    }

    /// Returns the negative of the remainder from dividing `self` by `other`, rounding towards
    /// minus infinity. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(12);
    ///
    /// assert_eq!(x.cdiv_ui(5), 3);
    /// ```
    #[inline]
    pub fn cdiv_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            Integer::from(flint_sys::fmpz::fmpz_cdiv_ui(self.as_ptr(), other))
        }
    }
    
    /// Return the quotient and remainder of `self/other` rounding down towards negative infinity. 
    /// Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// let (q, r) = x.fdiv_qr(int!(2));
    /// assert_eq!(q, 6);
    /// assert_eq!(r, 1);
    /// ```
    #[inline]
    pub fn fdiv_qr<T>(&self, other: T) -> (Integer, Integer) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut q = Integer::default();
        let mut r = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            )
        }
        (q, r)
    }

    /// Return the quotient `self/other` rounded down towards negative infinity. Panics if `other` 
    /// is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// assert_eq!(x.fdiv_q(int!(2)), 6);
    /// ```
    #[inline]
    pub fn fdiv_q<T>(&self, other: T) -> Integer where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
        res
    }
    
    /// Compute the quotient `self/other` in place, rounded down towards negative infinity. Panics 
    /// if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(13);
    /// x.fdiv_q_assign(int!(2));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    pub fn fdiv_q_assign<T>(&mut self, other: T) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
    }

    /// Return the quotient `self/other` rounded down towards negative infinity where `other` can 
    /// be converted to an unsigned long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.fdiv_q_ui(2), 5);
    /// ```
    #[inline]
    pub fn fdiv_q_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q_ui(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
    
    /// Compute the quotient `self/other` in place where `other` can be converted to an unsigned 
    /// long, rounded down towards negative infinity. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.fdiv_q_ui_assign(2);
    /// assert_eq!(x, 5);
    /// ```
    #[inline]
    pub fn fdiv_q_ui_assign<S>(&mut self, other: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q_ui(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
    
    /// Return the quotient `self/other` rounded down towards negative infinity where `other` can 
    /// be converted to a signed long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.fdiv_q_si(-2), -6);
    /// ```
    #[inline]
    pub fn fdiv_q_si<S>(&self, other: S) -> Integer where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to a signed long.");
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q_si(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
    
    /// Compute the quotient `self/other` in place where `other` can be converted to a signed long, 
    /// rounded down towards negative infinity. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.fdiv_q_si_assign(-2);
    /// assert_eq!(x, -6);
    /// ```
    #[inline]
    pub fn fdiv_q_si_assign<S>(&mut self, other: S) where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q_si(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
    
    /// Return the quotient `self/(2^exp)` rounded down towards negative infinity where `exp` can 
    /// be converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1025);
    /// assert_eq!(x.fdiv_q_2exp(10), 1);
    /// ```
    #[inline]
    pub fn fdiv_q_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
    }
    
    /// Compute the quotient `self/(2^exp)` in place where `exp` can be converted to an unsigned 
    /// long, rounded down towards negative infinity.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(1025);
    /// x.fdiv_q_2exp_assign(10);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    pub fn fdiv_q_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_q_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
    }
    
    /// Return the remainder of `self/(2^exp)` where the remainder is non-negative.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1025);
    /// assert_eq!(x.fdiv_r_2exp(10), 1);
    /// ```
    #[inline]
    pub fn fdiv_r_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_r_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
    }
    
    /// Compute the remainder of `self/(2^exp)` where the remainder is non-negative, in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(1025);
    /// x.fdiv_r_2exp_assign(10);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    pub fn fdiv_r_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_fdiv_r_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
    }

    /// Returns the remainder of `self` modulo `other` where `other` can be converted to an
    /// unsigned long, without changing `self`. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// assert_eq!(x.fdiv_ui(2), 1);
    /// ```
    #[inline]
    pub fn fdiv_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            Integer::from(flint_sys::fmpz::fmpz_fdiv_ui(self.as_ptr(), other))
        }
    }

    /// Return the quotient and remainder of `self/other` rounded towards zero.
    /// Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// let (q, r) = x.tdiv_qr(int!(2));
    /// assert_eq!(q, 6);
    /// assert_eq!(r, 1);
    /// ```
    #[inline]
    pub fn tdiv_qr<T>(&self, other: T) -> (Integer, Integer) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut q = Integer::default();
        let mut r = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            )
        }
        (q, r)
    }

    /// Return the quotient `self/other` rounded towards zero. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// assert_eq!(x.tdiv_q(int!(2)), 6);
    /// ```
    #[inline]
    pub fn tdiv_q<T>(&self, other: T) -> Integer where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
        res
    }
    
    /// Compute the quotient `self/other` in place, rounded towards zero. Panics if `other` 
    /// is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(13);
    /// x.tdiv_q_assign(int!(2));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    pub fn tdiv_q_assign<T>(&mut self, other: T) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
        }
    }

    /// Return the quotient `self/other` rounded towards zero where `other` can be converted to an 
    /// unsigned long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.tdiv_q_ui(2), 5);
    /// ```
    #[inline]
    pub fn tdiv_q_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q_ui(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
    
    /// Compute the quotient `self/other` in place where `other` can be converted to an unsigned 
    /// long, rounded towards zero. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.tdiv_q_ui_assign(2);
    /// assert_eq!(x, 5);
    /// ```
    #[inline]
    pub fn tdiv_q_ui_assign<S>(&mut self, other: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q_ui(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
    
    /// Return the quotient `self/other` rounded towards zero where `other` can be converted to a 
    /// signed long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(11);
    /// assert_eq!(x.tdiv_q_si(-2), -5);
    /// ```
    #[inline]
    pub fn tdiv_q_si<S>(&self, other: S) -> Integer where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q_si(res.as_mut_ptr(), self.as_ptr(), other);
        }
        res
    }
    
    /// Compute the quotient `self/other` in place where `other` can be converted to a signed long, 
    /// rounded towards zero. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(11);
    /// x.tdiv_q_si_assign(-2);
    /// assert_eq!(x, -5);
    /// ```
    #[inline]
    pub fn tdiv_q_si_assign<S>(&mut self, other: S) where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q_si(self.as_mut_ptr(), self.as_ptr(), other);
        }
    }
    
    /// Return the quotient `self/(2^exp)` rounded towards zero where `exp` can be converted to an 
    /// unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1025);
    /// assert_eq!(x.tdiv_q_2exp(10), 1);
    /// ```
    #[inline]
    pub fn tdiv_q_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
    }
    
    /// Compute the quotient `self/(2^exp)` in place where `exp` can be converted to an unsigned 
    /// long, rounded towards zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(1025);
    /// x.tdiv_q_2exp_assign(10);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    pub fn tdiv_q_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_q_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
    }
    
    /// Return the remainder of `self/(2^exp)` rounded towards zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(1025);
    /// assert_eq!(x.tdiv_r_2exp(10), 1);
    /// ```
    #[inline]
    pub fn tdiv_r_2exp<S>(&self, exp: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_r_2exp(res.as_mut_ptr(), self.as_ptr(), exp);
        }
        res
    }
    
    /// Compute the remainder of `self/(2^exp)` where the remainder has the same sign as `self`, 
    /// in place.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut x = int!(1025);
    /// x.tdiv_r_2exp_assign(10);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    pub fn tdiv_r_2exp_assign<S>(&mut self, exp: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_tdiv_r_2exp(self.as_mut_ptr(), self.as_ptr(), exp);
        }
    }

    /// Returns the absolute value of the remainder of `self/other` where `other` can be converted 
    /// to an unsigned long. Panics if `other` is zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(13);
    /// assert_eq!(x.tdiv_ui(2), 1);
    /// ```
    #[inline]
    pub fn tdiv_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        unsafe {
            Integer::from(flint_sys::fmpz::fmpz_tdiv_ui(self.as_ptr(), other))
        }
    }
    
    /// Return the quotient and remainder of `self/other` rounded towards the nearest integer.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(-7);
    /// let (q, r) = x.ndiv_qr(int!(3));
    /// assert_eq!(q, -2);
    /// assert_eq!(r, -1);
    /// ```
    #[inline]
    pub fn ndiv_qr<T>(&self, other: T) -> (Integer, Integer) where
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        let mut q = Integer::default();
        let mut r = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_ndiv_qr(
                q.as_mut_ptr(), 
                r.as_mut_ptr(), 
                self.as_ptr(), 
                other.as_ptr()
            );
        }
        (q, r)
    }
    
    /// Return the quotient of `self/other` rounded towards the nearest integer.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let x = int!(-7);
    /// assert_eq!(x.ndiv_q(int!(3)), -2);
    /// ```
    #[inline]
    pub fn ndiv_q<T>(&self, other: T) -> Integer where
        T: AsRef<Integer>
    {
        let (q, _) = self.ndiv_qr(other);
        q
    }
   
    /// Exact division of `self/other`. Panics if the division is not exact.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(8);
    /// assert_eq!(z.divexact(int!(2)), 4);
    /// ```
    #[inline]
    pub fn divexact<T>(&self, other: T) -> Integer where 
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        if self.rem(other) != 0 {
            panic!("Division is not exact.");
        } else {
            let mut res = Integer::default();
            unsafe { 
                flint_sys::fmpz::fmpz_divexact(res.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            }
            res
        }
    }
    
    /// Exact division of `self/other` in place. Panics if the division is not exact.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(8);
    /// z.divexact_assign(int!(2));
    /// assert_eq!(z, 4);
    /// ```
    #[inline]
    pub fn divexact_assign<T>(&mut self, other: T) where 
        T: AsRef<Integer>
    {
        let other = other.as_ref();
        assert!(!other.is_zero());
        if (&*self).rem(other) != 0 {
            panic!("Division is not exact.");
        } else {
            unsafe { 
                flint_sys::fmpz::fmpz_divexact(self.as_mut_ptr(), self.as_ptr(), other.as_ptr());
            }
        }
    }
    
    /// Exact division of `self/other` where `other` can be converted to an unsigned long. Panics
    /// if the division is not exact.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(8);
    /// assert_eq!(z.divexact_ui(2), 4);
    /// ```
    #[inline]
    pub fn divexact_ui<S>(&self, other: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        if self.rem(other) != 0 {
            panic!("Division is not exact.");
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_divexact_ui(res.as_mut_ptr(), self.as_ptr(), other);}
            res
        }
    }
    
    /// Exact division of `self/other` in place where `other` can be converted to an unsigned 
    /// long. Panics if the division is not exact.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(8);
    /// z.divexact_ui_assign(2);
    /// assert_eq!(z, 4);
    /// ```
    #[inline]
    pub fn divexact_ui_assign<S>(&mut self, other: S) where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to an unsigned long.");
        assert!(!other.is_zero());
        if (&*self).rem(other) != 0 {
            panic!("Division is not exact.");
        } else {
            unsafe { flint_sys::fmpz::fmpz_divexact_ui(self.as_mut_ptr(), self.as_ptr(), other);}
        }
    }
    
    /// Exact division of `self/other` where `other` can be converted to a signed long. Panics if 
    /// the division is not exact.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(8);
    /// assert_eq!(z.divexact_si(-2), -4);
    /// ```
    #[inline]
    pub fn divexact_si<S>(&self, other: S) -> Integer where
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to a signed long.");
        assert!(!other.is_zero());
        if self.rem(other.abs()) != 0 {
            panic!("Division is not exact.");
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_divexact_si(res.as_mut_ptr(), self.as_ptr(), other);}
            res
        }
    }
    
    /// Exact division of `self/other` in place where `other` can be converted to a signed 
    /// long. Panics if the division is not exact.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(8);
    /// z.divexact_si_assign(-2);
    /// assert_eq!(z, -4);
    /// ```
    #[inline]
    pub fn divexact_si_assign<S>(&mut self, other: S) where 
        S: TryInto<c_long>,
        S::Error: fmt::Debug,
    {
        let other = other.try_into().expect("Input cannot be converted to a signed long.");
        assert!(!other.is_zero());
        if (&*self).rem(other.abs()) != 0 {
            panic!("Division is not exact.");
        } else {
            unsafe { flint_sys::fmpz::fmpz_divexact_si(self.as_mut_ptr(), self.as_ptr(), other);}
        }
    }
   
    /// The symmetric remainder of an integer modulo `n` will be in the range 
    /// `[-(n-1)/2, ..., (n-1)/2]` symmetric around zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(9);
    /// assert_eq!(z.srem(int!(11)), -2);
    /// ```
    #[inline]
    pub fn srem<T>(&self, modulus: T) -> Integer where
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_smod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        }
        res
    }
    
    /// The symmetric remainder of an integer modulo `n` will be in the range 
    /// `[-(n-1)/2, ..., (n-1)/2]` symmetric around zero. The value is assigned to the input.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(9);
    /// z.srem_assign(int!(11));
    /// assert_eq!(z, -2);
    /// ```
    #[inline]
    pub fn srem_assign<T>(&mut self, modulus: T) where
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);
        unsafe {
            flint_sys::fmpz::fmpz_smod(self.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        }
    }
   
    /// Raises an `Integer` to the power `exp` modulo `modulus`. Panics if the exponent is negative 
    /// and no inverse exists.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.powm(int!(3), int!(5)), 3);
    /// ```
    #[inline]
    pub fn powm<T>(&self, exp: T, modulus: T) -> Integer where
        T: AsRef<Integer>,
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);
        let exp = exp.as_ref();
        if exp < &0 && !self.is_coprime(modulus) {
            panic!("Input is not invertible mod m.");
        } else {
            let mut res = Integer::default();
            unsafe {
                flint_sys::fmpz::fmpz_powm(
                    res.as_mut_ptr(), 
                    self.as_ptr(), 
                    exp.as_ptr(), 
                    modulus.as_ptr()
                );
            }
            res
        }
    }

    /// Raises an `Integer` to the power `exp` modulo `modulus`, assigning it to the input. Panics
    /// if the exponent is negative and no inverse exists.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(2);
    /// z.powm_assign(int!(3), int!(5));
    /// assert_eq!(z, 3);
    /// ```
    #[inline]
    pub fn powm_assign<T>(&mut self, exp: T, modulus: T) where
        T: AsRef<Integer>,
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);
        let exp = exp.as_ref();
        if exp < &0 && !self.is_coprime(modulus) {
            panic!("Input is not invertible mod m.");
        } else {
            unsafe {
                flint_sys::fmpz::fmpz_powm(
                    self.as_mut_ptr(), 
                    self.as_ptr(), 
                    exp.as_ptr(), 
                    modulus.as_ptr()
                );
            }
        }
    }
    
    /// Raises an `Integer` to the power `exp` modulo `modulus` where `exp` can be converted
    /// to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.powm_ui(3, int!(5)), 3);
    /// ```
    #[inline]
    pub fn powm_ui<S, T>(&self, exp: S, modulus: T) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe {
            flint_sys::fmpz::fmpz_powm_ui(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                exp, 
                modulus.as_ptr()
            );
        }
        res
    }
    
    /// Raises an `Integer` to the power `exp` modulo `modulus` in place where `exp` can be 
    /// converted to an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(2);
    /// z.powm_ui_assign(3, int!(5));
    /// assert_eq!(z, 3);
    /// ```
    #[inline]
    pub fn powm_ui_assign<S, T>(&mut self, exp: S, modulus: T) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);
        let exp = exp.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe {
            flint_sys::fmpz::fmpz_powm_ui(
                self.as_mut_ptr(), 
                self.as_ptr(), 
                exp, 
                modulus.as_ptr()
            );
        }
    }
    
    /// Return true if `self` divides `other`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(5);
    /// assert!(z.divides(int!(10)));
    /// assert!(!z.divides(int!(11)));
    /// ```
    #[inline]
    pub fn divides<T>(&self, other: T) -> bool where
        T: AsRef<Integer>
    {
        unsafe { 
            flint_sys::fmpz::fmpz_divisible(other.as_ref().as_ptr(), self.as_ptr()) == 1 
        }
    }

    /*
    // TODO: use Real?
    /// Compute the natural logarithm of an [Integer] as a double precision float. If the input 
    /// is less than or equal to zero the [Result] will be an [Err]. (For logarithms of negative 
    /// integers use (the Complex/arb crate, not yet complete.) 
    #[inline]
    pub fn log(&self) -> Result<f64, ()> {
        if self <= &0 {
            Err(())
        } else {
            unsafe { 
                Ok(flint_sys::fmpz::fmpz_dlog(self.as_ptr()))
            }
        }
    }*/

    /// Return the logarithm of `self` with integer base `base`, rounded up towards infinity. Panics 
    /// if `self < 1` or `base < 2`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(3);
    /// assert_eq!(z.clog(int!(2)), 2);
    /// ```
    #[inline]
    pub fn clog<T>(&self, base: T) -> Integer where
        T: AsRef<Integer>
    {
        let base = base.as_ref();
        if self < &1 {
            panic!("Logarithm input is less than one.");
        } else if base < &2 {
            panic!("Logarithm base is less than two.");
        } else {
            unsafe { 
                Integer::from(
                    flint_sys::fmpz::fmpz_clog(self.as_ptr(), base.as_ptr())
                )
            }
        }
    }

    /// Return the logarithm of `self` with integer base `base` rounded up towards infinity where
    /// `base` can be converted to an unsigned long. Panics if `self < 1` or `base < 2`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(3);
    /// assert_eq!(z.clog_ui(2), 2);
    /// ```
    #[inline]
    pub fn clog_ui<S>(&self, base: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let base = base.try_into().expect("Input cannot be converted to an unsigned long.");
        if self < &1 {
            panic!("Logarithm input is less than one.");
        } else if base < 2 {
            panic!("Logarithm base is less than two.");
        } else {
            unsafe { 
                Integer::from(
                    flint_sys::fmpz::fmpz_clog_ui(self.as_ptr(), base)
                )
            }
        }
    }
    
    /// Return the logarithm of `self` with integer base `base`, rounded down towards negative 
    /// infinity. Panics if `self < 1` or `base < 2`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(3);
    /// assert_eq!(z.flog(int!(2)), 1);
    /// ```
    #[inline]
    pub fn flog<T>(&self, base: T) -> Integer where
        T: AsRef<Integer>
    {
        let base = base.as_ref();
        if self < &1 {
            panic!("Logarithm input is less than one.");
        } else if base < &2 {
            panic!("Logarithm base is less than two.");
        } else {
            unsafe { 
                Integer::from(
                    flint_sys::fmpz::fmpz_flog(self.as_ptr(), base.as_ptr())
                )
            }
        }
    }

    /// Return the logarithm of `self` with integer base `base` rounded down towards negative 
    /// infinity where `base` can be converted to an unsigned long. Panics if `self < 1` or 
    /// `base < 2`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(3);
    /// assert_eq!(z.flog_ui(2), 1);
    /// ```
    #[inline]
    pub fn flog_ui<S>(&self, base: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug,
    {
        let base = base.try_into().expect("Input cannot be converted to an unsigned long.");
        if self < &1 {
            panic!("Logarithm input is less than one.");
        } else if base < 2 {
            panic!("Logarithm base is less than two.");
        } else {
            unsafe { 
                Integer::from(
                    flint_sys::fmpz::fmpz_flog_ui(self.as_ptr(), base)
                )
            }
        }
    }

    /// Return the square root of `self` modulo `n` if it exists.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(3);
    /// assert_eq!(z.sqrtmod(int!(13)).unwrap(), 4);
    /// ```
    #[inline]
    pub fn sqrtmod<T>(&self, n: T) -> Option<Integer> where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe { 
            let r = flint_sys::fmpz::fmpz_sqrtmod(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                n.as_ref().as_ptr()
            );
            if r == 0 {
                None
            } else {
                Some(res)
            }
        }
    }

    /// Return the integer part `a` of the square root of an positive integer and it's remainder 
    /// `b`, that is, the difference `self - b^2`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(7);
    /// let (a, b) = z.sqrtrem();
    /// assert_eq!(a, 2);
    /// assert_eq!(b, 3);
    /// ```
    #[inline]
    pub fn sqrtrem(&self) -> (Integer, Integer) {
        if self < &0 {
            panic!("Input is less than zero.");
        } else {
            let mut s = Integer::default();
            let mut r = Integer::default();
            unsafe { 
                flint_sys::fmpz::fmpz_sqrtrem(s.as_mut_ptr(), r.as_mut_ptr(), self.as_ptr());
            }
            (s, r)
        }
    }
   
    /// Return true if `self` is a square.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(9);
    /// assert!(z.is_square());
    /// ```
    #[inline]
    pub fn is_square(&self) -> bool {
        unsafe { flint_sys::fmpz::fmpz_is_square(self.as_ptr()) != 0}
    }

    /*
    // TODO: use Complex?
    /// Return the integer part of the square root of an [Integer]. Returns an [Err] if the input
    /// is negative.
    #[inline]
    pub fn sqrt(&self) -> Result<Integer, ()> {
        if self < &0 {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_sqrt(res.as_mut_ptr(), self.as_ptr());}
            Ok(res)
        }
    }

    /// Return the integer part of the n-th root of an [Integer]. Requires `n > 0` and that if `n`
    /// is even then the input is nonnegative, otherwise an [Err] is returned.
    #[inline]
    pub fn root<S>(&self, n: S) -> Result<Integer, ()> where
        S: TryInto<c_long>,
        S::Error: fmt::Debug
    {
        let n = n.try_into().expect("Input cannot be converted to a long.");
        
        if n < 1 || (Integer::from(n).is_even() && self < &0) {
            Err(())
        } else {
            let mut res = Integer::default();
            unsafe { flint_sys::fmpz::fmpz_root(res.as_mut_ptr(), self.as_ptr(), n);}
            Ok(res)
        }
    }*/
  
    /// If the input is a perfect power then return an `Option` with the root and exponent, 
    /// otherwise `None`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(16);
    /// let (a, b) = z.perfect_power().unwrap();
    /// assert_eq!(a, 2);
    /// assert_eq!(b, 4);
    /// ```
    #[inline]
    pub fn perfect_power(&self) -> Option<(Integer, Integer)> {
        let mut res = Integer::default();
        unsafe { 
            let k = flint_sys::fmpz::fmpz_is_perfect_power(res.as_mut_ptr(), self.as_ptr());

            if k != 0 {
                Some((res, int!(k)))
            } else {
                None
            }
        }
    }
    
   
    /// Return the factorial of `self`. Requires that `self` fits in an unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(4);
    /// assert_eq!(z.factorial(), 24);
    /// ```
    #[inline]
    pub fn factorial(&self) -> Integer {
        assert!(self.abs_fits_ui());
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_fac_ui(res.as_mut_ptr(), self.get_ui().unwrap());}
        res
    }
    
    /// Compute the factorial of `self` and assign it to the input. Requires that `self` fits in an
    /// unsigned long.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(4);
    /// z.factorial_assign();
    /// assert_eq!(z, 24);
    /// ```
    #[inline]
    pub fn factorial_assign(&mut self) {
        assert!((&*self).abs_fits_ui());
        unsafe { flint_sys::fmpz::fmpz_fac_ui(self.as_mut_ptr(), self.get_ui().unwrap());}
    }

    /// Return the rising factorial `x(x+1)(x+2)...(x+k-1)`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.rising_factorial(4), 120);
    /// ```
    #[inline]
    pub fn rising_factorial<S>(&self, k: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let k = k.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_rfac_ui(res.as_mut_ptr(), self.as_ptr(), k);}
        res
    }
    
    /// Compute the rising factorial `x(x+1)(x+2)...(x+k-1)` and assign it to the input.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(2);
    /// z.rising_factorial_assign(4);
    /// assert_eq!(z, 120);
    /// ```
    #[inline]
    pub fn rising_factorial_assign<S>(&mut self, k: S) where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let k = k.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe { flint_sys::fmpz::fmpz_rfac_ui(self.as_mut_ptr(), self.as_ptr(), k);}
    }
    
    /// Return the negative of `self` modulo `modulus`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(4);
    /// assert_eq!(z.negmod(int!(7)), 3);
    /// ```
    #[inline]
    pub fn negmod<T>(&self, modulus: T) -> Integer where
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(!modulus.is_zero());

        if self > modulus {
            let mut res = self.rem(modulus);
            unsafe {
                flint_sys::fmpz::fmpz_negmod(res.as_mut_ptr(), res.as_ptr(), modulus.as_ptr());
            }
            res
        } else {
            let mut res = Integer::default();
            unsafe {
                flint_sys::fmpz::fmpz_negmod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
            }
            res
        }
    }
    
    /// Compute the negative of `self` modulo `modulus` and assign it to the input.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(4);
    /// z.negmod_assign(int!(7));
    /// assert_eq!(z, 3);
    /// ```
    #[inline]
    pub fn negmod_assign<T>(&mut self, modulus: T) where
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(!modulus.is_zero());

        if (&*self) > modulus {
            self.rem_assign(modulus);
        }
        
        unsafe {
            flint_sys::fmpz::fmpz_negmod(self.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        }
    }

    /// Attempt to invert `self` modulo `modulus`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(4);
    /// assert_eq!(z.invmod(int!(7)).unwrap(), 2);
    /// ```
    #[inline]
    pub fn invmod<T>(&self, modulus: T) -> Option<Integer> where
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);

        let mut res = Integer::default();
        unsafe{ 
            let r = flint_sys::fmpz::fmpz_invmod(res.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
        
            if r == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    /// Attempt to invert `self` modulo `modulus` and assign it to the input. Panics if the inverse
    /// does not exist.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(4);
    /// z.invmod_assign(int!(7));
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn invmod_assign<T>(&mut self, modulus: T) where
        T: AsRef<Integer>
    {
        let modulus = modulus.as_ref();
        assert!(modulus > &0);

        unsafe{ 
            let r = flint_sys::fmpz::fmpz_invmod(self.as_mut_ptr(), self.as_ptr(), modulus.as_ptr());
            if r == 0 {
                panic!("Inverse does not exist.");
            }
        }
    }
    

    /// Return the greatest common divisor of two integers. The result is always positive.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(21);
    /// assert_eq!(z.gcd(int!(14)), 7);
    /// ```
    #[inline]
    pub fn gcd<T>(&self, other: T) -> Integer where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe { 
            flint_sys::fmpz::fmpz_gcd(res.as_mut_ptr(), self.as_ptr(), other.as_ref().as_ptr()); 
        }
        res
    }
    
    /// Compute the greatest common divisor of two integers and assign it to the input. The result
    /// is always positive.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(21);
    /// z.gcd_assign(int!(14));
    /// assert_eq!(z, 7);
    /// ```
    #[inline]
    pub fn gcd_assign<T>(&mut self, other: T) where
        T: AsRef<Integer>
    {
        unsafe { 
            flint_sys::fmpz::fmpz_gcd(self.as_mut_ptr(), self.as_ptr(), other.as_ref().as_ptr()); 
        }
    }
    
    /// Return true if two integers are coprime, false otherwise.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(21);
    /// assert!(z.is_coprime(int!(4)));
    /// ```
    #[inline]
    pub fn is_coprime<T>(&self, other: T) -> bool where
        T: AsRef<Integer>
    {
        self.gcd(other.as_ref()) == 1
    }

    /// Return the least common multiple of two integers. The result is always nonnegative.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(6);
    /// assert_eq!(z.lcm(int!(10)), 30);
    /// ```
    #[inline]
    pub fn lcm<T>(&self, other: T) -> Integer where
        T: AsRef<Integer>
    {
        let mut res = Integer::default();
        unsafe { 
            flint_sys::fmpz::fmpz_lcm(res.as_mut_ptr(), self.as_ptr(), other.as_ref().as_ptr()); 
        }
        res
    }
    
    /// Compute the least common multiple of two integers and assign it to the input. The result is
    /// always nonnegative.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(6);
    /// z.lcm_assign(int!(10));
    /// assert_eq!(z, 30);
    /// ```
    #[inline]
    pub fn lcm_assign<T>(&mut self, other: T) where
        T: AsRef<Integer>
    {
        unsafe { 
            flint_sys::fmpz::fmpz_lcm(self.as_mut_ptr(), self.as_ptr(), other.as_ref().as_ptr()); 
        }
    }

    /// Compute the extended GCD of two integers. Call the input integers `f` and `g`. Then we return
    /// `(d, a, b)` where `d = gcd(f, g)` and `a*f + b*g = d`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let f = int!(21);
    /// let (d, a, b) = f.xgcd(int!(14));
    /// assert_eq!(d, 7);
    /// assert_eq!(a, -13);
    /// assert_eq!(b, 20);
    /// ```
    #[inline]
    pub fn xgcd<T>(&self, other: T) -> (Integer, Integer, Integer) where
        T: AsRef<Integer>
    {
        unsafe {
            let mut d = Integer::default();
            let mut a = Integer::default();
            let mut b = Integer::default();
            flint_sys::fmpz::fmpz_xgcd(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ref().as_ptr());
            (d, a, b)
        }
    } 
   
    /*
    /// Compute the extended GCD of two integers. Call the input integers `f` and `g`. Then we return
    /// `(d, a, b)` where `d = gcd(f, g)` and `a*f + b*g = d`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let f = int!(21);
    /// let (d, a, b) = f.xgcd_canonical(int!(14));
    /// assert_eq!(d, 7);
    /// assert_eq!(a, 1);
    /// assert_eq!(b, -1);
    /// ```
    #[inline]
    pub fn xgcd_canonical<T>(&self, other: T) -> (Integer, Integer, Integer) where
        T: AsRef<Integer>
    {
        unsafe {
            let mut d = Integer::default();
            let mut a = Integer::default();
            let mut b = Integer::default();
            flint_sys::fmpz::fmpz_xgcd_canonical_bezout(
                d.as_mut_ptr(), 
                a.as_mut_ptr(), 
                b.as_mut_ptr(),
                self.as_ptr(), 
                other.as_ref().as_ptr());
            (d, a, b)
        }
    } 
    */

    /// Attempt to reconstruct a [Rational] number from it's residue mod `m`. This is just
    /// [rational_reconstruction2][crate::base::integer::src::Integer::rational_reconstruction2] 
    /// with the numerator and denominator bounds `n == d == floor(sqrt((m-1)/2))`. If a solution 
    /// with these constraints exists then it is unique.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let a = int!(2);
    /// let b = int!(3);
    /// let modulus = int!(23);
    ///
    /// // x = a * (b^-1) mod 23 == 16
    /// let x = &a * &(b.invmod(&modulus).unwrap());
    ///
    /// // we recover r = a/b == 2/3 from x == 16 mod 23.
    /// let r = x.rational_reconstruction(&modulus).unwrap();
    ///
    /// assert_eq!(r.numerator(), a);
    /// assert_eq!(r.denominator(), b);
    /// ```
    #[inline]
    pub fn rational_reconstruction<T>(&self, m: T) -> Option<Rational> where
        T: AsRef<Integer>
    {
        let mut res = Rational::default();
        unsafe {
            let b = flint_sys::fmpq::fmpq_reconstruct_fmpz(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                m.as_ref().as_ptr()
            );
            if b == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
    
    /// Given bounds `n` and `d` satisfying `2*n*d < m`, attempt to reconstruct a [Rational] from 
    /// it's residue mod `m` with numerator and denominator absolutely bounded by `n` and `d`
    /// respectively. We also require `gcd(n, d) = 1` and `n = a*d % m`. If a solution exists then
    /// it is unique.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let n = int!(2);
    /// let d = int!(5);
    ///
    /// let a = int!(1);
    /// let b = int!(4);
    /// let modulus = int!(23);
    ///
    /// // x = a * (b^-1) mod 23 == 6
    /// let x = &a * &(b.invmod(&modulus).unwrap());
    ///
    /// // we recover r = a/b == 1/4 from x == 6 mod 23.
    /// let r = x.rational_reconstruction2(&modulus, n, d).unwrap();
    ///
    /// assert_eq!(r.numerator(), a);
    /// assert_eq!(r.denominator(), b);
    /// ```
    #[inline]
    pub fn rational_reconstruction2<M, N, D>(&self, m: M, n: N, d: D) -> Option<Rational> where
        M: AsRef<Integer>,
        N: AsRef<Integer>,
        D: AsRef<Integer>,
    {
        let mut res = Rational::default();
        unsafe {
            let b = flint_sys::fmpq::fmpq_reconstruct_fmpz_2(
                res.as_mut_ptr(), 
                self.as_ptr(), 
                m.as_ref().as_ptr(),
                n.as_ref().as_ptr(), 
                d.as_ref().as_ptr()
            );
            if b == 0 {
                None
            } else {
                Some(res)
            }
        }
    }

    /// Remove all occurrences of the factor `factor` from `self`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(18);
    /// z.remove(int!(3));
    /// assert_eq!(z, 2);
    /// ```
    #[inline]
    pub fn remove<T>(&mut self, factor: T) where
        T: AsRef<Integer>
    {
        let factor = factor.as_ref();
        assert!(factor > &1);
        unsafe {
            flint_sys::fmpz::fmpz_remove(self.as_mut_ptr(), self.as_ptr(), factor.as_ptr());
        }
    }

    /// Compute the jacobi symbol `(a/n)` for any `a` and odd positive `n`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.jacobi(int!(7)), 1);
    /// ```
    #[inline]
    pub fn jacobi<T>(&self, n: T) -> c_int where 
        T: AsRef<Integer>
    {
        let n = n.as_ref();
        assert!(n > &0 && n.is_odd());
        unsafe { flint_sys::fmpz::fmpz_jacobi(self.as_ptr(), n.as_ptr()) }
    }
    
    /// Compute the kronecker symbol `(a/n)` for any `a` and any `n`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(2);
    /// assert_eq!(z.kronecker(int!(-5)), -1);
    /// ```
    #[inline]
    pub fn kronecker<T>(&self, n: T) -> c_int where
        T: AsRef<Integer>
    {
        unsafe { flint_sys::fmpz::fmpz_kronecker(self.as_ptr(), n.as_ref().as_ptr()) }
    }

    // TODO: BIT PACKING
   
    /// Set the i-th bit of `self` to zero.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(1024);
    /// z.clear_bit(10);
    /// assert_eq!(z, 0);
    /// ```
    #[inline]
    pub fn clear_bit<S>(&mut self, i: S)  where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let i = i.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe { flint_sys::fmpz::fmpz_clrbit(self.as_mut_ptr(), i);}
    }
    
    /// Complement the i-th bit of `self`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(256);
    /// z.complement_bit(7);
    /// assert_eq!(z, 384);
    /// ```
    #[inline]
    pub fn complement_bit<S>(&mut self, i: S) where 
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let i = i.try_into().expect("Input cannot be converted to an unsigned long.");
        unsafe { flint_sys::fmpz::fmpz_combit(self.as_mut_ptr(), i);}
    }

    // PRIMALITY TESTING
    // TODO: probable prime tests?

    // a = 4, a = 6. a.is_prime() == true??
    /// Returns true if `self` is a prime.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let a = int!(3);
    /// assert!(a.is_prime());
    ///
    /// let b = int!(4);
    /// assert!(!b.is_prime());
    /// ```
    #[inline]
    pub fn is_prime(&self) -> bool {
        unsafe {
            flint_sys::fmpz::fmpz_is_prime(self.as_ptr()) == 1
        }
    }

    /// Returns the next prime greater than the input.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(7);
    /// assert_eq!(z.next_prime(), 11);
    /// ```
    #[inline]
    pub fn next_prime(&self) -> Integer {
        unsafe {
            let mut res = Integer::default();
            flint_sys::fmpz::fmpz_nextprime(res.as_mut_ptr(), self.as_ptr(), 1);
            res
        }
    }
    
    /// Compute the next prime greater than the input and assign it to the input.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let mut z = int!(7);
    /// z.next_prime_assign();
    /// assert_eq!(z, 11);
    /// ```
    #[inline]
    pub fn next_prime_assign(&mut self) {
        unsafe {
            flint_sys::fmpz::fmpz_nextprime(self.as_mut_ptr(), self.as_ptr(), 1);
        }
    }
    
    /// Outputs the primorial of `n`, the product of all primes less than or equal to `n`.
    ///
    /// ```
    /// use inertia::prelude::*;
    ///
    /// let z = int!(13);
    /// assert_eq!(z.primorial(), 30030);
    /// ```
    #[inline]
    pub fn primorial(&self) -> Integer where 
    {
        let n = self.get_ui().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_primorial(res.as_mut_ptr(), n);}
        res
    }

    /// Returns the value of the Euler totient/phi function at an [Integer] `n`, the number of 
    /// positive integers up to `n` inclusive that are coprime to `n`. The input must be greater
    /// than zero.
    #[inline]
    pub fn euler_phi(&self) -> Integer {
        assert!(self > &0);
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_euler_phi(res.as_mut_ptr(), self.as_ptr());}
        res
    }
    
    /// Compute the Moebius mu function at an [Integer] `n`, which is defined to be 0 if `n` has
    /// a prime factor of multiplicity greater than one, -1 if `n` has an odd number of distinct
    /// prime factors, and 1 otherwise.
    #[inline]
    pub fn moebius_mu(&self) -> c_int {
        unsafe { flint_sys::fmpz::fmpz_moebius_mu(self.as_ptr())}
    }
   
    /// Compute the divisor function `sigma_k(n)` of an [Integer] `n`, which is the sum of `k`-th
    /// powers of the divisors of `n`. If `k = 0` then it counts the number of divisors.
    #[inline]
    pub fn divisor_sigma<S>(&self, k: S) -> Integer where
        S: TryInto<c_ulong>,
        S::Error: fmt::Debug
    {
        let k = k.try_into().expect("Input cannot be converted to an unsigned long.");
        let mut res = Integer::default();
        unsafe { flint_sys::fmpz::fmpz_divisor_sigma(res.as_mut_ptr(), self.as_ptr(), k);}
        res
    }
}

impl Factorizable for Integer {
    type Output = Product<Integer>;
    fn factor(&self) -> Self::Output {
        assert!(self != &0);
        if self == &1 {
            return Product::from(Integer::from(1))
        };
       
        let mut fac = MaybeUninit::uninit();
        unsafe {
            flint_sys::fmpz_factor::fmpz_factor_init(fac.as_mut_ptr());
            let mut fac = fac.assume_init();
            
            flint_sys::fmpz_factor::fmpz_factor(&mut fac, self.as_ptr());

            let n = fac.num as usize;
            let base = std::slice::from_raw_parts(fac.p, n);
            let exp = std::slice::from_raw_parts(fac.exp, n);
            
            let mut hashmap = FxHashMap::<Integer, Integer>::default();
            for (p, k) in base.iter().zip(exp) {
                hashmap.insert(Integer { data: p.clone() }, Integer::from(k));
            }
            
            flint_sys::fmpz_factor::fmpz_factor_clear(&mut fac);
            let fac = Product::<Integer>::from(hashmap);
            fac
        }
    }
}

// Utility functions

/// Return `(a * b) + (c * d)`.
///
/// ```
/// use inertia::prelude::*;
///
/// // args can be a mix of values or borrows
/// assert_eq!(fmma(int!(1), &int!(2), int!(3), int!(4)), 14);
/// ```
#[inline]
pub fn fmma<T1, T2, T3, T4>(a: T1, b: T2, c: T3, d: T4) -> Integer where
    T1: AsRef<Integer>,
    T2: AsRef<Integer>,
    T3: AsRef<Integer>,
    T4: AsRef<Integer>,
{
    let mut res = Integer::default();
    unsafe {
        flint_sys::fmpz::fmpz_fmma(
            res.as_mut_ptr(), 
            a.as_ref().as_ptr(),
            b.as_ref().as_ptr(),
            c.as_ref().as_ptr(),
            d.as_ref().as_ptr()
        );
    }
    res
}

/// Return `(a * b) - (c * d)`.
///
/// ```
/// use inertia::prelude::*;
///
/// // args can be a mix of values or borrows
/// assert_eq!(fmms(&int!(4), int!(3), int!(2), &int!(1)), 10);
/// ```
#[inline]
pub fn fmms<T1, T2, T3, T4>(a: T1, b: T2, c: T3, d: T4) -> Integer where
    T1: AsRef<Integer>,
    T2: AsRef<Integer>,
    T3: AsRef<Integer>,
    T4: AsRef<Integer>,
{
    let mut res = Integer::default();
    unsafe {
        flint_sys::fmpz::fmpz_fmms(
            res.as_mut_ptr(), 
            a.as_ref().as_ptr(),
            b.as_ref().as_ptr(),
            c.as_ref().as_ptr(),
            d.as_ref().as_ptr()
        );
    }
    res
}
   
/// Return the n-th Fibonacci number.
///
/// ```
/// use inertia::prelude::*;
///
/// assert_eq!(fibonacci(6), 8);
/// ```
#[inline]
pub fn fibonacci<S>(n: S) -> Integer where
    S: TryInto<c_ulong>,
    S::Error: fmt::Debug
{
    let n = n.try_into().expect("Input cannot be converted to an unsigned long.");
    let mut res = Integer::default();
    unsafe { flint_sys::fmpz::fmpz_fib_ui(res.as_mut_ptr(), n);}
    res
}
    
/// Return the binomial coefficient n choose k.
///
/// ```
/// use inertia::prelude::*;
///
/// assert_eq!(binomial(5, 3), 10);
/// ```
#[inline]
pub fn binomial<S1, S2>(n: S1, k: S2) -> Integer where
    S1: TryInto<c_ulong>,
    S2: TryInto<c_ulong>,
    S1::Error: fmt::Debug,
    S2::Error: fmt::Debug,
{
    let n = n.try_into().expect("Input cannot be converted to an unsigned long.");
    let k = k.try_into().expect("Input cannot be converted to an unsigned long.");
    let mut res = Integer::default();
    unsafe { flint_sys::fmpz::fmpz_bin_uiui(res.as_mut_ptr(), n, k);}
    res
}
   
/// Use the Chinese Remainder Theorem to return the unique value `0 <= x < M` congruent to `r1`
/// modulo `m1` and `r2` modulo `m2` where `M = m1 * m2`. We require that the moduli are
/// greater than one and coprime and `0 <= r1 < m1`, `0 <= r2 < m2`.
///
/// ```
/// use inertia::prelude::*;
///
/// assert_eq!(crt(int!(2), int!(7), int!(1), int!(5)), 16);
/// ```
#[inline]
pub fn crt<R1, M1, R2, M2>(r1: R1, m1: M1, r2: R2, m2: M2) -> Integer where
    R1: AsRef<Integer>,
    M1: AsRef<Integer>,
    R2: AsRef<Integer>,
    M2: AsRef<Integer>,
{
    let r1 = r1.as_ref();
    let m1 = m1.as_ref();
    let r2 = r2.as_ref();
    let m2 = m2.as_ref();

    assert!(m1 > &1 && m2 > &1);
    assert!(r1 >= &0 && r2 >= &0);
    assert!(m1 > r1 && m2 > r2);
    assert!(m1.is_coprime(m2));

    let mut res = Integer::default();
    unsafe { 
        flint_sys::fmpz::fmpz_CRT(
            res.as_mut_ptr(), 
            r1.as_ptr(), 
            m1.as_ptr(),
            r2.as_ptr(), 
            m2.as_ptr(),
            0
        );
    }
    res
}

/// Use the Chinese Remainder Theorem to compute the unique [Integer] that is congruent to `r[i]`
/// modulo `m[i]` for all `i`. This uses the same assumptions as
/// [crt][crate::integer::src::Integer::crt], also requiring the inputs to have the same length.
#[inline]
pub fn multi_crt<R, M>(r: &[R], m: &[M]) -> Integer where
    R: AsRef<Integer>,
    M: AsRef<Integer>,
{
    assert!(r.len() == m.len());
    let mut res = Integer::default(); 
   
    let len = r.len();
    let vr: Vec<flint_sys::fmpz::fmpz> = r.iter().map(|x| x.as_ref().as_ptr().clone()).collect();
    let vm: Vec<flint_sys::fmpz::fmpz> = m.iter().map(|x| x.as_ref().as_ptr().clone()).collect();

    unsafe { 
        let b = flint_sys::fmpz::fmpz_multi_crt(
            res.as_mut_ptr(), 
            vm.as_ptr(), 
            vr.as_ptr(),
            len as c_long
        );
        assert!(b == 1, "The CRT assumptions were not satisfied and the output is undefined.");
    }
    res
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

impl<T> EvaluateProductMod<T> for Product<Integer> where
    T: AsRef<Integer>
{
    type Output = Result<Integer, ()>;
    fn evaluate_mod(&self, modulus: T) -> Result<Integer, ()> {
        let modulus = modulus.as_ref();
        let mut x = Integer::from(1);
        for (p, k) in self.hashmap.iter() {
            x *= p.powm(k, modulus);
            x %= modulus;
        }
        Ok(x)
    }
}
