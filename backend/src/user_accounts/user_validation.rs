
extern crate zxcvbn; //for validation
use zxcvbn::zxcvbn;
use::email_address::*;

use crate::{
    shb_error::BackendError, 
    user_accounts::users::{
        UserLoginDetails
    }
};




pub fn validate_email(email :&str) -> Result<(), BackendError>{
    //https://docs.rs/email_address/latest/email_address/struct.Options.html
    // let email_addr = EmailAddress::is_valid(email);
    if EmailAddress::parse_with_options(email, Options::default().with_required_tld()).is_ok() {
        return Ok(());
    }
    else{
        return Err(BackendError::BadRequest("Invalid email".to_string()));
    }
}


pub fn validate_password(user: &UserLoginDetails) -> Result<(), BackendError>{
    let user_email = user.email.clone();
    let user_email = user_email.unwrap();
    
    let password_strength_est = zxcvbn(&user.password, &[&user_email, &user.username]);
    match password_strength_est.feedback() {
        Some(feedback) =>{
            let mut errmsg = "Password not strong enough \nSuggestions: ".to_string();
            for suggestion in feedback.suggestions(){
                errmsg.push_str(&suggestion.to_string());
                errmsg.push_str(" ");
            }
            errmsg.push_str("\n");
            if feedback.warning().is_some() {
                errmsg.push_str(&feedback.warning().unwrap().to_string());
            }
            return Err(BackendError::BadRequest(errmsg));
        },
        None => { return Ok(()); }
    }
}