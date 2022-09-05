// Copyright 2022 Joshua Wong.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use ghostcell::GhostToken;
use ghostcell_macro::ghost;

#[ghost(mut token)]
fn ghost_test() {
    let token = &mut token;
    let gcell = token.cell(3);
    *gcell.rw(token) += 1;
}

fn main() {}
