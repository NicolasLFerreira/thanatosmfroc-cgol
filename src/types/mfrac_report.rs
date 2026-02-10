pub type MfracStatus = Option<MfracOutcome>;

pub enum MfracOutcome {
    Collision(u128),
    Termination(MfracTerminationReason),
}

pub enum MfracTerminationReason {
    GenerationLimitExceeded(u32),

    // Canonical hash stabilizes (p=1 under canonicalization).
    // Includes true still-life AND symmetric oscillators where all phases
    // are rotations/reflections of each other (e.g., block, blinker)
    CanonicalStabilisation,

    // Canonical hash cycles with p>1 under canonicalization.
    // Includes gliders/spaceships and true asymmetric oscillators whose
    // phases are not related by rotation/reflection (e.g., toad)
    CanonicalOscillation,
}
