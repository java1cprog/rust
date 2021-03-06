// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// This file tests repr(transparent)-related errors reported during typeck. Other errors
// that are reported earlier and therefore preempt these are tested in:
// - repr-transparent-other-reprs.rs
// - repr-transparent-other-items.rs

#![feature(repr_align, attr_literals)]

use std::marker::PhantomData;

#[repr(transparent)]
struct NoFields; //~ ERROR needs exactly one non-zero-sized field

#[repr(transparent)]
struct ContainsOnlyZst(()); //~ ERROR needs exactly one non-zero-sized field

#[repr(transparent)]
struct ContainsOnlyZstArray([bool; 0]); //~ ERROR needs exactly one non-zero-sized field

#[repr(transparent)]
struct ContainsMultipleZst(PhantomData<*const i32>, NoFields);
//~^ ERROR needs exactly one non-zero-sized field

#[repr(transparent)]
struct MultipleNonZst(u8, u8); //~ ERROR needs exactly one non-zero-sized field

trait Mirror { type It: ?Sized; }
impl<T: ?Sized> Mirror for T { type It = Self; }

#[repr(transparent)]
pub struct StructWithProjection(f32, <f32 as Mirror>::It);
//~^ ERROR needs exactly one non-zero-sized field

#[repr(transparent)]
struct NontrivialAlignZst(u32, [u16; 0]); //~ ERROR alignment larger than 1

#[repr(align(32))]
struct ZstAlign32<T>(PhantomData<T>);

#[repr(transparent)]
struct GenericAlign<T>(ZstAlign32<T>, u32); //~ ERROR alignment larger than 1
