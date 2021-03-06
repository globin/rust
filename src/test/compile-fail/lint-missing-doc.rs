// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// When denying at the crate level, be sure to not get random warnings from the
// injected intrinsics by the compiler.
#![feature(struct_variant)]
#![feature(globs)]
#![deny(missing_doc)]
#![allow(dead_code)]

//! Some garbage docs for the crate here
#![doc="More garbage"]

struct Foo {
    a: int,
    b: int,
}

pub struct PubFoo { //~ ERROR: missing documentation
    pub a: int,      //~ ERROR: missing documentation
    b: int,
}

#[allow(missing_doc)]
pub struct PubFoo2 {
    pub a: int,
    pub c: int,
}

mod module_no_dox {}
pub mod pub_module_no_dox {} //~ ERROR: missing documentation

/// dox
pub fn foo() {}
pub fn foo2() {} //~ ERROR: missing documentation
fn foo3() {}
#[allow(missing_doc)] pub fn foo4() {}

/// dox
pub trait A {
    /// dox
    fn foo();
    /// dox
    fn foo_with_impl() {}
}
#[allow(missing_doc)]
trait B {
    fn foo();
    fn foo_with_impl() {}
}
pub trait C { //~ ERROR: missing documentation
    fn foo(); //~ ERROR: missing documentation
    fn foo_with_impl() {} //~ ERROR: missing documentation
}
#[allow(missing_doc)] pub trait D {}

impl Foo {
    pub fn foo() {}
    fn bar() {}
}

impl PubFoo {
    pub fn foo() {} //~ ERROR: missing documentation
    /// dox
    pub fn foo1() {}
    fn foo2() {}
    #[allow(missing_doc)] pub fn foo3() {}
}

#[allow(missing_doc)]
trait F {
    fn a();
    fn b(&self);
}

// should need to redefine documentation for implementations of traits
impl F for Foo {
    fn a() {}
    fn b(&self) {}
}

// It sure is nice if doc(hidden) implies allow(missing_doc), and that it
// applies recursively
#[doc(hidden)]
mod a {
    pub fn baz() {}
    pub mod b {
        pub fn baz() {}
    }
}

enum Baz {
    BazA {
        a: int,
        b: int
    },
    BarB
}

pub enum PubBaz { //~ ERROR: missing documentation
    PubBazA { //~ ERROR: missing documentation
        pub a: int, //~ ERROR: missing documentation
        b: int
    },
}

/// dox
pub enum PubBaz2 {
    /// dox
    PubBaz2A {
        /// dox
        pub a: int,
        b: int
    },
}

#[allow(missing_doc)]
pub enum PubBaz3 {
    PubBaz3A {
        pub a: int,
        b: int
    },
}

#[doc(hidden)]
pub fn baz() {}

mod internal_impl {
    /// dox
    pub fn documented() {}
    pub fn undocumented1() {} //~ ERROR: missing documentation
    pub fn undocumented2() {} //~ ERROR: missing documentation
    fn undocumented3() {}
    /// dox
    pub mod globbed {
        /// dox
        pub fn also_documented() {}
        pub fn also_undocumented1() {} //~ ERROR: missing documentation
        fn also_undocumented2() {}
    }
}
/// dox
pub mod public_interface {
    pub use internal_impl::documented as foo;
    pub use internal_impl::undocumented1 as bar;
    pub use internal_impl::{documented, undocumented2};
    pub use internal_impl::globbed::*;
}

fn main() {}
