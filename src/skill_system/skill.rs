use bevy::platform::collections::HashMap;

pub struct Skill {
    pub skill_name: String,
    pub display_name: String,
    pub effcts: Vec<SkillEffect>,
}

pub struct SkillComponent {}

pub enum SkillError {}

pub enum SkillProperty {
    Number(f32),
}

pub struct SkillEffect {
    pub skill_effect_name: String,
    pub payload: HashMap<String, SkillProperty>,
}

pub struct SkillEffectResult {
    pub skill_effect_name: String,
    pub payload: HashMap<String, SkillProperty>,
}

pub struct SkillContext {
    pub results: HashMap<String, SkillEffectResult>,
}

pub trait SkillEffectProcessor {
    fn skill_effct_name(&self) -> String;

    fn process(
        &self,
        caster: &SkillComponent,
        target: &SkillComponent,
        skill_effct: &SkillEffect,
        context: &mut SkillContext,
    ) -> Result<(), SkillError>;
}
