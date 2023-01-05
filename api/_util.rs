use std::{env, error::Error};

use jsonwebtoken::{decode, errors::ErrorKind, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Serialize, Deserialize)]
struct FirebaseClaims {
    sign_in_provider: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    sub: String,
    name: String,
    email: String,
    email_verified: bool,
    user_id: String,
    picture: String,
    firebase: FirebaseClaims,
    exp: u64,
    iat: u64,
}

async fn get_pbkey() -> Result<Map<String, Value>, Box<dyn Error>> {
    let url = format!(
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com",
    );
    let url = Url::parse(&*url)?;
    let res = reqwest::get(url).await?.json().await?;
    Ok(res)
}

pub async fn verify_token(
    uid: &str,
    client_token: &str,
) -> Result<TokenData<Claims>, Box<dyn Error>> {
    let res = get_pbkey().await?;
    let ky = res.keys().nth(1).unwrap(); // we just need to pick one of the provided public key from firebase
    let ky = res[ky].as_str().unwrap();
    let pub_key = ky.as_bytes();

    let mut validation = Validation::new(Algorithm::RS256);

    let fb_project_id = env::var("FIREBASE_PROJECT_ID")?;
    let issuer_str = format!("https://securetoken.google.com/{fb_project_id}");
    validation.sub = Some(uid.to_string());
    validation.set_audience(&[fb_project_id]);
    validation.set_issuer(&[issuer_str]);

    let token_data: Result<TokenData<Claims>, Box<dyn Error>> = match decode::<Claims>(
        &client_token,
        &DecodingKey::from_rsa_pem(pub_key).unwrap(),
        &validation,
    ) {
        Ok(c) => Ok(c),
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => Err("invalid-token")?,
            ErrorKind::InvalidIssuer => Err("invalid-issuer")?,
            ErrorKind::InvalidSubject => Err("invalid-subject")?,
            ErrorKind::InvalidAlgorithm => Err("invalid-alg")?,
            ErrorKind::InvalidAudience => Err("invalid-aud")?,
            ErrorKind::InvalidEcdsaKey => Err("invalid-ecdsa-key")?,
            ErrorKind::InvalidAlgorithmName => Err("invalid-alg-name")?,
            ErrorKind::InvalidKeyFormat => Err("invalid-key-format")?,
            ErrorKind::ExpiredSignature => Err("expired-signature")?,
            ErrorKind::ImmatureSignature => Err("immature-signature")?,
            ErrorKind::RsaFailedSigning => Err("rsa-failed-signing")?,
            _ => Err("other-errors")?,
        },
    };

    return token_data;
}
