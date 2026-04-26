use crate::shared::config::Mode;

#[derive(Debug, Clone)]
pub struct TenantContext {
    pub client_id: i64,
    pub mode: Mode,
}

impl TenantContext {
    pub fn onprem() -> Self {
        Self {
            client_id: 1,
            mode: Mode::OnPrem,
        }
    }
}
