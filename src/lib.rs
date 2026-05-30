//! `lau-bridge-tutor` — the bridge between PLATO research concepts and Lau game mechanics.
//!
//! This crate maps abstract PLATO learning concepts into kid-friendly game actions,
//! tracks player mastery, defines prerequisite learning orders, and translates
//! between PLATO events and Lau game events.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// PlatoConcept — the core PLATO learning concepts
// ---------------------------------------------------------------------------

/// A research concept from the PLATO framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PlatoConcept {
    Conservation,
    JEPA,
    RoomLifecycle,
    Distillation,
    Vibe,
    Deadband,
    FibonacciGrowth,
    Topology,
    SignalChain,
    Murmur,
}

impl PlatoConcept {
    /// A name a 10-year-old would understand.
    pub fn kid_name(&self) -> String {
        match self {
            Self::Conservation => "Balance".into(),
            Self::JEPA => "Prediction".into(),
            Self::RoomLifecycle => "Growing Rooms".into(),
            Self::Distillation => "Extract".into(),
            Self::Vibe => "Vibe".into(),
            Self::Deadband => "Quiet Zone".into(),
            Self::FibonacciGrowth => "Spiral Growth".into(),
            Self::Topology => "Shape Shifter".into(),
            Self::SignalChain => "Signal Chain".into(),
            Self::Murmur => "Murmur".into(),
        }
    }

    /// A one‑sentence explanation a 10-year-old can digest.
    pub fn kid_explanation(&self) -> String {
        match self {
            Self::Conservation => {
                "When you change something, the total amount of stuff stays the same \
                 — it just looks different."
                    .into()
            }
            Self::JEPA => {
                "Your brain makes a little movie of what will happen next, \
                 so you can be ready before it actually happens."
                    .into()
            }
            Self::RoomLifecycle => {
                "Rooms grow up, just like you — they start small, get bigger, \
                 get messy, and sometimes get a fresh coat of paint."
                    .into()
            }
            Self::Distillation => {
                "Take a big messy pile of information and squeeze out only the \
                 important parts — like making concentrated juice."
                    .into()
            }
            Self::Vibe => {
                "Every room, every creature, every object has a feeling — \
                 and when you change one thing, the feeling spreads."
                    .into()
            }
            Self::Deadband => {
                "Sometimes the best thing to do is nothing — small changes \
                 don't always need a reaction."
                    .into()
            }
            Self::FibonacciGrowth => {
                "Nature has a secret number pattern that makes things grow \
                 in beautiful spirals — like sunflowers and pinecones."
                    .into()
            }
            Self::Topology => {
                "A shape is still the same shape even if you stretch it, \
                 squish it, or twist it — as long as you don't tear it."
                    .into()
            }
            Self::SignalChain => {
                "Messages travel from one place to another like a bucket \
                 brigade — if one person drops it, the message is lost."
                    .into()
            }
            Self::Murmur => {
                "Lots of tiny quiet voices can add up to something big — \
                 like a crowd whispering all at once."
                    .into()
            }
        }
    }

    /// How this concept manifests as a game mechanic.
    pub fn game_mechanic(&self) -> String {
        match self {
            Self::Conservation => {
                "Resources can be transformed but never lost — trading 3 wood \
                 for 1 plank keeps the same 'value'."
                    .into()
            }
            Self::JEPA => {
                "The pet predicts where the next collectible will appear; \
                 you score bonus points for going there first."
                    .into()
            }
            Self::RoomLifecycle => {
                "Rooms gain XP and evolve through stages: seedling → blooming \
                 → sparkling — each level unlocks new decorations."
                    .into()
            }
            Self::Distillation => {
                "Combine 5 common items into 1 rare essence that powers \
                 special abilities."
                    .into()
            }
            Self::Vibe => {
                "Placing a happy object raises the room's mood meter, \
                 which affects how creatures behave."
                    .into()
            }
            Self::Deadband => {
                "Minor temperature or mood changes (&plusmn;5 %) are ignored — \
                 the game doesn't punish tiny fluctuations."
                    .into()
            }
            Self::FibonacciGrowth => {
                "Pets and plants level up following the Fibonacci sequence: \
                 1 &rarr; 1 &rarr; 2 &rarr; 3 &rarr; 5 &rarr; 8 &rarr; 13 ..."
                    .into()
            }
            Self::Topology => {
                "The player can reshape rooms (stretch, shrink, rotate) \
                 without losing the items inside."
                    .into()
            }
            Self::SignalChain => {
                "Chain 3+ matching objects to trigger a combo cascade — the \
                 longer the chain, the bigger the reward."
                    .into()
            }
            Self::Murmur => {
                "Each pet in a room adds a small hum; when 7+ are together \
                 they unlock a hidden 'Murmur melody'."
                    .into()
            }
        }
    }

    /// A quest that teaches this concept.
    pub fn example_quest(&self) -> String {
        match self {
            Self::Conservation => {
                "Trade 10 apples for 5 apple pies, then trade the pies back \
                 — prove the total value never changed."
                    .into()
            }
            Self::JEPA => {
                "Watch your pet predict where 3 raindrops will fall — place \
                 a bucket on each predicted spot before the drop lands."
                    .into()
            }
            Self::RoomLifecycle => {
                "Raise one room from 'dusty basement' to 'shining star room' \
                 by completing 10 room tasks."
                    .into()
            }
            Self::Distillation => {
                "Collect 5 different flowers and distill them into one \
                 'Essence of Spring' to revive a wilted garden."
                    .into()
            }
            Self::Vibe => {
                "Change the vibe of a sad room from 20 % to 80 % by placing \
                 3 happy objects, without using any sad objects."
                    .into()
            }
            Self::Deadband => {
                "Let the room temperature drift between 18&deg;C and 22&deg;C \
                 for 5 minutes — no adjustments needed!"
                    .into()
            }
            Self::FibonacciGrowth => {
                "Grow a sunflower from level 1 to level 8, discovering the \
                 Fibonacci pattern in its seed spiral."
                    .into()
            }
            Self::Topology => {
                "Reshape your bedroom into a circle, then a triangle, then \
                 back to a square — keep all furniture inside each shape."
                    .into()
            }
            Self::SignalChain => {
                "Build a chain of 5 sound crystals from the microphone to \
                 the speaker — the melody must reach the end without breaking."
                    .into()
            }
            Self::Murmur => {
                "Gather 7 whispering pets in one room and stand still for \
                 10 seconds to hear the hidden Murmur song."
                    .into()
            }
        }
    }
}

// ---------------------------------------------------------------------------
// EncounterRecord — per-concept player history
// ---------------------------------------------------------------------------

/// Tracks a single concept's encounter history for one player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncounterRecord {
    /// Game tick when the player first encountered this concept.
    pub first_tick: u64,
    /// How many times they have encountered it.
    pub times_encountered: u32,
    /// Their best score (0.0 – 1.0) on related challenges.
    pub best_score: f64,
    /// Whether the player has mastered this concept (score ≥ 0.85).
    pub mastered: bool,
}

impl EncounterRecord {
    /// Create a new encounter record marking a first encounter.
    pub fn new(first_tick: u64) -> Self {
        Self {
            first_tick,
            times_encountered: 1,
            best_score: 0.0,
            mastered: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Prerequisites — learning order
// ---------------------------------------------------------------------------

/// Statically defines which concepts must be encountered before others.
///
/// The ordering is:
///   - Vibe → Deadband → Conservation → Distillation
///   - Vibe → JEPA → RoomLifecycle
///   - Vibe → SignalChain → Murmur
///   - FibonacciGrowth → Topology (after Conservation)
pub struct Prerequisites;

impl Prerequisites {
    /// Return the concepts that are direct prerequisites of `concept`.
    ///
    /// The order is significant — the earlier prerequisite should be learned
    /// first when multiple exist.
    pub fn of(concept: &PlatoConcept) -> Vec<PlatoConcept> {
        match concept {
            PlatoConcept::Deadband => vec![PlatoConcept::Vibe],
            PlatoConcept::Conservation => vec![PlatoConcept::Deadband],
            PlatoConcept::Distillation => vec![PlatoConcept::Conservation],
            PlatoConcept::JEPA => vec![PlatoConcept::Vibe],
            PlatoConcept::RoomLifecycle => vec![PlatoConcept::JEPA],
            PlatoConcept::SignalChain => vec![PlatoConcept::Vibe],
            PlatoConcept::Murmur => vec![PlatoConcept::SignalChain],
            PlatoConcept::Topology => vec![PlatoConcept::FibonacciGrowth, PlatoConcept::Conservation],
            // Vibe and FibonacciGrowth have no prerequisites.
            PlatoConcept::Vibe | PlatoConcept::FibonacciGrowth => vec![],
        }
    }

    /// All concepts in recommended learning order (topological sort).
    pub fn ordered() -> Vec<PlatoConcept> {
        vec![
            PlatoConcept::Vibe,
            PlatoConcept::FibonacciGrowth,
            PlatoConcept::Deadband,
            PlatoConcept::JEPA,
            PlatoConcept::Conservation,
            PlatoConcept::SignalChain,
            PlatoConcept::Distillation,
            PlatoConcept::RoomLifecycle,
            PlatoConcept::Murmur,
            PlatoConcept::Topology,
        ]
    }
}

// ---------------------------------------------------------------------------
// ConceptMap — tracks which concepts a player has encountered
// ---------------------------------------------------------------------------

/// The player's personal concept progress map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMap {
    pub encountered: HashMap<PlatoConcept, EncounterRecord>,
}

impl ConceptMap {
    /// Create an empty concept map.
    pub fn new() -> Self {
        Self {
            encountered: HashMap::new(),
        }
    }

    /// Mark a concept as encountered at the given tick.
    ///
    /// If already encountered, increments the counter and updates `best_score`
    /// if the current score improves. The `score` is an option because not
    /// every encounter carries a challenge result.
    pub fn encounter(&mut self, concept: PlatoConcept, tick: u64, score: Option<f64>) {
        let record = self
            .encountered
            .entry(concept)
            .and_modify(|r| {
                r.times_encountered += 1;
                if let Some(s) = score && s > r.best_score {
                    r.best_score = s;
                    if s >= 0.85 {
                        r.mastered = true;
                    }
                }
            })
            .or_insert_with(|| {
                let mut r = EncounterRecord::new(tick);
                if let Some(s) = score {
                    r.best_score = s;
                    if s >= 0.85 {
                        r.mastered = true;
                    }
                }
                r
            });
        // Keep first_tick unchanged if we already had one.
        if record.times_encountered == 0 {
            // Should not happen because or_insert_with created it with 1,
            // but guard against future refactors.
            record.first_tick = tick;
        }
    }

    /// Estimate mastery level (0.0 – 1.0) for a concept based on
    /// best_score and how many times encountered (diminishing returns).
    pub fn mastery_level(&self, concept: &PlatoConcept) -> f64 {
        match self.encountered.get(concept) {
            None => 0.0,
            Some(r) => {
                // Base mastery from best score.
                let base = r.best_score;
                // Diminishing bonus from repeated encounters: +0.05 for each
                // encounter beyond the first, capped at total +0.15.
                let bonus =
                    (0.05_f64).mul_add((r.times_encountered.saturating_sub(1)) as f64, 0.0);
                let bonus = bonus.min(0.15);
                (base + bonus).min(1.0)
            }
        }
    }

    /// Suggest the next concept the player should learn, based on
    /// the prerequisite graph.
    pub fn next_concept(&self) -> Option<PlatoConcept> {
        for candidate in Prerequisites::ordered() {
            if self.encountered.contains_key(&candidate) {
                // Already started; skip unless mastered.
                if !self.encountered[&candidate].mastered {
                    return Some(candidate);
                }
                continue;
            }
            // Not yet encountered — check prerequisites.
            let prereqs = Prerequisites::of(&candidate);
            let all_prereqs_mastered = prereqs
                .iter()
                .all(|p| self.encountered.get(p).is_some_and(|r| r.mastered));
            if all_prereqs_mastered {
                return Some(candidate);
            }
        }
        None
    }
}

impl Default for ConceptMap {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// PlatoEvent — events emitted by the PLATO research system
// ---------------------------------------------------------------------------

/// An event originating from the PLATO research system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatoEvent {
    VibeChanged {
        old: f64,
        new: f64,
    },
    ConservationChecked {
        error: f64,
    },
    RoomStateChanged {
        room: String,
        from: String,
        to: String,
    },
    AgentPredicted {
        accuracy: f64,
    },
    SignalReceived {
        source: String,
        strength: f64,
    },
}

// ---------------------------------------------------------------------------
// GameEvent — events consumable by the Lau game engine
// ---------------------------------------------------------------------------

/// An event the Lau game engine can consume.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    WeatherChanged {
        to: String,
    },
    StructureBuilt {
        name: String,
    },
    QuestProgress {
        quest: String,
        percent: f64,
    },
    PetEvolved {
        from: String,
        to: String,
    },
    AchievementUnlocked {
        badge: String,
    },
    MusicChanged {
        mood: String,
    },
}

// ---------------------------------------------------------------------------
// BridgeTranslator — translates between PLATO and game events
// ---------------------------------------------------------------------------

/// Translates PLATO research events into Lau game events and vice‑versa.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTranslator;

impl BridgeTranslator {
    /// Map a PLATO event to a corresponding game event.
    pub fn plato_to_game_event(&self, plato_event: &PlatoEvent) -> GameEvent {
        match plato_event {
            PlatoEvent::VibeChanged { old: _, new } => {
                let mood = if *new > 0.7 {
                    "joyful"
                } else if *new > 0.4 {
                    "calm"
                } else {
                    "gloomy"
                };
                GameEvent::MusicChanged {
                    mood: mood.to_string(),
                }
            }
            PlatoEvent::ConservationChecked { error } => {
                let badge = if *error < 0.05 {
                    "Perfect Balance"
                } else if *error < 0.15 {
                    "Close Enough"
                } else {
                    "Keep Trying"
                };
                GameEvent::AchievementUnlocked {
                    badge: badge.to_string(),
                }
            }
            PlatoEvent::RoomStateChanged { room, from, to } => {
                GameEvent::StructureBuilt {
                    name: format!("{room} [{from} → {to}]"),
                }
            }
            PlatoEvent::AgentPredicted { accuracy } => {
                GameEvent::QuestProgress {
                    quest: "Prediction Master".into(),
                    percent: (*accuracy * 100.0).min(100.0),
                }
            }
            PlatoEvent::SignalReceived { source, strength } => {
                let weather = if *strength > 0.8 {
                    "stormy"
                } else if *strength > 0.4 {
                    "cloudy"
                } else {
                    "clear"
                };
                GameEvent::WeatherChanged {
                    to: format!("{source}:{weather}"),
                }
            }
        }
    }

    /// Reverse‑lookup: which PLATO concept is most associated with a
    /// game action string.
    pub fn game_to_plato_concept(&self, game_action: &str) -> Option<PlatoConcept> {
        let action = game_action.to_lowercase();
        if action.contains("trade") || action.contains("exchange") || action.contains("convert") {
            Some(PlatoConcept::Conservation)
        } else if action.contains("predict") || action.contains("forecast") {
            Some(PlatoConcept::JEPA)
        } else if action.contains("room") && (action.contains("grow") || action.contains("evolve")) {
            Some(PlatoConcept::RoomLifecycle)
        } else if action.contains("distill") || action.contains("extract") || action.contains("essence") {
            Some(PlatoConcept::Distillation)
        } else if action.contains("vibe") || action.contains("mood") || action.contains("feeling") {
            Some(PlatoConcept::Vibe)
        } else if action.contains("quiet") || action.contains("ignore") || action.contains("deadband") {
            Some(PlatoConcept::Deadband)
        } else if action.contains("spiral") || action.contains("fibonacci") || action.contains("level up") {
            Some(PlatoConcept::FibonacciGrowth)
        } else if action.contains("reshape") || action.contains("stretch") || action.contains("topology") || action.contains("shrink") {
            Some(PlatoConcept::Topology)
        } else if action.contains("chain") || action.contains("signal") || action.contains("cascade") || action.contains("combo") {
            Some(PlatoConcept::SignalChain)
        } else if action.contains("whisper") || action.contains("murmur") || action.contains("hum") {
            Some(PlatoConcept::Murmur)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // PlatoConcept tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_plato_concept_variants_exist() {
        // Every variant must be instantiable.
        let concepts = vec![
            PlatoConcept::Conservation,
            PlatoConcept::JEPA,
            PlatoConcept::RoomLifecycle,
            PlatoConcept::Distillation,
            PlatoConcept::Vibe,
            PlatoConcept::Deadband,
            PlatoConcept::FibonacciGrowth,
            PlatoConcept::Topology,
            PlatoConcept::SignalChain,
            PlatoConcept::Murmur,
        ];
        assert_eq!(concepts.len(), 10);
    }

    #[test]
    fn test_kid_names_are_unique() {
        // Kid names should be distinct for clarity.
        let mut names: Vec<String> = Vec::new();
        for c in &[
            PlatoConcept::Conservation,
            PlatoConcept::JEPA,
            PlatoConcept::RoomLifecycle,
            PlatoConcept::Distillation,
            PlatoConcept::Vibe,
            PlatoConcept::Deadband,
            PlatoConcept::FibonacciGrowth,
            PlatoConcept::Topology,
            PlatoConcept::SignalChain,
            PlatoConcept::Murmur,
        ] {
            names.push(c.kid_name());
        }
        let mut deduped = names.clone();
        deduped.sort();
        deduped.dedup();
        assert_eq!(names.len(), deduped.len(), "kid names must be unique");
    }

    #[test]
    fn test_explanations_are_non_empty() {
        for c in &[
            PlatoConcept::Conservation,
            PlatoConcept::JEPA,
            PlatoConcept::RoomLifecycle,
            PlatoConcept::Distillation,
            PlatoConcept::Vibe,
            PlatoConcept::Deadband,
            PlatoConcept::FibonacciGrowth,
            PlatoConcept::Topology,
            PlatoConcept::SignalChain,
            PlatoConcept::Murmur,
        ] {
            let e = c.kid_explanation();
            assert!(!e.is_empty(), "explanation for {c:?} is empty");
            assert!(e.len() >= 40, "explanation for {c:?} too short: {e}");
        }
    }

    #[test]
    fn test_game_mechanics_are_non_empty() {
        for c in &[
            PlatoConcept::Conservation,
            PlatoConcept::JEPA,
            PlatoConcept::RoomLifecycle,
            PlatoConcept::Distillation,
            PlatoConcept::Vibe,
            PlatoConcept::Deadband,
            PlatoConcept::FibonacciGrowth,
            PlatoConcept::Topology,
            PlatoConcept::SignalChain,
            PlatoConcept::Murmur,
        ] {
            assert!(!c.game_mechanic().is_empty(), "mechanic for {c:?} is empty");
        }
    }

    #[test]
    fn test_example_quests_are_non_empty() {
        for c in &[
            PlatoConcept::Conservation,
            PlatoConcept::JEPA,
            PlatoConcept::RoomLifecycle,
            PlatoConcept::Distillation,
            PlatoConcept::Vibe,
            PlatoConcept::Deadband,
            PlatoConcept::FibonacciGrowth,
            PlatoConcept::Topology,
            PlatoConcept::SignalChain,
            PlatoConcept::Murmur,
        ] {
            assert!(!c.example_quest().is_empty(), "quest for {c:?} is empty");
        }
    }

    // -----------------------------------------------------------------------
    // EncounterRecord tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_encounter_record_new() {
        let r = EncounterRecord::new(42);
        assert_eq!(r.first_tick, 42);
        assert_eq!(r.times_encountered, 1);
        assert_eq!(r.best_score, 0.0);
        assert!(!r.mastered);
    }

    // -----------------------------------------------------------------------
    // ConceptMap tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_concept_map_empty() {
        let map = ConceptMap::new();
        assert!(map.encountered.is_empty());
        assert_eq!(map.mastery_level(&PlatoConcept::Vibe), 0.0);
    }

    #[test]
    fn test_concept_map_first_encounter() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Vibe, 100, None);
        let rec = map.encountered.get(&PlatoConcept::Vibe).unwrap();
        assert_eq!(rec.first_tick, 100);
        assert_eq!(rec.times_encountered, 1);
    }

    #[test]
    fn test_concept_map_second_encounter() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Vibe, 100, None);
        map.encounter(PlatoConcept::Vibe, 200, Some(0.5));
        let rec = map.encountered.get(&PlatoConcept::Vibe).unwrap();
        assert_eq!(rec.first_tick, 100); // unchanged
        assert_eq!(rec.times_encountered, 2);
        assert!((rec.best_score - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_concept_map_best_score_improves() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Vibe, 100, Some(0.3));
        map.encounter(PlatoConcept::Vibe, 200, Some(0.9));
        let rec = map.encountered.get(&PlatoConcept::Vibe).unwrap();
        assert!((rec.best_score - 0.9).abs() < 1e-9);
        assert!(rec.mastered);
    }

    #[test]
    fn test_mastery_level_no_encounter() {
        let map = ConceptMap::new();
        assert_eq!(map.mastery_level(&PlatoConcept::JEPA), 0.0);
    }

    #[test]
    fn test_mastery_level_improves_with_practice() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Deadband, 1, Some(0.6));
        let lvl1 = map.mastery_level(&PlatoConcept::Deadband);
        map.encounter(PlatoConcept::Deadband, 2, Some(0.6));
        let lvl2 = map.mastery_level(&PlatoConcept::Deadband);
        assert!(lvl2 > lvl1, "repetition should increase mastery");
    }

    #[test]
    fn test_mastery_level_capped_at_one() {
        let mut map = ConceptMap::new();
        // Many encounters with perfect score.
        for i in 0..20 {
            map.encounter(PlatoConcept::Conservation, i, Some(1.0));
        }
        let level = map.mastery_level(&PlatoConcept::Conservation);
        assert!(level <= 1.0 + 1e-12);
    }

    // -----------------------------------------------------------------------
    // Next concept tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_next_concept_starts_with_vibe_or_fibonacci() {
        let map = ConceptMap::new();
        let next = map.next_concept();
        assert!(next.is_some());
        let n = next.unwrap();
        // Vibe and FibonacciGrowth have no prerequisites.
        assert!(
            n == PlatoConcept::Vibe || n == PlatoConcept::FibonacciGrowth,
            "first concept should be Vibe or FibonacciGrowth, got {n:?}"
        );
    }

    #[test]
    fn test_next_concept_after_vibe() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Vibe, 0, Some(0.9));
        // Vibe mastered → next should be FibonacciGrowth or one of Vibe's
        // children (Deadband, JEPA, SignalChain).
        let next = map.next_concept().unwrap();
        assert!(
            next == PlatoConcept::FibonacciGrowth
                || next == PlatoConcept::Deadband
                || next == PlatoConcept::JEPA
                || next == PlatoConcept::SignalChain,
            "unexpected next: {next:?}"
        );
    }

    #[test]
    fn test_next_concept_respects_prerequisites() {
        let mut map = ConceptMap::new();
        // Master Vibe.
        map.encounter(PlatoConcept::Vibe, 0, Some(0.9));
        // Also master FibonacciGrowth so it doesn't block Topology later.
        map.encounter(PlatoConcept::FibonacciGrowth, 0, Some(0.9));
        // Next should NOT be Topology because Conservation isn't mastered.
        assert!(
            map.next_concept() != Some(PlatoConcept::Topology),
            "Topology requires Conservation, which isn't mastered yet"
        );
    }

    #[test]
    fn test_next_concept_returns_none_when_done() {
        let mut map = ConceptMap::new();
        // Master everything.
        for c in Prerequisites::ordered() {
            map.encounter(c, 0, Some(0.9));
        }
        assert!(map.next_concept().is_none(), "all concepts mastered");
    }

    #[test]
    fn test_next_concept_returns_unmastered() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Vibe, 0, Some(0.3));
        // Vibe encountered but not mastered → should suggest Vibe.
        assert_eq!(map.next_concept(), Some(PlatoConcept::Vibe));
    }

    #[test]
    fn test_concept_map_default() {
        let map: ConceptMap = Default::default();
        assert!(map.encountered.is_empty());
    }

    // -----------------------------------------------------------------------
    // Prerequisites tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_prerequisites_ordered_length() {
        assert_eq!(Prerequisites::ordered().len(), 10);
    }

    #[test]
    fn test_prerequisites_ordered_no_dupes() {
        let ordered = Prerequisites::ordered();
        let mut deduped = ordered.clone();
        deduped.sort();
        deduped.dedup();
        assert_eq!(ordered.len(), deduped.len());
    }

    #[test]
    fn test_prerequisites_vibe_has_none() {
        assert!(Prerequisites::of(&PlatoConcept::Vibe).is_empty());
    }

    #[test]
    fn test_prerequisites_conservation_requires_deadband() {
        let prereqs = Prerequisites::of(&PlatoConcept::Conservation);
        assert_eq!(prereqs, vec![PlatoConcept::Deadband]);
    }

    #[test]
    fn test_prerequisites_topology_two_prerequisites() {
        let prereqs = Prerequisites::of(&PlatoConcept::Topology);
        assert_eq!(
            prereqs,
            vec![PlatoConcept::FibonacciGrowth, PlatoConcept::Conservation]
        );
    }

    // -----------------------------------------------------------------------
    // PlatoEvent tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_plato_event_vibe_changed() {
        let ev = PlatoEvent::VibeChanged {
            old: 0.3,
            new: 0.8,
        };
        match ev {
            PlatoEvent::VibeChanged { old, new } => {
                assert!((old - 0.3).abs() < 1e-9);
                assert!((new - 0.8).abs() < 1e-9);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_plato_event_signal_received() {
        let ev = PlatoEvent::SignalReceived {
            source: "tower".into(),
            strength: 0.9,
        };
        match ev {
            PlatoEvent::SignalReceived { source, strength } => {
                assert_eq!(source, "tower");
                assert!((strength - 0.9).abs() < 1e-9);
            }
            _ => panic!("wrong variant"),
        }
    }

    // -----------------------------------------------------------------------
    // GameEvent tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_game_event_weather_changed() {
        let ev = GameEvent::WeatherChanged {
            to: "rainy".into(),
        };
        match ev {
            GameEvent::WeatherChanged { to } => assert_eq!(to, "rainy"),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_game_event_achievement_unlocked() {
        let ev = GameEvent::AchievementUnlocked {
            badge: "Gold Star".into(),
        };
        match ev {
            GameEvent::AchievementUnlocked { badge } => assert_eq!(badge, "Gold Star"),
            _ => panic!("wrong variant"),
        }
    }

    // -----------------------------------------------------------------------
    // BridgeTranslator tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_vibe_to_music() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::VibeChanged {
            old: 0.2,
            new: 0.8,
        };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::MusicChanged { mood } => assert_eq!(mood, "joyful"),
            _ => panic!("expected MusicChanged, got {game:?}"),
        }
    }

    #[test]
    fn test_conservation_to_achievement() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::ConservationChecked { error: 0.03 };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::AchievementUnlocked { badge } => assert_eq!(badge, "Perfect Balance"),
            _ => panic!("expected AchievementUnlocked, got {game:?}"),
        }
    }

    #[test]
    fn test_room_state_to_structure() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::RoomStateChanged {
            room: "kitchen".into(),
            from: "messy".into(),
            to: "clean".into(),
        };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::StructureBuilt { name } => assert_eq!(name, "kitchen [messy → clean]"),
            _ => panic!("expected StructureBuilt, got {game:?}"),
        }
    }

    #[test]
    fn test_agent_predicted_to_quest() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::AgentPredicted { accuracy: 0.85 };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::QuestProgress { quest, percent } => {
                assert_eq!(quest, "Prediction Master");
                assert!((percent - 85.0).abs() < 1e-9);
            }
            _ => panic!("expected QuestProgress, got {game:?}"),
        }
    }

    #[test]
    fn test_signal_received_to_weather() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::SignalReceived {
            source: "radio".into(),
            strength: 0.9,
        };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::WeatherChanged { to } => assert_eq!(to, "radio:stormy"),
            _ => panic!("expected WeatherChanged, got {game:?}"),
        }
    }

    #[test]
    fn test_signal_weak_to_clear_weather() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::SignalReceived {
            source: "beacon".into(),
            strength: 0.1,
        };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::WeatherChanged { to } => assert_eq!(to, "beacon:clear"),
            _ => panic!("expected WeatherChanged, got {game:?}"),
        }
    }

    #[test]
    fn test_ambient_vibe_to_calm_mood() {
        let translator = BridgeTranslator;
        let plato = PlatoEvent::VibeChanged {
            old: 0.3,
            new: 0.55,
        };
        let game = translator.plato_to_game_event(&plato);
        match game {
            GameEvent::MusicChanged { mood } => assert_eq!(mood, "calm"),
            _ => panic!("expected MusicChanged, got {game:?}"),
        }
    }

    #[test]
    fn test_game_to_plato_concept_trade() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("trade 3 apples"),
            Some(PlatoConcept::Conservation)
        );
    }

    #[test]
    fn test_game_to_plato_concept_predict() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("predict raindrop position"),
            Some(PlatoConcept::JEPA)
        );
    }

    #[test]
    fn test_game_to_plato_concept_vibe() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("change room mood"),
            Some(PlatoConcept::Vibe)
        );
    }

    #[test]
    fn test_game_to_plato_concept_unknown() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("jump over the fence"),
            None
        );
    }

    #[test]
    fn test_game_to_plato_concept_chain() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("build a chain of crystals"),
            Some(PlatoConcept::SignalChain)
        );
    }

    #[test]
    fn test_game_to_plato_concept_spiral() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("watch the spiral growth"),
            Some(PlatoConcept::FibonacciGrowth)
        );
    }

    #[test]
    fn test_game_to_plato_concept_whisper() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("hear the whisper"),
            Some(PlatoConcept::Murmur)
        );
    }

    #[test]
    fn test_game_to_plato_concept_reshape() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("reshape the room"),
            Some(PlatoConcept::Topology)
        );
    }

    #[test]
    fn test_game_to_plato_concept_quiet() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("stay in the quiet zone"),
            Some(PlatoConcept::Deadband)
        );
    }

    #[test]
    fn test_game_to_plato_concept_distill() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("distill the flower essence"),
            Some(PlatoConcept::Distillation)
        );
    }

    #[test]
    fn test_game_to_plato_concept_grow_room() {
        let translator = BridgeTranslator;
        assert_eq!(
            translator.game_to_plato_concept("grow the room bigger"),
            Some(PlatoConcept::RoomLifecycle)
        );
    }

    // -----------------------------------------------------------------------
    // Serde round-trip tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_serde_plato_concept_roundtrip() {
        let original = PlatoConcept::Conservation;
        let json = serde_json::to_string(&original).unwrap();
        let restored: PlatoConcept = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn test_serde_plato_event_roundtrip() {
        let original = PlatoEvent::VibeChanged {
            old: 0.2,
            new: 0.9,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PlatoEvent = serde_json::from_str(&json).unwrap();
        match (&original, &restored) {
            (PlatoEvent::VibeChanged { old: a, new: b }, PlatoEvent::VibeChanged { old: c, new: d }) => {
                let _ = a;
                assert!((a - c).abs() < 1e-9);
                assert!((b - d).abs() < 1e-9);
            }
            _ => panic!("variant mismatch after round-trip"),
        }
    }

    #[test]
    fn test_serde_game_event_roundtrip() {
        let original = GameEvent::PetEvolved {
            from: "egg".into(),
            to: "dragon".into(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GameEvent = serde_json::from_str(&json).unwrap();
        match (&original, &restored) {
            (GameEvent::PetEvolved { from: a, to: b }, GameEvent::PetEvolved { from: c, to: d }) => {
                assert_eq!(a, c);
                assert_eq!(b, d);
            }
            _ => panic!("variant mismatch after round-trip"),
        }
    }

    #[test]
    fn test_serde_concept_map_roundtrip() {
        let mut map = ConceptMap::new();
        map.encounter(PlatoConcept::Vibe, 10, Some(0.95));
        map.encounter(PlatoConcept::Deadband, 20, None);
        let json = serde_json::to_string(&map).unwrap();
        let restored: ConceptMap = serde_json::from_str(&json).unwrap();
        assert_eq!(
            map.mastery_level(&PlatoConcept::Vibe),
            restored.mastery_level(&PlatoConcept::Vibe)
        );
        assert_eq!(
            map.encountered.len(),
            restored.encountered.len()
        );
    }
}
