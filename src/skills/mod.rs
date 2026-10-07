pub mod test;

use crate::prelude::*;

use anylm::api::Tool;
use osy_skill::AgentSkill;

/// System agent skills.
#[derive(AgentSkill, Clone, Copy, Debug, Display, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
#[display(rename_all = "snake_case")]
pub enum SkillKind {
    #[skill(module = "test", description = "Test skill.")]
    #[tools("hello")]
    Test,
}
