//! This module implements reusable components for constructing the web interface
//! of the application.
//!
//! # ATOMS
//! Atoms are the basic building blocks of our UI, for example:
//! - inputs, buttons, titles, icons;
//! - color palettes, fonts, animations.
//!
//! # MOLECULES
//! Molecules are a group of atoms that work together, for example:
//! - a labeled input - combines a label and an input.
//!
//! # ORGANISMS
//! Organisms are a combination of atoms, molecules and other organisms that
//! work together to provide the distinct section of the interface, for example:
//! - the login form - a combination of several inputs, a link and a submit button.
//!
//! # TEMPLATES
//! Templates are wireframes/layouts of pages, for example:
//! - a page with navigation and any content.
//!
//! # PAGES
//! Pages are implementation of concrete pages of our interface.
//!

mod atoms;
mod molecules;
pub mod organisms;
pub mod pages;
pub mod templates;

pub use atoms::Icon;
