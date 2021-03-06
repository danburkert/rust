// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(unboxed_closures)]

pub trait Handler {
    fn handle(&self, &mut String);
}

impl<F> Handler for F
where F: for<'a, 'b> Fn(&'a mut String) {
    fn handle(&self, st: &mut String) {
        self.call((st,))
    }
}

fn main() {}
