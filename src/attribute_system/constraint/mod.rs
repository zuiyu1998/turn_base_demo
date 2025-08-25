use bevy::platform::collections::HashMap;

use crate::Attribute;

pub struct AttributeConstraintProcessorContainer(
    HashMap<String, Box<dyn AttributeConstraintProcessor>>,
);

impl AttributeConstraintProcessorContainer {
    pub fn new() -> Self {
        let mut container = Self::empty();

        container.add_processor("max", MaxAttributeConstraintProcessor);

        container
    }

    pub fn empty() -> Self {
        Self(HashMap::default())
    }

    pub fn add_processor<T: AttributeConstraintProcessor>(&mut self, name: &str, value: T) {
        self.0.insert(name.to_string(), Box::new(value));
    }

    pub fn get_processor(&self, name: &str) -> Option<&dyn AttributeConstraintProcessor> {
        self.0.get(name).map(|v| v.as_ref())
    }
}

impl Default for AttributeConstraintProcessorContainer {
    fn default() -> Self {
        Self::new()
    }
}

///属性约束处理器
pub trait AttributeConstraintProcessor: 'static + Sync + Send {
    //在更新属性之后，对被影响的属性进行更新
    fn on_after_update_attribute_current_value(
        &self,
        attribute: &mut Attribute,
        value: f32,
        constraint: &AttributeConstraint,
    );

    //在更新属性之前，重新计算属性要设置的值
    fn on_before_update_attribute_current_value(
        &self,
        _attribute: &mut Attribute,
        value: f32,
        _constraint: &AttributeConstraint,
    ) -> f32 {
        value
    }
}

pub struct MaxAttributeConstraintProcessor;

impl AttributeConstraintProcessor for MaxAttributeConstraintProcessor {
    fn on_after_update_attribute_current_value(
        &self,
        attribute: &mut Attribute,
        value: f32,
        _constraint: &AttributeConstraint,
    ) {
        attribute.recalculate_current_value();
        let cul_value = attribute.get_current_value();
        let value = cul_value.clamp(0.0, value);
        attribute.set_current_value(value);
    }

    fn on_before_update_attribute_current_value(
        &self,
        attribute: &mut Attribute,
        value: f32,
        _constraint: &AttributeConstraint,
    ) -> f32 {
        let cul_value = attribute.get_current_value();

        value.min(cul_value)
    }
}

///属性约束
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeConstraint {
    pub attribute_name: String,
    pub target_attribute_name: String,
    pub constraint_name: String,
}
