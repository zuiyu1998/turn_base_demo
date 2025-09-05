use bevy::platform::collections::HashMap;
use rand::{Rng, rngs::ThreadRng};

pub struct RandNumberGenerator(ThreadRng);

impl Default for RandNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RandNumberGenerator {
    pub fn new() -> Self {
        Self(rand::rng())
    }
}

impl NumberGenerator for RandNumberGenerator {
    fn generate(&mut self, range: &NumberRange) -> usize {
        self.0.random_range(range.start..range.end)
    }
}

pub struct NumberGeneratorContainer(HashMap<String, Box<dyn NumberGenerator>>);

impl Default for NumberGeneratorContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl NumberGeneratorContainer {
    pub fn empty() -> Self {
        NumberGeneratorContainer(Default::default())
    }

    pub fn new() -> Self {
        let mut empty = Self::empty();
        empty.register("rand", RandNumberGenerator::new());

        empty
    }

    pub fn register<T: NumberGenerator>(&mut self, name: &str, value: T) {
        self.0.insert(name.to_string(), Box::new(value));
    }

    pub fn generate(&mut self, range: &NumberRange) -> Option<usize> {
        self.0
            .get_mut(&range.name)
            .map(|generator| generator.generate(range))
    }
}

pub trait NumberGenerator: 'static {
    fn generate(&mut self, range: &NumberRange) -> usize;
}

pub struct NumberRange {
    start: usize,
    end: usize,
    pub name: String,
}

impl NumberRange {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn set_end(&mut self, end: usize) {
        if end < self.start {
            self.start = end;
        }

        self.end = end;
    }

    pub fn set_start(&mut self, start: usize) {
        if start > self.end {
            self.end = start;
        }

        self.start = start;
    }
}
