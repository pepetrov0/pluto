# Forbid flags

- **Author(s):** <ppetrov@pbyte.xyz>
- **Approver(s):** <ppetrov@pbyte.xyz>
- **Status:** Implemented
- **Created:** 2024-03-31
- **Last updated:** 2024-03-31
## Overview

This request for comment is about adding a global `#![forbid(missing_docs)]` and `#![forbid(unused)]` to **pluto**'s compilation flags.

## Goals and Non-Goals:

The goal of this change is to ensure that documentation is present on all public interfaces and there is a basic level of code quality when it comes to unused variables/methods/structs/enums/etc.

## Background & Motivation:

Currently, we allow interfaces to NOT have any documentation and also code can become littered with unused variables/methods/etc.

Thus the motivation of this proposal is to enforce basic requirements through already available compiler flags.

## Design:

To ensure the code is optimized and well-documented, we can simply add `#![forbid(unused)]` and `#![forbid(missing_docs)]` to the `lib.rs` file.

## Dependencies:

No dependencies have been identified yet.

## Revisions:

1. *2024-03-31* - RFC Created
