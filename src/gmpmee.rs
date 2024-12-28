// Copyright © 2024 Denis Morel

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU Lesser General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU Lesser General Public License and
// a copy of the GNU General Public License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

#![allow(non_camel_case_types, dead_code)]
use gmp_mpfr_sys::gmp::{mpz_ptr, mpz_srcptr, mpz_t, randstate_ptr, size_t};

#[doc = " Stores the tables of precomputed products of subsets of the\n bases. Each table contains the precomputed products for a range of\n a given width of the bases."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gmpmee_spowm_tab {
    #[doc = "< Total number of bases/exponents."]
    pub len: size_t,
    #[doc = "< Number of bases/exponents in each block."]
    pub block_width: size_t,
    #[doc = "< Number of blocks."]
    pub tabs_len: size_t,
    #[doc = "< Table of tables, one sub-table for each block."]
    pub tabs: *mut *mut mpz_t,
    #[doc = "< Modulus used in computations."]
    pub modulus: mpz_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gmpmee_spowm_tab"][::std::mem::size_of::<gmpmee_spowm_tab>() - 48usize];
    ["Alignment of gmpmee_spowm_tab"][::std::mem::align_of::<gmpmee_spowm_tab>() - 8usize];
    ["Offset of field: gmpmee_spowm_tab::len"]
        [::std::mem::offset_of!(gmpmee_spowm_tab, len) - 0usize];
    ["Offset of field: gmpmee_spowm_tab::block_width"]
        [::std::mem::offset_of!(gmpmee_spowm_tab, block_width) - 8usize];
    ["Offset of field: gmpmee_spowm_tab::tabs_len"]
        [::std::mem::offset_of!(gmpmee_spowm_tab, tabs_len) - 16usize];
    ["Offset of field: gmpmee_spowm_tab::tabs"]
        [::std::mem::offset_of!(gmpmee_spowm_tab, tabs) - 24usize];
    ["Offset of field: gmpmee_spowm_tab::modulus"]
        [::std::mem::offset_of!(gmpmee_spowm_tab, modulus) - 32usize];
};

#[link(name = "gmpmee", kind = "static")]
unsafe extern "C" {
    #[doc = " Allocates and initializes a table for the given modulus, block\n width, and total number of bases.\n\n @param table Table to be initialized\n @param len Number of bases in the simultaneous exponentiation.\n @param modulus Modulus.\n @param block_width Number of bases used to build each subtable."]
    fn gmpmee_spowm_init(
        table: *mut gmpmee_spowm_tab,
        len: usize,
        modulus: mpz_ptr,
        block_width: usize,
    );

    #[doc = " Frees the memory allocated by table.\n\n @param table Table to be deallocated."]
    fn gmpmee_spowm_clear(table: *mut gmpmee_spowm_tab);

    #[doc = " Fills the table with precomputed values using the given bases. The\n array of bases must be of the length for which the table was\n allocated.\n\n @param table Table to be initialized.\n @param bases Bases for which precomputation is performed."]
    fn gmpmee_spowm_precomp(table: *mut gmpmee_spowm_tab, bases: mpz_srcptr);

    #[doc = " Computes a simultaneous exponentiation using the given table and\n exponents. The number of exponents must match the number of bases\n that was used during precomputation.\n\n @param rop Destination of result.\n @param table Precomputed table representing the bases used.\n @param exponents Exponents used in simultaneous exponentiation."]
    fn gmpmee_spowm_table(rop: mpz_ptr, table: *mut gmpmee_spowm_tab, exponents: mpz_srcptr);

    #[doc = " Computes a simultaneous exponentiation. Precomputation is performed\n in blocks of the given width in batches of the given batch size.\n\n @param rop Destination of result.\n @param bases Bases for which precomputation is performed.\n @param exponents Exponents used in simultaneous exponentiation.\n @param len Number of bases in the simultaneous exponentiation.\n @param modulus Modulus.\n @param block_width Number of bases used to build each subtable.\n @param batch_len Number of bases in each batch, where each batch\n is computed independently."]
    fn gmpmee_spowm_block_batch(
        rop: mpz_ptr,
        bases: mpz_srcptr,
        exponents: mpz_srcptr,
        len: size_t,
        modulus: mpz_ptr,
        block_width: size_t,
        batch_len: size_t,
    );

    #[doc = " Computes a simultaneous exponentiation. Precomputation is performed\n in blocks of a reasonable width in a single batch.\n\n @param rop Destination of result.\n @param bases Bases for which precomputation is performed.\n @param exponents Exponents used in simultaneous exponentiation.\n @param len Number of bases in the simultaneous exponentiation.\n @param modulus Modulus."]
    pub fn gmpmee_spowm(
        rop: mpz_ptr,
        bases: mpz_srcptr,
        exponents: mpz_srcptr,
        len: size_t,
        modulus: mpz_srcptr,
    );
}

#[doc = " Stores a fixed base exponentiation table."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gmpmee_fpowm_tab {
    #[doc = "< We exploit simultaneous exp. table."]
    pub spowm_table: gmpmee_spowm_tab,
    #[doc = "< Normal number of bits of each\n\"subexponent\"."]
    pub stretch: size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gmpmee_fpowm_tab"][::std::mem::size_of::<gmpmee_fpowm_tab>() - 56usize];
    ["Alignment of gmpmee_fpowm_tab"][::std::mem::align_of::<gmpmee_fpowm_tab>() - 8usize];
    ["Offset of field: gmpmee_fpowm_tab::spowm_table"]
        [::std::mem::offset_of!(gmpmee_fpowm_tab, spowm_table) - 0usize];
    ["Offset of field: gmpmee_fpowm_tab::stretch"]
        [::std::mem::offset_of!(gmpmee_fpowm_tab, stretch) - 48usize];
};

#[link(name = "gmpmee", kind = "static")]
unsafe extern "C" {
    #[doc = " Allocates and initializes a table with the given modulus, block\n width, and expected exponent bit length.\n\n @param table Table to be initialized\n @param modulus Modulus.\n @param block_width Number of bases used to build each subtable.\n @param exponent_bitlen Expected bit length of exponent."]
    pub fn gmpmee_fpowm_init(
        table: *mut gmpmee_fpowm_tab,
        modulus: mpz_srcptr,
        block_width: size_t,
        exponent_bitlen: size_t,
    );

    #[doc = " Frees the memory allocated by table.\n\n @param table Table to be deallocated."]
    pub fn gmpmee_fpowm_clear(table: *mut gmpmee_fpowm_tab);

    #[doc = " Fills the table with precomputed values using the given basis.\n\n @param table Table to be initialized.\n @param basis Basis for which precomputation is performed."]
    pub fn gmpmee_fpowm_precomp(table: *mut gmpmee_fpowm_tab, basis: mpz_srcptr);

    #[doc = " Equivalent to calling gmpmee_fpowm_init and then gmpmee_fpowm_precomp.\n\n @param table Table to be initialized\n @param basis Basis for which precomputation is performed.\n @param modulus Modulus.\n @param block_width Number of bases used to build each subtable.\n @param exponent_bitlen Expected bit length of exponent."]
    pub fn gmpmee_fpowm_init_precomp(
        table: *mut gmpmee_fpowm_tab,
        basis: mpz_srcptr,
        modulus: mpz_srcptr,
        block_width: size_t,
        exponent_bitlen: size_t,
    );

    #[doc = " Computes a fixed base exponentiation using the given table and\n exponent.\n\n @param rop Destination of result.\n @param table Precomputed table representing the basis used.\n @param exponent Exponent."]
    pub fn gmpmee_fpowm(rop: mpz_ptr, table: *const gmpmee_fpowm_tab, exponent: mpz_srcptr);
}

#[doc = " Stores state inbetween individual invokations of the Miller-Rabin\n test and keeps allocated space."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gmpmee_millerrabin_state {
    #[doc = "< integer to be tested"]
    pub n: mpz_t,
    #[doc = "< n minus one"]
    pub n_minus_1: mpz_t,
    #[doc = "< q is defined by n=q*2^k+1"]
    pub q: mpz_t,
    #[doc = "< k is defined by n=q*2^k+1"]
    pub k: ::std::ffi::c_ulong,
    #[doc = "< y is temporary space"]
    pub y: mpz_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gmpmee_millerrabin_state"]
        [::std::mem::size_of::<gmpmee_millerrabin_state>() - 72usize];
    ["Alignment of gmpmee_millerrabin_state"]
        [::std::mem::align_of::<gmpmee_millerrabin_state>() - 8usize];
    ["Offset of field: gmpmee_millerrabin_state::n"]
        [::std::mem::offset_of!(gmpmee_millerrabin_state, n) - 0usize];
    ["Offset of field: gmpmee_millerrabin_state::n_minus_1"]
        [::std::mem::offset_of!(gmpmee_millerrabin_state, n_minus_1) - 16usize];
    ["Offset of field: gmpmee_millerrabin_state::q"]
        [::std::mem::offset_of!(gmpmee_millerrabin_state, q) - 32usize];
    ["Offset of field: gmpmee_millerrabin_state::k"]
        [::std::mem::offset_of!(gmpmee_millerrabin_state, k) - 48usize];
    ["Offset of field: gmpmee_millerrabin_state::y"]
        [::std::mem::offset_of!(gmpmee_millerrabin_state, y) - 56usize];
};

#[link(name = "gmpmee", kind = "static")]
unsafe extern "C" {
    #[doc = " Allocate and initialize Miller-Rabin state using the given integer.\n\n @param state State for testing.\n @param n Integer to test."]
    pub fn gmpmee_millerrabin_init(state: *mut gmpmee_millerrabin_state, n: mpz_ptr);

    #[doc = " Updates the state to correspond to the next larger candidate\n integer that passes the trial divisions.\n\n @param state State for testing primality."]
    pub fn gmpmee_millerrabin_next_cand(state: *mut gmpmee_millerrabin_state);

    #[doc = " Free memory resources allocated for testing.\n\n @param state State for testing."]
    pub fn gmpmee_millerrabin_clear(state: *mut gmpmee_millerrabin_state);

    #[doc = " Performs trial divisions and returns 0 or 1 depending on if a small\n factor of the integer has been found or not. Assumes that the input\n is greater than three.\n\n @param n Integer to test."]
    pub fn gmpmee_millerrabin_trial(n: mpz_ptr) -> ::std::ffi::c_int;

    #[doc = " Executes one round of the Miller-Rabin test and returns 0 or 1\n depending on if the tested integer is deemed to be composite or\n not.\n\n @param state State for testing.\n\n @param base Base element used for testing. This must be an integer\n in [2,n-2]."]
    pub fn gmpmee_millerrabin_once(
        state: *mut gmpmee_millerrabin_state,
        base: mpz_ptr,
    ) -> ::std::ffi::c_int;

    #[doc = " Executes the Miller-Rabin test using randomness from one of GMP's\n random sources. Assumes that the tested integer is greater than\n three.\n\n @param rstate Source of randomness.\n @param state State for testing.\n @param reps Number of repetitions."]
    pub fn gmpmee_millerrabin_reps_rs(
        rstate: randstate_ptr,
        state: *mut gmpmee_millerrabin_state,
        reps: ::std::ffi::c_int,
    ) -> ::std::ffi::c_int;

    #[doc = " Executes a number or repetitions of the Miller-Rabin test using\n basis derived from the given GMP random source and returns 0 or 1\n depending on if the tested integer is deemed to be composite or\n not.\n\n <p>\n\n WARNING! GMP's random number generators are NOT cryptographically\n secure.\n\n <p>\n\n @param rstate State of random number generator.\n @param n Integer to test.\n @param reps Repetitions of the Miller-Rabin test performed."]
    pub fn gmpmee_millerrabin_rs(
        rstate: randstate_ptr,
        n: mpz_srcptr,
        reps: ::std::ffi::c_int,
    ) -> ::std::ffi::c_int;

    #[doc = " Searches for the smallest prime larger than the given\n integer. Primality testing is done using the Miller-Rabin test\n using randomness from one of GMP's random sources.\n\n @param rop Result destination.\n @param rstate Source of randomness.\n @param n Starting point in search.\n @param reps Number of repetitions."]
    pub fn gmpmee_millerrabin_next_rs(
        rop: mpz_ptr,
        rstate: randstate_ptr,
        n: mpz_ptr,
        reps: ::std::ffi::c_int,
    );
}

#[doc = " Stores the states needed for using the Miller-Rabin test for\n testing for safe-primality."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gmpmee_millerrabin_safe_state {
    #[doc = " State of the integer <i>n</i> to be tested."]
    pub nstate: gmpmee_millerrabin_state,
    #[doc = " State of the integer <i>(n-1)/2</i> to be tested."]
    pub mstate: gmpmee_millerrabin_state,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gmpmee_millerrabin_safe_state"]
        [::std::mem::size_of::<gmpmee_millerrabin_safe_state>() - 144usize];
    ["Alignment of gmpmee_millerrabin_safe_state"]
        [::std::mem::align_of::<gmpmee_millerrabin_safe_state>() - 8usize];
    ["Offset of field: gmpmee_millerrabin_safe_state::nstate"]
        [::std::mem::offset_of!(gmpmee_millerrabin_safe_state, nstate) - 0usize];
    ["Offset of field: gmpmee_millerrabin_safe_state::mstate"]
        [::std::mem::offset_of!(gmpmee_millerrabin_safe_state, mstate) - 72usize];
};

#[link(name = "gmpmee", kind = "static")]
unsafe extern "C" {
    #[doc = " Initialize Miller-Rabin state to be used for safe-primality\n testing using the given integer.\n\n @param state State for testing safe-primality.\n @param n Integer to test."]
    pub fn gmpmee_millerrabin_safe_init(state: *mut gmpmee_millerrabin_safe_state, n: mpz_ptr);

    #[doc = " Sets the state to the next candidate integer larger than the most\n recently tested candidate that passes the trial divisions.\n\n @param state State for testing safe-primality."]
    pub fn gmpmee_millerrabin_safe_next_cand(state: *mut gmpmee_millerrabin_safe_state);

    #[doc = " Free memory allocated in the states.\n\n @param state State for testing safe-primality."]
    pub fn gmpmee_millerrabin_safe_clear(state: *mut gmpmee_millerrabin_safe_state);

    #[doc = " Performs trial divisions and returns 0 or 1 depending on if the\n integer is definitely a not a safe prime, or if it could\n potentially be a safe prime. Assumes that n is at least 8.\n\n @param n Integer to test."]
    pub fn gmpmee_millerrabin_safe_trial(n: mpz_ptr) -> ::std::ffi::c_int;

    #[doc = " Executes one round of the Miller-Rabin test and returns 0 or 1\n depending on if the tested integer is deemed to not be a safe\n prime, or a safe prime. Assumes that the tested integer is at least\n 8.\n\n @param state State for testing safe-primality.\n @param nbase Base element used for testing safe-primality. This\n must be an integer in [2,n-1], where n is the integer to be tested.\n @param mbase Base element used for testing safe-primality.  This\n must be an integer in [2,m-1], where n=2m+1."]
    pub fn gmpmee_millerrabin_safe_once(
        state: *mut gmpmee_millerrabin_safe_state,
        nbase: mpz_ptr,
        mbase: mpz_ptr,
    ) -> ::std::ffi::c_int;

    #[doc = " Executes a safe primality test for the integer used to initialize\n the given testing state, using randomness from the given GMP's\n random source.\n\n @param rstate Source of randomness.\n @param state State for testing safe-primality.\n @param reps Number of repetitions."]
    pub fn gmpmee_millerrabin_safe_reps_rs(
        rstate: randstate_ptr,
        state: *mut gmpmee_millerrabin_safe_state,
        reps: ::std::ffi::c_int,
    ) -> ::std::ffi::c_int;

    #[doc = " Executes several repetitions of the of the Miller-Rabin test and\n returns 0 or 1 depending on if the tested integer is deemed to not\n be a safe prime, or a safe prime. The basis elements are derived\n from the given random number generator.\n\n <p>\n\n WARNING! GMP's random number generators are NOT cryptographically\n secure.\n\n <p>\n\n @param rstate State of random number generator.\n @param n Integer to test.\n @param reps Repetitions of the Miller-Rabin test performed."]
    pub fn gmpmee_millerrabin_safe_rs(
        rstate: randstate_ptr,
        n: mpz_srcptr,
        reps: ::std::ffi::c_int,
    ) -> ::std::ffi::c_int;

    #[doc = " Uses gmpmee_millerrabin_safe_rs to find the smallest safe prime\n larger than the input integer.\n\n <p>\n\n WARNING! GMP's random number generators are NOT cryptographically\n secure.\n\n <p>\n\n @param rop Found safe prime.\n @param rstate State of random number generator.\n @param n Integer to test.\n @param reps Repetitions of the Miller-Rabin test performed."]
    pub fn gmpmee_millerrabin_safe_next_rs(
        rop: mpz_ptr,
        rstate: randstate_ptr,
        n: mpz_ptr,
        reps: ::std::ffi::c_int,
    );

    #[doc = " Naive implementation of a search for the next prime test.  Based on\n GMP's probab_prime_p. Used for debugging.\n\n @param rop Next prime greater than the input integer.\n @param n Starting point of search.\n @param reps Number of repetitions of the Miller-Rabin test."]
    pub fn mpz_probab_prime_p_next(rop: mpz_ptr, n: mpz_ptr, reps: ::std::ffi::c_int);

    #[doc = " Naive implementation of a safe-primality test based on GMP's\n probab_prime_p. Used for debugging.\n\n @param n Integer to be tested.\n @param reps Number of repetitions of the Miller-Rabin test."]
    pub fn mpz_probab_safe_prime_p(n: mpz_ptr, reps: ::std::ffi::c_int) -> ::std::ffi::c_int;

    #[doc = " Naive implementation of a search for the next safe prime test.\n Based on probab_safe_prime_p. Used for debugging.\n\n @param rop Next safe prime greater than the input integer.\n @param n Starting point of search.\n @param reps Number of repetitions of the Miller-Rabin test."]
    pub fn mpz_probab_safe_prime_p_next(rop: mpz_ptr, n: mpz_ptr, reps: ::std::ffi::c_int);
}

/* Not used as internal functions of GMPMEE
unsafe extern "C" {
    #[doc = " Allocates an array of <code>len</code> <code>mpz_ptr</code>.\n\n @param len Number of elements in array.\n @return Pointer to allocated array."]
    pub fn gmpmee_array_alloc(len: usize) -> *mut mpz_ptr;
}
unsafe extern "C" {
    #[doc = " Allocates and initializes an array of <code>len</code>\n <code>mpz_ptr</code>.\n\n @param len Number of elements in array.\n @return Pointer to allocated array."]
    pub fn gmpmee_array_alloc_init(len: usize) -> *mut mpz_ptr;
}
unsafe extern "C" {
    #[doc = " Clears and deallocates the array containing <code>len</code>\n <code>mpz_ptr</code>.\n\n @param a Array to be cleared and deallocated.\n @param len Number of elements in array."]
    pub fn gmpmee_array_clear_dealloc(a: *mut mpz_ptr, len: usize);
}
unsafe extern "C" {
    #[doc = " Fills the array rop containing <code>len</code> <code>mpz_ptr</code>\n with random positive <code>n</code>-bit integers. <b>WARNING! The\n pseudo-random generator of GMP used as a subroutine is *not*\n cryptographically secure.</b>\n\n @param rop Destination of result.\n @param len Number of elements in array.\n @param state State of pseudo-random generator.\n @param n Number of bits in each random integer."]
    pub fn gmpmee_array_urandomb(
        rop: *mut mpz_ptr,
        len: usize,
        state: *mut __gmp_randstate_struct,
        n: ::std::ffi::c_ulong,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::ffi::c_uint,
    pub fp_offset: ::std::ffi::c_uint,
    pub overflow_arg_area: *mut ::std::ffi::c_void,
    pub reg_save_area: *mut ::std::ffi::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
    ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
    ["Offset of field: __va_list_tag::gp_offset"]
        [::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
    ["Offset of field: __va_list_tag::fp_offset"]
        [::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
    ["Offset of field: __va_list_tag::overflow_arg_area"]
        [::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
    ["Offset of field: __va_list_tag::reg_save_area"]
        [::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
};
  */
