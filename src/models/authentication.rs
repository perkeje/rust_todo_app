use super::user::{User};
use std::{error::Error,fmt};


#[derive(Debug,serde::Deserialize)]
pub struct AuthUser{
    pub email:String,
    pub pass: String
}

impl AuthUser{

    pub fn authenticate(&self, connection: &diesel::PgConnection)->Result<(User,String),AuthError>{

        let user = match User::find_by_email(connection, &self.email){
            Ok(user) => user,
            Err(_) => return Err(AuthError)
        };

        match bcrypt::verify(&self.pass,&user.pass){
            Ok(check) => {
                if check==true{
                    let token = &user.generate_jwt();
                    return Ok((user,token.to_string()))
                }
                else{
                    return Err(AuthError)
                }
            }
            Err(_) => Err(AuthError)
        }

    }

}


#[derive(Debug)]
pub struct AuthError;

impl Error for AuthError{
    fn description(&self)->&str{
        "Unauthorized"
    }

}

impl fmt::Display for AuthError{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"Unauthorized")
    }
}