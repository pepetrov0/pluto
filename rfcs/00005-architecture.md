# Architecture

- **Author(s):** <ppetrov@pbyte.xyz>
- **Approver(s):** <ppetrov@pbyte.xyz>
- **Status:** Implemented
- **Created:** 2024-03-31
- **Last updated:** 2024-03-31

## Overview

This RFC proposes the implementation of the Action-Domain-Responder (ADR) 
pattern within our system architecture to improve code organization, 
maintainability, and testability.

## Goals and Non-Goals:

### Goals:

- Enhance code organization and readability.
- Improve testability and maintainability.
- Promote separation of concerns within the system architecture.

### Non-Goals:

- Fundamental changes to existing system functionality.
- Introducing significant overhead in terms of performance or resource utilization.

## Background & Motivation:

Currently, our system architecture lacks a clear structure for organizing 
business logic, leading to issues with maintainability and testability. 

By adopting the Action-Domain-Responder pattern, we aim to address these 
concerns by defining clear boundaries between presentation logic, domain logic, 
and action logic.

### Key Terms:

- **Action:** Represents the handling of a specific HTTP request or user interaction.
- **Domain:** Encompasses the core business logic and rules.
- **Responder:** Handles the response generation based on the output of domain logic execution.

## Design:

The Action-Domain-Responder pattern separates the concerns of handling an HTTP request into three distinct components:

- **Action:** Receives the HTTP request, coordinates the execution of domain logic, and delegates response generation to a responder.
- **Domain:** Contains the core business logic, including validation, processing, and decision making.
- **Responder:** Generates an HTTP response based on the output of domain logic execution.

For the implementation of this pattern, we propose using three Rust modules, each corresponding to one of the components mentioned above.

## Dependencies:

The implementation of the Action-Domain-Responder pattern does not introduce 
any new external dependencies. However, it relies on existing components within 
our system, such as the HTTP server framework and the domain logic modules.

## Alternatives Considered/Prior Art:

Several alternatives were considered for improving code organization and 
testability, including the Model-View-Controller (MVC) pattern. However, these patterns were deemed less suitable for our requirements.

## Risks:

- **Complexity:** Adopting the Action-Domain-Responder pattern may introduce complexity in the initial implementation phase. However, proper documentation will mitigate this risk.

## Revisions:

1. *2024-03-31* - RFC Created

