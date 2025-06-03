use serde::{Deserialize, Serialize};

/// 属性
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttributeBase {
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
    pub can_be_negative: f32,
    /// 属性的基础值
    base_value: f32,
    /// 属性的当前值
    current_value: f32,
}

impl AttributeBase {
    pub fn get_base_value(&self) -> f32 {
        self.base_value
    }

    pub fn get_current_value(&self) -> f32 {
        self.current_value
    }
}
