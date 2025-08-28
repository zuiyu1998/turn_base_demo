use super::{
    SkillComponent, SkillContext, SkillEffect, SkillEffectProcessor, SkillEffectResult, SkillError,
};

pub struct DamageSkillEffectProcessor;

pub fn new_damage_skill_effect_result() -> SkillEffectResult {
    let mut res = SkillEffectResult::default();

    res.set_value("damage", 0);
    res.set_value("hit", false);

    res
}

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
        let res = new_damage_skill_effect_result();

        Ok(res)
    }
}
