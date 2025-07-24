pub mod fade;
pub mod dissolve;
pub mod wipe;

use crate::{
    error::Result,
    parameters::{ParameterManager, ParameterValue},
    traits::*,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Base implementation for transitions
pub struct BaseTransition {
    pub id: Uuid,
    pub name: String,
    pub parameters: ParameterManager,
    pub duration: f64,
    pub requires_gpu: bool,
}

impl BaseTransition {
    pub fn new(name: impl Into<String>, requires_gpu: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            parameters: ParameterManager::new(),
            duration: 1.0, // Default 1 second
            requires_gpu,
        }
    }
}

/// Registry for transition effects
pub struct TransitionRegistry {
    factories: HashMap<String, Box<dyn Fn() -> Box<dyn Transition> + Send + Sync>>,
}

impl TransitionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: HashMap::new(),
        };
        
        // Register built-in transitions
        registry.register_builtin_transitions();
        
        registry
    }
    
    fn register_builtin_transitions(&mut self) {
        self.register("fade", || Box::new(fade::FadeTransition::new()));
        self.register("dissolve", || Box::new(dissolve::DissolveTransition::new()));
        self.register("wipe_left", || Box::new(wipe::WipeTransition::new_left()));
        self.register("wipe_right", || Box::new(wipe::WipeTransition::new_right()));
        self.register("wipe_up", || Box::new(wipe::WipeTransition::new_up()));
        self.register("wipe_down", || Box::new(wipe::WipeTransition::new_down()));
    }
    
    pub fn register<F>(&mut self, name: &str, factory: F)
    where
        F: Fn() -> Box<dyn Transition> + Send + Sync + 'static,
    {
        self.factories.insert(name.to_string(), Box::new(factory));
    }
    
    pub fn create(&self, name: &str) -> Option<Box<dyn Transition>> {
        self.factories.get(name).map(|factory| factory())
    }
    
    pub fn list_transitions(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

impl Default for TransitionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating transitions
pub struct DefaultTransitionFactory {
    registry: TransitionRegistry,
}

impl DefaultTransitionFactory {
    pub fn new() -> Self {
        Self {
            registry: TransitionRegistry::new(),
        }
    }
    
    pub fn create(&self, name: &str) -> Result<Box<dyn Transition>> {
        self.registry
            .create(name)
            .ok_or_else(|| crate::error::EffectError::Other(
                anyhow::anyhow!("Transition '{}' not found", name)
            ))
    }
    
    pub fn available_transitions(&self) -> Vec<String> {
        self.registry.list_transitions()
    }
}