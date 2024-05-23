use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct JointConnection {
    inner_body: Option<Uuid>,
    outer_body: Option<Uuid>,    
}
impl Default for JointConnection {
    fn default() -> Self {
        Self {
            inner_body: None,
            outer_body: None,
        }
    }
}