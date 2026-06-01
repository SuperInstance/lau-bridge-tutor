# lau-bridge-tutor

> Part of the PLATO/LAU mathematical agent framework

## What This Does

Part of the PLATO/LAU mathematical agent framework. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-bridge-tutor
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_bridge_tutor::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub enum PlatoConcept 
    pub fn kid_name(&self) -> String 
    pub fn kid_explanation(&self) -> String 
    pub fn game_mechanic(&self) -> String 
    pub fn example_quest(&self) -> String 
pub struct EncounterRecord 
    pub fn new(first_tick: u64) -> Self 
pub struct Prerequisites;
    pub fn of(concept: &PlatoConcept) -> Vec<PlatoConcept> 
    pub fn ordered() -> Vec<PlatoConcept> 
pub struct ConceptMap 
    pub fn new() -> Self 
    pub fn encounter(&mut self, concept: PlatoConcept, tick: u64, score: Option<f64>) 
    pub fn mastery_level(&self, concept: &PlatoConcept) -> f64 
    pub fn next_concept(&self) -> Option<PlatoConcept> 
pub enum PlatoEvent 
pub enum GameEvent 
pub struct BridgeTranslator;
    pub fn plato_to_game_event(&self, plato_event: &PlatoEvent) -> GameEvent 
    pub fn game_to_plato_concept(&self, game_action: &str) -> Option<PlatoConcept> 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**50 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
