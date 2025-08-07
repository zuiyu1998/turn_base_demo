use super::{
    SkillComponent, SkillContext, SkillEffect, SkillEffectProcessor, SkillEffectResult, SkillError,
};

pub struct DamageSkillEffectProcessor;

impl SkillEffectProcessor for DamageSkillEffectProcessor {
    fn name() -> String {
        "damage_skill_effect".to_string()
    }

    fn process(
        &self,
        _caster: &SkillComponent,
        _target: &SkillComponent,
        _skill_effect: &SkillEffect,
        _context: &SkillContext,
    ) -> Result<SkillEffectResult, SkillError> {
        todo!()
    }
}
