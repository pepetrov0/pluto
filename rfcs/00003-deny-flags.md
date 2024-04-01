# Deny flags

- **Author(s):** <ppetrov@pbyte.xyz>
- **Approver(s):** <ppetrov@pbyte.xyz>
- **Status:** Implemented
- **Created:** 2024-03-31
- **Last updated:** 2024-03-31

## Overview

This RFC proposes the addition of global `#![deny(missing_docs)]` and `#![deny(unused)]` directives to **pluto**'s compilation flags, aiming to bolster code quality and documentation standards.

## Goals and Non-Goals

### Goals:

- Ensure comprehensive documentation for all public interfaces.
- Improve code quality by minimizing unused variables, methods, structs, enums, etc.

### Non-Goals:

- Introduce significant changes to existing code structure or functionality.

## Background & Motivation

At present, **pluto** allows interfaces to lack documentation and permits the accumulation of unused variables, methods, and other elements within the codebase. This lax approach can lead to reduced code clarity and maintainability.

Therefore, this proposal seeks to enforce fundamental coding standards by leveraging existing compiler flags.

## Design

To enforce optimized code and thorough documentation, the proposed solution involves adding `#![deny(unused)]` and `#![deny(missing_docs)]` directives to the `lib.rs` file.

## Dependencies

No identified dependencies are required for this implementation.

## Revisions:

1. *2024-03-31* - RFC Created
