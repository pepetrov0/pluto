# Async runtime

- **Author(s):** <ppetrov@pbyte.xyz> 
- **Approver(s):** <ppetrov@pbyte.xyz>
- **Status:** Implemented
- **Created:** 2024-03-31
- **Last updated:** 2024-03-31

## Overview

The goal of this RFC is to propose the adoption of Tokio as the Rust async runtime for our new project. Tokio is a mature, battle-tested async runtime for Rust, providing excellent performance and scalability for asynchronous applications. By adopting Tokio, we aim to leverage its robust ecosystem and community support to build a reliable and efficient asynchronous system.

## Goals and Non-Goals:

### Goals:

- Improve performance and scalability of our asynchronous system.
- Utilize a well-supported and widely adopted async runtime.
- Benefit from Tokio's ecosystem for async I/O, task scheduling, and concurrency primitives.
- Ensure compatibility and interoperability with existing and future Rust async libraries and frameworks.

### Non-Goals:

- Rewrite existing synchronous code unless necessary for integration with Tokio.
- Address unrelated architectural or design concerns not directly related to async runtime selection.

## Background & Motivation:

The current state of the world sees a significant shift towards asynchronous programming paradigms, especially in networked applications where high concurrency and responsiveness are crucial. Rust, with its strong safety guarantees and performance characteristics, has become increasingly popular for building such systems. However, achieving optimal performance and scalability in asynchronous Rust applications requires a reliable async runtime.

Tokio has emerged as one of the leading async runtimes for Rust, offering a comprehensive set of features tailored for high-performance async I/O, task scheduling, and concurrency management. Its architecture is designed to efficiently handle thousands of concurrent connections while ensuring low latency and minimal overhead. By adopting Tokio, we align with industry best practices and tap into a vibrant ecosystem of async libraries and tools, accelerating our development process and enhancing the maintainability of our codebase.

## Design:

The design entails integrating Tokio into our project as the primary async runtime. This involves:

- Incorporating Tokio's core runtime loop into our application structure.
- Refactoring existing synchronous code to utilize Tokio's asynchronous APIs where applicable.
- Leveraging Tokio's executor for scheduling asynchronous tasks and managing concurrency.
- Adapting our application architecture to fit within the Tokio framework, including handling async I/O operations, managing task lifecycles, and orchestrating asynchronous workflows.

## Dependencies:

The adoption of Tokio introduces a dependency on its core runtime and associated ecosystem of libraries. Additionally, it may require updates to existing dependencies to ensure compatibility with Tokio's APIs and conventions. However, Tokio's modular design allows for selective usage of its components, minimizing potential conflicts with other dependencies.

## Alternatives Considered/Prior Art:

### Alternatives Considered:

- **Async-std:** Another popular async runtime for Rust, offering similar features and performance characteristics as Tokio. However, Tokio's broader ecosystem and maturity make it a preferable choice for our project's requirements.
- **Custom Implementation:** Building a custom async runtime tailored to our specific needs. While providing flexibility and control, this approach requires significant development effort and ongoing maintenance, which may outweigh the benefits compared to adopting an existing solution like Tokio.

### Prior Art:

Several projects have successfully adopted Tokio as their async runtime, demonstrating its effectiveness in real-world scenarios. Examples include web servers, networked applications, and distributed systems leveraging Tokio's asynchronous capabilities for improved performance and scalability.

## Risks:

### Known Risks:

- **Learning Curve:** Adopting Tokio may require developers to familiarize themselves with its APIs and concurrency model, potentially leading to a learning curve.
- **Performance Overhead:** While Tokio is known for its performance, improper usage or misconfiguration may introduce performance overhead or latency issues.

### Factors to Address:

- Compatibility with other Rust async libraries and frameworks used in conjunction with Tokio.

## Revisions:

1. _2024-03-31_ - RFC Created
