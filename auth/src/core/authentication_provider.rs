use crate::AuthenticationDetails;
use elf_model::User;

pub trait AuthenticationProvider {
    fn accept(&self, details: &AuthenticationDetails) -> bool;
    fn authenticate(&self, details: &AuthenticationDetails) -> Option<User>;
}
