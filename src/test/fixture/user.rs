#[cfg(test)]
use crate::model::User;

#[cfg(test)]
pub fn user_fixture(id: usize) -> User {
    User {
        id: id as i32,
        name: String::from("taro"),
    }
}

#[cfg(test)]
pub fn users_fixture(num: usize) -> Vec<User> {
    let mut users = vec![];
    for i in 1..num + 1 {
        users.push(user_fixture(i));
    }
    users
}