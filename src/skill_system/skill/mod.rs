mod processors;

pub use processors::*;

use bevy::platform::collections::HashMap;

use crate::AttributeSet;

pub struct Skill {
    pub skill_name: String,
    pub display_name: String,
    pub effcts: Vec<SkillEffect>,
}

pub struct SkillComponent {
    pub attribute_set: AttributeSet,
}

pub enum SkillError {}

pub enum SkillProperty {
    Number(f32),
}

pub struct SkillEffect {
    pub skill_effect_name: String,
    pub payload: HashMap<String, SkillProperty>,
}

pub struct SkillEffectResult {
    pub payload: HashMap<String, SkillProperty>,
}

#[derive(Default)]
pub struct SkillContext {
    pub results: HashMap<String, SkillEffectResult>,
}

pub trait SkillEffectProcessor: 'static + Sync + Send {
    fn name() -> String;

    fn process(
        &self,
        caster: &SkillComponent,
        target: &SkillComponent,
        skill_effect: &SkillEffect,
        context: &SkillContext,
    ) -> Result<SkillEffectResult, SkillError>;
}

pub trait ErasedSkillEffectProcessor: 'static + Sync + Send {
    fn skill_effect_name(&self) -> String;

    fn update_skill_context(
        &self,
        caster: &SkillComponent,
        target: &SkillComponent,
        skill_effect: &SkillEffect,
        context: &mut SkillContext,
    ) -> Result<(), SkillError>;
}

impl<T: SkillEffectProcessor> ErasedSkillEffectProcessor for T {
    fn skill_effect_name(&self) -> String {
        T::name()
    }

    fn update_skill_context(
        &self,
        caster: &SkillComponent,
        target: &SkillComponent,
        skill_effct: &SkillEffect,
        context: &mut SkillContext,
    ) -> Result<(), SkillError> {
        let result = self.process(caster, target, skill_effct, context)?;
        let skill_effect_name = self.skill_effect_name();

        context.results.insert(skill_effect_name, result);

        Ok(())
    }
}

pub struct SkillEffectProcessorContainer(HashMap<String, Box<dyn ErasedSkillEffectProcessor>>);

impl Default for SkillEffectProcessorContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillEffectProcessorContainer {
    pub fn new() -> Self {
        let mut container = Self::empty();
        container.register_skill_effect_processor(DamageSkillEffectProcessor);

        container
    }

    pub fn empty() -> Self {
        SkillEffectProcessorContainer(Default::default())
    }

    pub fn register_skill_effect_processor<T: SkillEffectProcessor>(&mut self, value: T) {
        self.0.insert(T::name(), Box::new(value));
    }
}

impl SkillEffectProcessorContainer {
    pub fn execute(
        &self,
        caster: &SkillComponent,
        target: &SkillComponent,
        skill: &Skill,
    ) -> Result<SkillContext, SkillError> {
        let mut skill_context = SkillContext::default();

        for skill_effect in skill.effcts.iter() {
            if let Some(processor) = self.0.get(&skill_effect.skill_effect_name) {
                processor.update_skill_context(caster, target, skill_effect, &mut skill_context)?;
            }
        }

        Ok(skill_context)
    }
}
