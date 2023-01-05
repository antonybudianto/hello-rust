use http::StatusCode;
use jsonwebtoken::TokenData;
use std::error::Error;
use util::{verify_token, Claims};
use vercel_lambda::{error::VercelError, lambda, IntoResponse, Request, Response};

#[tokio::main]
async fn fetch_data(uid: &str, token: &str) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let result = verify_token(uid, token).await;
    return result;
}

fn handler(request: Request) -> Result<impl IntoResponse, VercelError> {
    let opt_uid = request.headers().get("X-Firebase-UID");
    let opt_client_token = request.headers().get("X-Firebase-ClientToken");
    let mut uid = "";
    let mut client_token = "";

    match opt_uid {
        Some(x) => {
            uid = x.to_str().unwrap();
        }
        None => println!("x-firebase-uid not found"),
    }
    match opt_client_token {
        Some(x) => {
            client_token = x.to_str().unwrap();
        }
        None => println!("x-firebase-clienttoken not found"),
    }

    let body: String;
    let mut status_code = StatusCode::OK;

    let result = fetch_data(uid, client_token);
    match result {
        Ok(res) => {
            let text = serde_json::to_string(&res.claims).unwrap();
            body = text;
        }
        Err(e) => {
            println!("Error fetch_data: {e}");
            status_code = StatusCode::BAD_REQUEST;
            let err = format!("{{ \"error\": \"{e}\" }}");
            body = err.to_string();
        }
    }

    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .expect("Internal Server Error");
    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
