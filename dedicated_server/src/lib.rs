#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![feature(iter_intersperse)]

#[macro_use]
extern crate slog;

pub mod api;
pub mod config;
pub mod storage;
