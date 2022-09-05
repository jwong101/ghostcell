// Copyright 2022 Joshua Wong.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use ghostcell::GhostToken;
use ghostcell_macro::ghost;

#[ghost(mut tok1, mut tok2, mut tok3)]
fn multi() {
    let tok1 = &mut tok1;
    let tok2 = &mut tok2;
    let tok3 = &mut tok3;
    let gcell1 = tok1.cell(3);
    *gcell1.rw(tok1) += 1;
    let gcell2 = tok2.cell(3);
    *gcell2.rw(tok2) += 1;
    let gcell3 = tok3.cell(3);
    *gcell3.rw(tok3) += 1;
    *gcell1.rw(tok1) += 4;
}

fn main() {}
