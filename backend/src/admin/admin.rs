use axum::{ 
    extract::Request, 
    http::{
        StatusCode
    }, 
    middleware::{
        Next
    }, 
    response::{
        IntoResponse,
        Response,
        Html
    }
    
};

pub async fn hello_admin() -> Response{
    println!("admin accessed");
    Html("<h1> Hey Fella </h1>").into_response()
}

pub async fn auth_admin(
    request: Request,
    next: Next,
) -> Response {


    //check role

    //get user from session
    //check user admin status
   
    if true {
        next.run(request).await
    }
    else{
       
       (StatusCode::FORBIDDEN, "not cool").into_response()
    }



}
