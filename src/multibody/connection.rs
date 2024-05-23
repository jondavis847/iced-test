use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct JointConnection {
    pub inner_body: Option<Uuid>,
    pub outer_body: Option<Uuid>,    
}
impl Default for JointConnection {
    fn default() -> Self {
        Self {
            inner_body: None,
            outer_body: None,
        }
    }
}