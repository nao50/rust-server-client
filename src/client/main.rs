use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PublicKeyCredentialCreationOptions {
    publicKey: PublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicKey {
    challenge: String,
    rp: Rp,
    user: User,
    pubKeyCredParams: Vec<PubKeyCredParams>,
    authenticatorSelection: AuthenticatorSelection,
    timeout: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Rp {
    name: String,
    icon: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    displayName: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PubKeyCredParams {
    r#type: String,
    alg: i16,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticatorSelection {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let resp: PublicKeyCredentialCreationOptions =
        reqwest::get("http://localhost:5051/register/begin/rust")
            .await?
            .json()
            .await?;

    // println!("{:#?}", resp);
    println!("{:#?}", resp.publicKey);
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

// use hyper::body::HttpBody as _;
// use hyper::Client;
// use tokio::io::{stdout, AsyncWriteExt as _};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     // This is where we will setup our HTTP client requests.
//     let client = Client::new();
//     let uri = "http://httpbin.org/ip".parse()?;
//     let mut resp = client.get(uri).await?;
//     println!("Response: {}", resp.status());

//     while let Some(chunk) = resp.body_mut().data().await {
//         stdout().write_all(&chunk?).await?;
//     }

//     Ok(())
// }
