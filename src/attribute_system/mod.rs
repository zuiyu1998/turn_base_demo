mod attribute;

pub use attribute::*;
use bevy::{
    platform::collections::HashMap,
    prelude::{Deref, DerefMut},
};

#[derive(Debug, Deref, DerefMut, Default)]
pub struct AttributeSet(HashMap<String, Attribute>);

///描述属性之间的相互影响
pub trait AttributeSetComponent: 'static + Sync + Send {
    fn get_attribute_set(&self) -> &AttributeSet;

    fn get_attribute_set_mut(&mut self) -> &mut AttributeSet;

    ///在实际更新属性之前，将所要改变的值更改为合理的值
    fn on_before_update_attribute_current_value(&self, _attribute_name: &str, value: f32) -> f32 {
        value
    }

    ///在更新属性之后，对被属性影响的值进行更新
    fn on_after_update_attribute_current_value(&mut self, _attribute_name: &str, _value: f32) {}

    fn update_attribute_current_value(&mut self, attribute_name: &str, value: f32) {
        let final_value = self.on_before_update_attribute_current_value(attribute_name, value);

        if final_value == value {
            return;
        }

        let attribute_set = self.get_attribute_set_mut();

        if let Some(attribute) = attribute_set.get_mut(attribute_name) {
            attribute.set_current_value(final_value);
        }

        if let Some(current_value) = self
            .get_attribute_set()
            .get(attribute_name)
            .map(|v| v.get_current_value())
        {
            self.on_after_update_attribute_current_value(attribute_name, current_value);
        }
    }

    fn update_attribute_base_value(&mut self, attribute_name: &str, value: f32) {
        let attribute_set = self.get_attribute_set_mut();

        let Some(current_value) = attribute_set
            .get(attribute_name)
            .map(|v| v.get_current_value())
        else {
            return;
        };

        attribute_set
            .get_mut(attribute_name)
            .unwrap()
            .set_base_value(value);

        let final_value = attribute_set
            .get(attribute_name)
            .unwrap()
            .get_current_value();

        if final_value != current_value {
            self.update_attribute_current_value(attribute_name, final_value);
        }
    }
}
