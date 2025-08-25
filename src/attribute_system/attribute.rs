use bevy::platform::collections::HashMap;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

///属性
#[derive(Debug)]
pub struct Attribute {
    base_value: f32,
    current_value: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub attribute_name: String,
    pub display_name: String,
    modifiers: Vec<AttributeModifier>,
    unique_modifiers: HashMap<String, AttributeModifier>,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            base_value: 0.0,
            current_value: 0.0,
            min_value: f32::MIN,
            max_value: f32::MAX,
            attribute_name: Default::default(),
            display_name: Default::default(),
            modifiers: Default::default(),
            unique_modifiers: Default::default(),
        }
    }
}

impl Attribute {
    pub fn set_base_value(&mut self, v: f32) {
        self.base_value = v;
        self.recalculate_current_value();
    }

    pub fn set_current_value(&mut self, current_value: f32) {
        self.current_value = current_value.clamp(self.min_value, self.max_value);
    }

    pub fn get_base_value(&self) -> f32 {
        self.base_value
    }

    pub fn get_current_value(&self) -> f32 {
        self.current_value
    }

    pub fn add_modifier(&mut self, modifier: &AttributeModifier) {
        if !self.modifiers.contains(modifier) {
            self.modifiers.push(modifier.clone());
            self.recalculate_current_value();
        }
    }

    pub fn insert_modifier(&mut self, modifier: &AttributeModifier) {
        self.unique_modifiers
            .insert(modifier.source_id.clone(), modifier.clone());
        self.recalculate_current_value();
    }

    pub fn remove_modifier(&mut self, modifier: &AttributeModifier) {
        if let Some(index) = self.modifiers.iter().position(|v| v.uuid == modifier.uuid) {
            self.modifiers.remove(index);
            self.recalculate_current_value();
        }
    }

    pub fn recalculate_current_value(&mut self) {
        self.current_value = self.base_value;
        let mut absolute_modifiers = vec![];
        let mut percentage_modifiers = vec![];
        let mut override_modifiers = vec![];

        for modifier in self.modifiers.iter().chain(self.unique_modifiers.values()) {
            match modifier.operation {
                ModifierOperation::Absolute => {
                    absolute_modifiers.push(modifier.clone());
                }
                ModifierOperation::Percentage => {
                    percentage_modifiers.push(modifier.clone());
                }
                ModifierOperation::Override => {
                    override_modifiers.push(modifier.clone());
                }
            }
        }

        for modifier in absolute_modifiers.clone().iter() {
            self.apply_modifier(modifier);
        }

        for modifier in percentage_modifiers.clone().iter() {
            self.apply_modifier(modifier);
        }

        if !override_modifiers.is_empty() {
            override_modifiers.sort_by(|a, b| b.priority.cmp(&a.priority));

            self.apply_modifier(&override_modifiers[0]);
        }

        let current_value = self.current_value;

        self.set_current_value(current_value);
    }

    fn apply_modifier(&mut self, modifier: &AttributeModifier) {
        match modifier.operation {
            ModifierOperation::Absolute => {
                self.current_value += modifier.value;
            }
            ModifierOperation::Override => {
                self.current_value = modifier.value;
            }
            ModifierOperation::Percentage => {
                self.current_value += self.base_value * modifier.value;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModifierOperation {
    Absolute,
    Override,
    Percentage,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModifierPriority(usize);

impl ModifierPriority {
    pub const NORMAL: ModifierPriority = ModifierPriority(50);
    pub const LOW: ModifierPriority = ModifierPriority(25);
}

impl Default for ModifierPriority {
    fn default() -> Self {
        ModifierPriority::NORMAL
    }
}

///属性更改器
#[derive(Debug, Clone)]
pub struct AttributeModifier {
    pub operation: ModifierOperation,
    pub value: f32,
    pub uuid: Uuid,
    pub source_id: String,
    pub attribute_name: String,
    pub priority: ModifierPriority,
}

impl AttributeModifier {
    pub fn empty() -> Self {
        Self {
            operation: ModifierOperation::Absolute,
            value: 0.0,
            uuid: Uuid::new_v4(),
            attribute_name: "".into(),
            source_id: "".into(),
            priority: Default::default(),
        }
    }

    pub fn new_override_modifier(source_id: &str, value: f32) -> Self {
        let mut modifier = AttributeModifier::empty();
        modifier.source_id = source_id.to_string();
        modifier.operation = ModifierOperation::Override;
        modifier.value = value;

        modifier
    }
}

impl PartialEq for AttributeModifier {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Eq for AttributeModifier {}

impl Hash for AttributeModifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_attribute() {
        use super::*;

        let mut absolute_modifier = AttributeModifier::empty();
        absolute_modifier.operation = ModifierOperation::Absolute;
        absolute_modifier.value = 20.0;

        let mut attribute = Attribute::default();
        attribute.set_base_value(0.0);
        assert_eq!(0.0, attribute.get_base_value());

        attribute.add_modifier(&absolute_modifier);
        assert_eq!(20.0, attribute.get_current_value());

        attribute.remove_modifier(&absolute_modifier);
        assert_eq!(0.0, attribute.get_current_value());

        let mut percentage_modifier = AttributeModifier::empty();
        percentage_modifier.operation = ModifierOperation::Percentage;
        percentage_modifier.value = 0.5;

        attribute.set_base_value(10.0);
        attribute.add_modifier(&percentage_modifier);

        assert_eq!(15.0, attribute.get_current_value());
        attribute.remove_modifier(&percentage_modifier);

        let mut override_modifier1 = AttributeModifier::empty();
        override_modifier1.operation = ModifierOperation::Override;
        override_modifier1.value = 10.0;
        override_modifier1.priority = ModifierPriority::LOW;

        attribute.add_modifier(&override_modifier1);

        assert_eq!(10.0, attribute.get_current_value());

        let mut override_modifier2 = AttributeModifier::empty();
        override_modifier2.operation = ModifierOperation::Override;
        override_modifier2.value = 20.0;

        attribute.add_modifier(&override_modifier2);

        assert_eq!(20.0, attribute.get_current_value());
    }
}
