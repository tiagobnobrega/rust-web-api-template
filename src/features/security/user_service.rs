use crate::db::DbConnection;
use crate::schema::users::dsl::users;
use crate::{NewUser, Role, User};
use diesel::associations::HasTable;
use diesel::RunQueryDsl;

pub struct UserService<'db> {
    db: &'db DbConnection,
}

impl<'db> UserService<'db> {
    pub fn new(db: &'db DbConnection) -> Self {
        Self { db }
    }

    pub fn create_user_with_roles(&self, user: &NewUser, roles: Vec<&str>) -> User {
        let res = diesel::insert_into(users::table).values(user).get_result();
    }
}
