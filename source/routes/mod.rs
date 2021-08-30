/// The route for `/`.
mod home;
/// The route for `/userstyles`.
mod userstyles;

pub(crate) use self::userstyles::UserstylesRoute;
pub(crate) use home::HomeRoute;
