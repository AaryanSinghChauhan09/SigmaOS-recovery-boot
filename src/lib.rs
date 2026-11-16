// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-recovery-boot: recovery boot component model.
//!
//! Real implementation: boots, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boot {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecoveryBootError {
    EmptyBootName,
    DuplicateBoot(String),
    EmptyEventSubject,
}

impl fmt::Display for RecoveryBootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBootName => write!(f, "boot name must not be empty"),
            Self::DuplicateBoot(id) => write!(f, "duplicate boot {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for RecoveryBootError {}

pub struct RecoveryBoot {
    pub name: String,
    boots: BTreeMap<String, Boot>,
    events: BTreeMap<String, Event>,
}

impl RecoveryBoot {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            boots: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_boot(&mut self, item: Boot) -> Result<(), RecoveryBootError> {
        if item.name.trim().is_empty() {
            return Err(RecoveryBootError::EmptyBootName);
        }
        let id = item.id.clone();
        if self.boots.contains_key(&id) {
            return Err(RecoveryBootError::DuplicateBoot(id.clone()));
        }
        self.boots.insert(id, item);
        Ok(())
    }

    pub fn boot_count(&self) -> usize {
        self.boots.len()
    }

    pub fn get_boot(&self, id: &str) -> Option<&Boot> {
        self.boots.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), RecoveryBootError> {
        if event.subject.trim().is_empty() {
            return Err(RecoveryBootError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_boot(id: &str, name: &str) -> Boot {
        Boot {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = RecoveryBoot::new("RecoveryBoot");
        assert_eq!(m.name, "RecoveryBoot");
        assert_eq!(m.boot_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_boot() {
        let mut m = RecoveryBoot::new("RecoveryBoot");
        m.add_boot(sample_boot("a1", "Alpha")).unwrap();
        assert_eq!(m.boot_count(), 1);
        assert!(m.get_boot("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = RecoveryBoot::new("RecoveryBoot");
        assert!(m.add_boot(sample_boot("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = RecoveryBoot::new("RecoveryBoot");
        m.add_boot(sample_boot("x", "One")).unwrap();
        assert!(m.add_boot(sample_boot("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = RecoveryBoot::new("RecoveryBoot");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
