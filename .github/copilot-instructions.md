# GitHub Copilot Instructions for `rust_math_practice`

These instructions are for AI coding agents working in this repo.

## Project Overview
- **Purpose**: Rust solutions to problems from [Project Euler](https://projecteuler.net/).
- **Crate type**: Single binary crate with entrypoint `src/main.rs`.
- **Structure**:
  - `src/main.rs`: Program entry; chooses which Project Euler solver(s) to run.
  - `src/solvers/`: Each file `pN.rs` contains solution `pN()` for problem N.
  - `src/solvers/mod.rs`: Exposes each solver as `solvers::pN()` via `pub mod pN; pub use pN::pN;`.
  - `src/utility.rs`: Reusable math helpers (currently `is_prime`).

## How to Run and Develop
- Use standard Cargo commands from repo root:
  - Build: `cargo build`
  - Run (current main): `cargo run`
- `main()` currently hard‑codes which problems run. To experiment with another solver:
  - Import via `mod solvers;` (already present).
  - Call the desired function, e.g. `solvers::p1();` inside `main()`.
  - You can temporarily comment out other solver calls or gate them with flags/booleans, following the existing `skip` pattern.

## Conventions and Patterns
- **One file per Project Euler problem**:
  - File: `src/solvers/pN.rs`.
  - Public API: `pub fn pN()` with no parameters; it prints the answer via `println!` or `print!`.
  - Module export: add to `src/solvers/mod.rs`:
    - `pub mod pN;`
    - `pub use pN::pN;`
  - `main.rs` calls `solvers::pN()` directly.
- **Printing vs returning**:
  - Existing solvers print their result and do not return it.
  - When adding new solvers, follow this style unless the user explicitly requests a different API.
- **Numeric types**:
  - Use `i64` for larger integers and `i32` for smaller loop counters/accumulators, mirroring existing solutions (`p1`–`p5`).
  - For prime checks or factorization, reuse or extend `utility::is_prime` instead of reimplementing.
- **Simple, direct algorithms**:
  - Solutions favor readability and straightforward loops over heavy abstractions.
  - It is acceptable to use `while` loops, manual counters, and early `return` for clarity.

## Example: Adding a New Problem Solver
When implementing `p6` (for example):
- Create `src/solvers/p6.rs` with `pub fn p6() { /* compute + print */ }`.
- Update `src/solvers/mod.rs` with `pub mod p6;` and `pub use p6::p6;`.
- Update `src/main.rs` to call `solvers::p6();` in `main()`.

## Style and Coding Guidelines
- Stick to the existing Rust style:
  - Use explicit type annotations for key variables (`let mut sum: i32 = 0;`).
  - Prefer early returns for simple checks (as in `utility::is_prime`).
  - Keep each solver self‑contained; only move logic to `utility.rs` if reused.
- Do **not** introduce additional crates or dependencies in `Cargo.toml` unless explicitly requested.

## Testing and Validation
- There is no formal test suite; validation is by running the binary and inspecting stdout.
- For new or refactored solvers, add temporary debug `println!`s if needed, but remove noisy debug output once the solution is correct.

If any conventions here seem mismatched with future changes in the repo, please adjust this file to reflect the actual, current patterns before generating large changes.