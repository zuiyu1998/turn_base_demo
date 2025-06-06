use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Hash)]
pub enum ModifierOperation {
    Absolute,
    Override,
    Percentage,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AttributeModifier {
    pub source_uid: String,
    value: f32,
    operation: ModifierOperation,
}

impl AttributeModifier {
    pub fn new(value: f32, operation: ModifierOperation, source_uid: String) -> Self {
        Self {
            value,
            operation,
            source_uid,
        }
    }

    pub fn apply(&self, attr: &mut Attribute) {
        match self.operation {
            ModifierOperation::Absolute => attr.current_value += self.value,
            ModifierOperation::Override => attr.current_value = self.value,
            ModifierOperation::Percentage => attr.current_value = attr.base_value * self.value,
        }
    }
}

/// 属性
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attribute {
    /// 属性的唯一标识名
    pub attribute_name: String,
    /// 属性的显示名称
    pub display_name: String,
    /// 属性的详细描述
    pub description: String,
    /// 属性允许的最小值
    pub min_value: f32,
    /// 属性允许的最大值
    pub max_value: f32,
    /// 属性值是否可以为负
    pub can_be_negative: bool,
    /// 属性的基础值
    base_value: f32,
    /// 属性的当前值
    current_value: f32,
    modifier_list: Vec<AttributeModifier>,
}

impl Attribute {
    pub fn remove_modifier(&mut self, value: &AttributeModifier) {
        self.modifier_list.retain(|modifier| modifier == value);

        self.recalculate_current_value();
    }

    pub fn add_modifier(&mut self, modifier: AttributeModifier) {
        if !self.modifier_list.contains(&modifier) {
            self.modifier_list.push(modifier);

            self.recalculate_current_value();
        }
    }

    pub fn set_base_value(&mut self, value: f32) {
        self.base_value = value;
        self.recalculate_current_value();
    }

    fn recalculate_current_value(&mut self) {
        let mut absolute_modifier_list = vec![];
        let mut percentage_modifier_list = vec![];
        let mut override_modifier = None;

        for modifier in self.modifier_list.iter() {
            match modifier.operation {
                ModifierOperation::Absolute => {
                    absolute_modifier_list.push(modifier.clone());
                }
                ModifierOperation::Percentage => {
                    percentage_modifier_list.push(modifier.clone());
                }
                ModifierOperation::Override => override_modifier = Some(modifier.clone()),
            }
        }

        for modifier in absolute_modifier_list.iter() {
            modifier.apply(self);
        }

        for modifier in percentage_modifier_list.iter() {
            modifier.apply(self);
        }

        if let Some(modifier) = override_modifier {
            modifier.apply(self);
        }

        let mut final_value = self.current_value;

        if !self.can_be_negative && final_value < 0.0 {
            final_value = 0.0
        }

        final_value = final_value.clamp(self.min_value, self.max_value);
        self.current_value = final_value;
    }

    pub fn get_base_value(&self) -> f32 {
        self.base_value
    }

    pub fn get_current_value(&self) -> f32 {
        self.current_value
    }
}

//属性集合
pub trait AttributeSet {
    // 初始化属性之间的依赖
    fn initialize_attribute_dependencies(&mut self) {}

    // 单一属性更新之前的回调
    fn before_base_value_change(&mut self, _attribute_name: &str, _old_v: f32, new_v: f32) -> f32 {
        new_v
    }

    // 单一属性更新之后的回调
    fn after_base_value_change(&mut self, _attribute_name: &str, _old_v: f32, _new_v: f32) {}

    // 单一属性更新之前的回调
    fn before_current_value_change(
        &mut self,
        _attribute_name: &str,
        _old_v: f32,
        new_v: f32,
    ) -> f32 {
        new_v
    }

    // 单一属性更新之后的回调
    fn after_current_value_change(&mut self, _attribute_name: &str, _old_v: f32, _new_v: f32) {}
}
