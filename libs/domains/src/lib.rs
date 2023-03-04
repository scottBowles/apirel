/// Users
pub mod users;

/// Profiles
pub mod associations;

pub mod prelude {
    pub use super::associations::model as association;
    pub use super::associations::model::Entity as Association;
    pub use super::users::model as user;
    pub use super::users::model::Entity as User;
}
