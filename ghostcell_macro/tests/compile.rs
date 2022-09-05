// Copyright 2022 Joshua Wong.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/*.rs");
}
