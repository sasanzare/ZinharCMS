use crate::routes::DependencyCheck;

pub fn dependency(
    name: impl Into<String>,
    ok: bool,
    message: impl Into<String>,
) -> DependencyCheck {
    DependencyCheck {
        name: name.into(),
        ok,
        message: message.into(),
    }
}
