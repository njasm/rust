// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    |_: [_; continue]| {}; //~ ERROR: `continue` outside of loop

    while |_: [_; continue]| {} {} //~ ERROR: `break` or `continue` with no label

    while |_: [_; break]| {} {} //~ ERROR: `break` or `continue` with no label
}