use crate::GameSystemType;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum VSError {
    Backend_NoVideo,
    Backend_NoVideoModes,
    Backend_InvalidResolutionValue(i32),
    Backend_NoResolutionFound,
    Backend_DisplayBoundsInfoMissing,
    Backend_NoWindow,

    Core_DuplicateGameSystem,
    Core_SystemNotFound(GameSystemType),
}
