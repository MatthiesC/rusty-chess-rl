# AGENTS.md

## Project Overview

This repository contains a chess environment for reinforcement learning.

Goals:

1. Implement a correct and well-tested chess rules engine in Rust.
2. Expose the engine to Python via PyO3.
3. Train chess-playing agents through self-play in Python.

Performance matters, but correctness matters more.

---

## Architecture

Workspace crates:

* `crates/chess-core`: Pure Rust chess rules and game state.
* `crates/chess-py`: Python bindings using PyO3.

Python code:

* `python/`: reinforcement learning, self-play, experiments, training utilities. Machine learning is done with PyTorch.

The Rust engine must not depend on Python code.

---

## Development Principles

* Prefer correctness over optimization.
* Prefer simple solutions over clever solutions.
* Avoid premature optimization.
* Keep dependencies minimal.
* Use explicit types instead of strings.
* Favor enums over magic constants.
* Keep changes small and focused.
* Do not introduce large abstractions before they are needed.

---

## Rust Guidelines

* Use stable Rust.
* Public crates should expose a small, clear API.
* Prefer strong domain types:

  * `Color`
  * `PieceKind`
  * `Piece`
  * `Square`
  * `Move`
  * `Board`
  * `GameState`
* Avoid stringly typed APIs.
* Avoid panics in library code unless an invariant is clearly violated.
* Use `Result` for recoverable errors.
* Prefer exhaustive pattern matching.
* Keep the Rust chess engine deterministic.

---

## Python Guidelines

Python version: 3.13

Use `uv` for all Python dependency and environment management.

Allowed:

* `uv sync`
* `uv add`
* `uv remove`
* `uv run`
* `uv python install`

Do not use:

* `pip`
* `virtualenv`
* `venv`
* `poetry`
* `conda`

All Python code must:

* use type hints
* use Google-style docstrings
* pass `ruff`
* pass `mypy`
* avoid untyped public APIs
* keep training code separate from environment bindings

---

## Documentation

### Rust

All public types, traits, enums, structs, functions, and modules must contain Rust documentation comments.

Use Rust doc comments:

```rust
/// Represents a chess board state.
pub struct Board {
    // ...
}
```

Documentation should explain:

* purpose
* invariants
* important implementation details
* parameters and return values where helpful

Examples are encouraged for public APIs.

Rust documentation should build successfully with:

```bash
cargo doc --workspace --no-deps
```

Documentation warnings should be treated as defects.

### Python

Use Google-style docstrings.

Example:

```python
def legal_moves(position: Position) -> list[Move]:
    """Return all legal moves for a position.

    Args:
        position: Current chess position.

    Returns:
        List of legal moves.
    """
```

All public functions, classes, and modules must be documented.

Type hints are required for all public APIs.

---

## Testing and Quality Checks

Before every commit, run:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
cargo test --workspace
cargo doc --workspace --no-deps
uv run ruff check .
uv run mypy .
```

Do not commit code with failing tests, formatting errors, lint errors, or type-checking errors.

Add tests for all new functionality.

Prefer test-driven development when practical.

---

## Chess Rules Implementation Order

Implement chess rules in this order:

1. Board representation
2. FEN parsing and serialization
3. Pseudo-legal move generation
4. Legal move validation
5. Check detection
6. Checkmate and stalemate detection
7. Castling
8. En passant
9. Promotion
10. Draw rules
11. Perft tests

Do not skip ahead unless explicitly instructed.

---

## Rust Chess Engine Scope

The Rust engine is responsible for:

* board representation
* move representation
* legal move generation
* move application
* game state tracking
* FEN support
* deterministic simulation
* exposing a stable API for Python bindings

The Rust engine is not responsible for:

* neural networks
* reinforcement learning
* training loops
* model evaluation
* plotting
* experiment tracking

Those belong in Python.

---

## Python Reinforcement Learning Scope

Python is responsible for:

* self-play loops
* environment wrappers
* policy/value models
* training loops
* evaluation
* logging
* experiment scripts

Keep the initial RL implementation simple. The ML package to use is PyTorch.

Do not implement AlphaZero-style MCTS unless explicitly requested.

Start with:

1. random self-play
2. simple policy model
3. simple value model
4. basic training loop

---

## PyO3 Boundary

The `chess-py` crate exposes the Rust engine to Python.

Keep the Python API simple and environment-like:

```python
env = ChessEnv()
obs = env.reset()
legal_moves = env.legal_moves()
obs, reward, done, info = env.step(move)
```

The Python API should be stable, documented, and typed where possible.

Do not leak unnecessary Rust internals into Python.

---

## Git Workflow

* Work on feature branches.
* Keep commits small and atomic.
* One logical change per commit.
* Use conventional commit messages.
* Run all checks before committing.
* Do not push unless explicitly instructed.
* Do not force-push unless explicitly instructed.
* Do not rewrite history unless explicitly instructed.

Good commit message examples:

```text
feat(core): add board representation
feat(core): implement FEN parsing
test(core): add starting position tests
docs: add architecture overview
chore: configure workspace checks
```

---

## Dependency Policy

Avoid unnecessary dependencies.

Before adding a dependency, explain:

1. why it is needed
2. why the standard library is insufficient
3. whether it affects runtime, build time, or Python packaging

Rust dependencies should be minimal.

Python dependencies should be managed only through `uv`.

---

## Agent Behavior

When working as an agent:

1. Read this file before making changes.
2. Read relevant source files before editing.
3. Make the smallest useful change.
4. Run the relevant checks.
5. Explain what changed.
6. Do not silently ignore failing checks.
7. Ask for clarification before large architectural changes.
8. Do not introduce unrelated changes.
9. Do not change formatting conventions without instruction.
10. Do not modify secrets, credentials, tokens, or private keys.

---

## When Unsure

Prefer:

* simple over clever
* explicit over implicit
* tested over assumed
* documented over tribal knowledge
* correctness over speed

Ask before making major architectural decisions.
