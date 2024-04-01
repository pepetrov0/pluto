# Database

- **Author(s):** <ppetrov@pbyte.xyz>
- **Approver(s):** <ppetrov@pbyte.xyz>
- **Status:** Accepted
- **Created:** 2024-03-30
- **Last updated:** 2024-03-30

## Overview

This RFC proposes the adoption of PostgreSQL as the primary database management system for the project.

## Goals and Non-Goals:

- Standardize database management from the project's inception.
- Leverage PostgreSQL's robust features for reliability, performance, and scalability.
- Simplify database administration and maintenance tasks for the project.
- Establish a strong foundation for future growth and development.

## Background & Motivation:

As we embark on a new project, it's crucial to establish a solid infrastructure from the outset. PostgreSQL offers a mature and feature-rich solution that aligns with our project's requirements for reliability, performance, and extensibility.

## Design:

The adoption of PostgreSQL for the new project involves the following steps:

- **Schema Design:** Design the database schema using PostgreSQL's relational capabilities, taking advantage of features such as constraints, indexes, and foreign keys to ensure data integrity and performance.
- **Application Integration:** Integrate PostgreSQL with the project's application stack. On the Rust side, SQLx will be used as the database driver to interact with PostgreSQL. SQLx provides asynchronous, type-safe database access, which aligns well with Rust's strengths. All database-related logic will be encapsulated within the database module of the domain section, ensuring clear separation of concerns and maintainability.

## Alternatives Considered/Prior Art:

- **MySQL:** While MySQL is a popular choice, PostgreSQL offers advanced features such as support for JSONB data type, better concurrency control, and a more robust ACID compliance.
- **NoSQL Solutions:** NoSQL databases provide flexibility for certain use cases, but PostgreSQL's support for both relational and NoSQL data structures makes it a versatile option without sacrificing ACID compliance.
- **Proprietary Solutions:** Proprietary databases offer vendor-specific features, but PostgreSQL's open-source nature provides flexibility, cost-effectiveness, and a vibrant community for support and development.

## Risks:

- **Learning Curve:** Team members unfamiliar with PostgreSQL may require time to adapt to its features and administration tools, potentially affecting productivity during the transition period.
- **Dependency on Third-Party Extensions:** Certain project requirements may rely on third-party extensions or features not directly supported by PostgreSQL, necessitating custom development or alternative solutions.

## Revisions:

1. *2024-03-30* - RFC Created
