#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Phase {
    Untap,
    Upkeep,
    Draw,
    PreCombatMain,
    Combat,
    PostCombatMain,
    End,
    Cleanup,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SubPhase {
    Beginning,
    End,
}

#[derive(Debug)]
pub struct PhaseManager {
    current_phase: Phase,
    current_subphase: SubPhase,
}

#[allow(dead_code)]
impl PhaseManager {
    pub fn new() -> Self {
        Self {
            current_phase: Phase::Untap,
            current_subphase: SubPhase::Beginning,
        }
    }

    pub fn process_phase(&mut self) {
        self.resolve_phase_triggers(&self.current_phase, &self.current_subphase);
        self.next()
    }

    pub fn next(&mut self) {
        match self.current_subphase {
            SubPhase::Beginning => {
                self.current_subphase = SubPhase::End;
            }
            SubPhase::End => {
                self.current_phase = match self.current_phase {
                    Phase::Untap => Phase::Upkeep,
                    Phase::Upkeep => Phase::Draw,
                    Phase::Draw => Phase::PreCombatMain,
                    Phase::PreCombatMain => Phase::Combat,
                    Phase::Combat => Phase::PostCombatMain,
                    Phase::PostCombatMain => Phase::End,
                    Phase::End => Phase::Cleanup,
                    Phase::Cleanup => Phase::Untap,
                };
                self.current_subphase = SubPhase::Beginning;
            }
        }
    }

    pub fn get_phase(&self) -> (Phase, SubPhase) {
        (self.current_phase, self.current_subphase)
    }

    pub fn is_phase(&self, phase: Phase, subphase: SubPhase) -> bool {
        self.current_phase == phase && self.current_subphase == subphase
    }

    pub fn resolve_phase_triggers(&self, phase: &Phase, subphase: &SubPhase) {
        println!(
            "resolve_phase_triggers is currently unimplemented. [Current phase {:?} - {:?}",
            phase, subphase
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_phase() {
        let manager = PhaseManager::new();
        assert_eq!(manager.get_phase(), (Phase::Untap, SubPhase::Beginning));
    }

    #[test]
    fn test_next_phase() {
        let mut manager = PhaseManager::new();

        manager.next();
        assert_eq!(manager.get_phase(), (Phase::Untap, SubPhase::End));

        manager.next();
        assert_eq!(manager.get_phase(), (Phase::Upkeep, SubPhase::Beginning));
    }

    #[test]
    fn test_full_phase_cycle() {
        let mut manager = PhaseManager::new();
        let expected_sequence = vec![
            (Phase::Untap, SubPhase::Beginning),
            (Phase::Untap, SubPhase::End),
            (Phase::Upkeep, SubPhase::Beginning),
            (Phase::Upkeep, SubPhase::End),
            (Phase::Draw, SubPhase::Beginning),
            (Phase::Draw, SubPhase::End),
            (Phase::PreCombatMain, SubPhase::Beginning),
            (Phase::PreCombatMain, SubPhase::End),
            (Phase::Combat, SubPhase::Beginning),
            (Phase::Combat, SubPhase::End),
            (Phase::PostCombatMain, SubPhase::Beginning),
            (Phase::PostCombatMain, SubPhase::End),
            (Phase::End, SubPhase::Beginning),
            (Phase::End, SubPhase::End),
            (Phase::Cleanup, SubPhase::Beginning),
            (Phase::Cleanup, SubPhase::End),
            (Phase::Untap, SubPhase::Beginning), // Back to start
        ];

        for expected in expected_sequence {
            assert_eq!(manager.get_phase(), expected);
            manager.next();
        }
    }

    #[test]
    fn test_process_phase_should_advance_subphase_then_phase() {
        let mut manager = PhaseManager::new();

        manager.process_phase();
        assert_eq!(manager.get_phase(), (Phase::Untap, SubPhase::End));

        manager.process_phase();
        assert_eq!(manager.get_phase(), (Phase::Upkeep, SubPhase::Beginning));
    }

    #[test]
    fn test_is_phase() {
        let manager = PhaseManager::new();
        assert!(manager.is_phase(Phase::Untap, SubPhase::Beginning));
        assert!(!manager.is_phase(Phase::Upkeep, SubPhase::Beginning));
    }
}
