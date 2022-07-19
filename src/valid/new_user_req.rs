use validr::*;
use serde::Deserialize;

#[derive(Clone,Deserialize,Debug)]
pub struct NewUserReq{
    pub email:Option<String>,
    pub pass: String
}
impl Validation for NewUserReq{
    fn modifiers(&self) -> Vec<Modifier<Self>> {
        vec![modifier_lowercase!(email)]
    }
    fn rules(&self) -> Vec<Rule<Self>> {
        
        vec![rule_email!(email),
            rule_length_min!(pass,3),
            rule_required!(email),
            rule_required!(pass)]
    }
    
}