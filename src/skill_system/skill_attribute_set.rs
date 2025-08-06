use crate::{AttributeSet, AttributeSetComponent};

#[derive(Default)]
pub struct SkillAttributeSet(AttributeSet);

impl SkillAttributeSet {
    pub const HEALTH: &str = "health";
    pub const MAX_HEALTH: &str = "max_health";

    pub const POWER: &str = "power";
    pub const MAX_POWER: &str = "max_power";
}

pub trait SkillAttributeSetComponent: AttributeSetComponent {
    fn get_target_attribute_current_value_with_max_attribute_name(
        &self,
        max_attribute_name: &str,
        value: f32,
    ) -> f32 {
        if let Some(attribute) = self.get_attribute_set().get(max_attribute_name) {
            let max_health = attribute.get_current_value();
            value.clamp(0.0, max_health)
        } else {
            value
        }
    }
}

impl<T: AttributeSetComponent> SkillAttributeSetComponent for T {}

impl AttributeSetComponent for SkillAttributeSet {
    fn get_attribute_set(&self) -> &AttributeSet {
        &self.0
    }

    fn get_attribute_set_mut(&mut self) -> &mut AttributeSet {
        &mut self.0
    }

    fn on_before_update_attribute_current_value(&self, attribute_name: &str, value: f32) -> f32 {
        match attribute_name {
            //血量值更改时不能超过血量最大值
            Self::HEALTH => {
                if let Some(attribute) = self.get_attribute_set().get(Self::MAX_HEALTH) {
                    let max_health = attribute.get_current_value();
                    value.clamp(0.0, max_health)
                } else {
                    value
                }
            }
            //力量值更改时不能超过血量最大值
            Self::POWER => {
                if let Some(attribute) = self.get_attribute_set().get(Self::MAX_POWER) {
                    let max_health = attribute.get_current_value();
                    value.clamp(0.0, max_health)
                } else {
                    value
                }
            }
            _ => value,
        }
    }

    fn on_after_update_attribute_current_value(&mut self, attribute_name: &str, value: f32) {
        match attribute_name {
            //血量最大值更改时,血量不能超过血量最大值
            Self::MAX_HEALTH => {
                if let Some(attribute) = self.get_attribute_set_mut().get_mut(Self::HEALTH) {
                    let health = attribute.get_current_value();
                    let final_value = health.min(value);
                    attribute.set_current_value(final_value);
                }
            }

            Self::MAX_POWER => {
                if let Some(attribute) = self.get_attribute_set_mut().get_mut(Self::POWER) {
                    let power = attribute.get_current_value();

                    let final_value = power.min(value);
                    attribute.set_current_value(final_value);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Attribute, AttributeSetComponent, SkillAttributeSet};

    fn get_test_skill_attribute_set() -> SkillAttributeSet {
        let mut set = SkillAttributeSet::default();

        let mut health = Attribute::default();
        health.attribute_name = SkillAttributeSet::HEALTH.to_string();

        set.get_attribute_set_mut()
            .insert(SkillAttributeSet::HEALTH.to_string(), health);

        let mut max_health = Attribute::default();
        max_health.attribute_name = SkillAttributeSet::MAX_HEALTH.to_string();

        max_health.set_base_value(10.0);

        set.get_attribute_set_mut()
            .insert(SkillAttributeSet::MAX_HEALTH.to_string(), max_health);

        set
    }

    #[test]
    fn test_skill_attribute_set() {
        let mut set = get_test_skill_attribute_set();

        set.update_attribute_current_value(SkillAttributeSet::HEALTH, 20.0);

        let current_value = set
            .get_attribute_set()
            .get(SkillAttributeSet::HEALTH)
            .unwrap()
            .get_current_value();

        assert_eq!(current_value, 10.0);

        set.update_attribute_base_value(SkillAttributeSet::HEALTH, 20.0);

        let current_value = set
            .get_attribute_set()
            .get(SkillAttributeSet::HEALTH)
            .unwrap()
            .get_current_value();

        assert_eq!(current_value, 10.0);
    }
}
