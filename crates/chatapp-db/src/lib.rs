pub mod entity;
pub mod migrations;

pub use entity::users::{ActiveModel as UserActiveModel, Entity as Users, Model as UserModel};
