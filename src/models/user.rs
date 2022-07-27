use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::users;
use bcrypt;
use diesel::pg::PgConnection;
use diesel::result;
use serde;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub pass: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("email", &self.email)?;
        state.end()
    }
}

impl User {
    pub fn find_by_email(connection: &PgConnection, email: &str) -> Result<User, result::Error> {
        users::table
            .filter(users::email.eq(email))
            .first::<User>(connection)
    }

    pub fn generate_jwt(&self) -> String {
        crate::jwt::generate(self)
    }

    pub fn from_jwt(payload: &crate::jwt::Claims) -> Self {
        User {
            id: String::from(&payload.sub),
            email: String::from(&payload.email),
            pass: String::new(),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub pass: String,
}
impl NewUser {
    pub fn create(
        connection: &PgConnection,
        email: &str,
        password: &str,
    ) -> Result<User, result::Error> {
        let hash_pass = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_err) => return Err(result::Error::__Nonexhaustive),
        };

        let user = Self {
            email: String::from(email),
            pass: hash_pass,
        };

        diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(connection)
    }
}
