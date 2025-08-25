mod attribute;
mod constraint;

pub use attribute::*;
pub use constraint::*;

use bevy::platform::collections::HashMap;

#[derive(Debug, Default)]
pub struct AttributeSet {
    attributes: HashMap<String, Attribute>,
    constraints: Vec<AttributeConstraint>,
}

impl AttributeSet {
    pub fn insert_constraint(&mut self, constraint: AttributeConstraint) {
        if !self.constraints.contains(&constraint) {
            self.constraints.push(constraint);
        }
    }

    pub fn insert_attribute(&mut self, attribute: Attribute) {
        self.attributes
            .insert(attribute.attribute_name.clone(), attribute);
    }

    ///在实际更新属性之前，将所要改变的值更改为合理的值
    fn on_before_update_attribute_current_value(
        &mut self,
        attribute_name: &str,
        value: f32,
        container: &AttributeConstraintProcessorContainer,
    ) -> f32 {
        let index = match self
            .constraints
            .iter()
            .position(|v| v.target_attribute_name == attribute_name)
        {
            Some(index) => index,
            None => {
                return value;
            }
        };

        let constraint = &self.constraints[index];

        if let Some(processor) = container.get_processor(&constraint.constraint_name) {
            if let Some(attribute) = self.attributes.get_mut(&constraint.attribute_name) {
                processor.on_before_update_attribute_current_value(attribute, value, constraint)
            } else {
                value
            }
        } else {
            value
        }
    }

    ///在更新属性之后，对被影响的属性进行更新
    fn on_after_update_attribute_current_value(
        &mut self,
        attribute_name: &str,
        value: f32,
        container: &AttributeConstraintProcessorContainer,
    ) {
        for constraint in self
            .constraints
            .iter()
            .filter(|v| v.attribute_name == attribute_name)
        {
            if let Some(processor) = container.get_processor(&constraint.constraint_name) {
                if let Some(attribute) = self.attributes.get_mut(&constraint.target_attribute_name)
                {
                    processor.on_after_update_attribute_current_value(attribute, value, constraint);
                }
            }
        }
    }

    fn update_attribute_current_value(
        &mut self,
        attribute_name: &str,
        value: f32,
        container: &AttributeConstraintProcessorContainer,
    ) {
        let final_value =
            self.on_before_update_attribute_current_value(attribute_name, value, container);

        if final_value != value {
            if let Some(attribute) = self.attributes.get_mut(attribute_name) {
                attribute.set_current_value(final_value);
            }
        }

        if let Some(current_value) = self
            .attributes
            .get(attribute_name)
            .map(|v| v.get_current_value())
        {
            self.on_after_update_attribute_current_value(attribute_name, current_value, container);
        }
    }

    pub fn update_attribute_base_value(
        &mut self,
        attribute_name: &str,
        value: f32,
        container: &AttributeConstraintProcessorContainer,
    ) {
        let Some(current_value) = self
            .attributes
            .get(attribute_name)
            .map(|v| v.get_current_value())
        else {
            return;
        };

        self.attributes
            .get_mut(attribute_name)
            .unwrap()
            .set_base_value(value);

        let final_value = self
            .attributes
            .get(attribute_name)
            .unwrap()
            .get_current_value();

        if final_value != current_value {
            self.update_attribute_current_value(attribute_name, final_value, container);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{
        Attribute, AttributeConstraint, AttributeConstraintProcessorContainer, AttributeSet,
    };

    fn new_attribute_set() -> AttributeSet {
        let mut set = AttributeSet::default();

        let mut power = Attribute::default();
        power.attribute_name = "power".to_string();
        power.set_base_value(10.0);
        set.insert_attribute(power);

        let mut max_power = Attribute::default();
        max_power.attribute_name = "max_power".to_string();
        max_power.set_base_value(30.0);
        set.insert_attribute(max_power);

        set.insert_constraint(AttributeConstraint {
            attribute_name: "max_power".to_string(),
            target_attribute_name: "power".to_string(),
            constraint_name: "max".to_string(),
        });

        set
    }

    #[test]
    fn test_attribute_set() {
        let mut set = new_attribute_set();

        let container = AttributeConstraintProcessorContainer::new();

        set.update_attribute_base_value("power", 40.0, &container);

        assert_eq!(
            30.0,
            set.attributes.get("power").unwrap().get_current_value()
        );

        set.update_attribute_base_value("max_power", 35.0, &container);

        assert_eq!(
            35.0,
            set.attributes.get("power").unwrap().get_current_value()
        );
    }
}
