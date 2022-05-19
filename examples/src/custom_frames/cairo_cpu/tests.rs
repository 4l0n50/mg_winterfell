// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::super::super::fibonacci::utils::build_proof_options;

#[test]
fn cairo_cpu_test_basic_proof_verification() {
    let cairo_cpu = Box::new(super::CairoCpuExample::new(16, build_proof_options(false)));
    crate::tests::test_basic_proof_verification(cairo_cpu);
}

#[test]
fn cairo_cpu_test_basic_proof_verification_extension() {
    let cairo_cpu = Box::new(super::CairoCpuExample::new(16, build_proof_options(true)));
    crate::tests::test_basic_proof_verification(cairo_cpu);
}

#[test]
fn cairo_cpu_test_basic_proof_verification_fail() {
    let cairo_cpu = Box::new(super::CairoCpuExample::new(16, build_proof_options(false)));
    crate::tests::test_basic_proof_verification_fail(cairo_cpu);
}
