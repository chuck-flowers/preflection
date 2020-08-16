# Preflect

Preflect is an experimental Rust library used to simulate simple runtime 
reflection. This feature is typically only available in languages that include 
a heavier runtime such as C# or Java, but with Rust's type system, some simple 
use cases of reflection are possible.

**DISCLAIMER:** This library is highly experimental and is only a proof of 
concept. This should not be used in a production environment.

## Features

This is a table of optional features that can be enabled on this crate.

| Name        | Description                                                                                                                                                             |
| ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `has-field` | Adds the `HasField` trait and derive macro with allows for encoding of field names and types into the type system. This feature is currently only available on nightly. |
