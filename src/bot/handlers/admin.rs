use teloxide::types::UserId;

pub fn is_admin(admin_ids: &[i64], user_id: UserId) -> bool {
    admin_ids.iter().any(|id| *id == user_id.0 as i64)
}
