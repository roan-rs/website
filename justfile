#!/usr/bin/env -S just --justfile

set windows-shell := ["powershell"]
set shell := ["bash", "-cu"]

_default:
    @just --list -u

init:
    cargo install cargo-shear

ready:
    just fmt
    just check
    just fix
    git status

check:
    cargo check --workspace --all-features --all-targets --locked

fmt:
    cargo shear --fix
    cargo fmt --all

fix:
    cargo fix --allow-dirty
    just fmt
