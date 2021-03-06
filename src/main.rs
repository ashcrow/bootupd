/*
 * Copyright (C) 2020 Red Hat, Inc.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

//! See: https://github.com/coreos/fedora-coreos-tracker/issues/510
//! This is an early prototype hidden/not-yet-standardized mechanism
//! which just updates EFI for now (x86_64/aarch64 only).
//!
//! But in the future will hopefully gain some independence from
//! ostree and also support e.g. updating the MBR etc.

// To run the unit tests for this code, use `make check TESTS=tests/check/test-ex-boot-update.sh`

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    bootupd::boot_update_main(&args)
}
