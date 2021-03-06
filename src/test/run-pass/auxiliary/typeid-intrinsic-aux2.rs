// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(core)]

use std::any::{Any, TypeId};

pub struct A;
pub struct B(Option<A>);
pub struct C(Option<isize>);
pub struct D(Option<&'static str>);
pub struct E(Result<&'static str, isize>);

pub type F = Option<isize>;
pub type G = usize;
pub type H = &'static str;
pub type I = Box<Fn()>;
pub type I32Iterator = Iterator<Item=i32>;
pub type U32Iterator = Iterator<Item=u32>;

pub fn id_A() -> TypeId { TypeId::of::<A>() }
pub fn id_B() -> TypeId { TypeId::of::<B>() }
pub fn id_C() -> TypeId { TypeId::of::<C>() }
pub fn id_D() -> TypeId { TypeId::of::<D>() }
pub fn id_E() -> TypeId { TypeId::of::<E>() }
pub fn id_F() -> TypeId { TypeId::of::<F>() }
pub fn id_G() -> TypeId { TypeId::of::<G>() }
pub fn id_H() -> TypeId { TypeId::of::<H>() }
pub fn id_I() -> TypeId { TypeId::of::<I>() }

pub fn foo<T: Any>() -> TypeId { TypeId::of::<T>() }

pub fn id_i32_iterator() -> TypeId { TypeId::of::<I32Iterator>() }
pub fn id_u32_iterator() -> TypeId { TypeId::of::<U32Iterator>() }
