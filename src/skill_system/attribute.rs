use crate::{Attribute, AttributeConstraint, AttributeSet};

pub struct AttributeUtils;

impl AttributeUtils {
    pub fn new_skill_attribute_set() -> AttributeSet {
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
}

#[cfg(test)]
mod test {}
