use watchmen_model::User;
use crate::AuthenticationDetails;

pub trait AuthenticationProvider {
	fn accept(&self, details: &AuthenticationDetails) -> bool;
	fn authenticate(&self, details: &AuthenticationDetails) -> Option<User>;
}
