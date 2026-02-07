use crate::models::UserId;

pub const SESSION_USER_ID_KEY: &str = "authenticated_user_id";

/// Injected via Extension by load_session_extensions middleware.
///
/// Protected routes use `require_authentication` middleware which redirects
/// guests before handlers run. If `require_authenticated()` returns None:
/// check route is in protected_routes(), verify middleware ordering
/// (session_layer → load_session_extensions → require_authentication).
#[derive(Clone, Debug)]
pub enum CurrentUser {
    Authenticated {
        user_id: UserId,
        email: String,
        is_admin: bool,
    },
    Guest,
}

impl CurrentUser {
    /// Only call in protected routes — returns None on Guest.
    /// If this returns None, check route is in protected_routes() and
    /// verify middleware ordering (session_layer → load_session_extensions → require_authentication).
    pub fn require_authenticated(&self) -> Option<&UserId> {
        match self {
            CurrentUser::Authenticated { user_id, .. } => Some(user_id),
            CurrentUser::Guest => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            CurrentUser::Authenticated { is_admin, .. } => *is_admin,
            CurrentUser::Guest => false,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, CurrentUser::Authenticated { .. })
    }
}
