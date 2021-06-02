#[cfg(feature = "cardlife")]
use libfj::cardlife;

#[cfg(feature = "cardlife")]
const EMAIL: &str = "";
#[cfg(feature = "cardlife")]
const PASSWORD: &str = "";

#[cfg(feature = "cardlife")]
#[test]
fn live_api_init() -> Result<(), ()> {
    cardlife::LiveAPI::new();
    Ok(())
}

#[cfg(feature = "cardlife")]
#[tokio::test]
async fn live_api_init_auth() -> Result<(), ()> {
    let live = cardlife::LiveAPI::login_email(EMAIL, PASSWORD).await;
    assert!(live.is_err()); // invalid credentials
    Ok(())
}

#[cfg(feature = "cardlife")]
#[tokio::test]
async fn live_api_authenticate() -> Result<(), ()> {
    let mut live = cardlife::LiveAPI::new();
    let result = live.authenticate_email(EMAIL, PASSWORD).await;
    assert!(result.is_err()); // invalid credentials
    /*let auth_info = result.unwrap();
    assert_ne!(auth_info.token, "");
    assert_ne!(auth_info.display_name, "");
    assert_eq!(auth_info.email_address, EMAIL);
    assert_ne!(auth_info.public_id, "");
    println!("AuthenticationInfo.to_string() -> `{}`", auth_info.to_string());*/
    Ok(())
}

#[cfg(feature = "cardlife")]
#[tokio::test]
async fn live_api_lobbies() -> Result<(), ()> {
    //let live = cardlife::LiveAPI::login_email(EMAIL, PASSWORD).await.unwrap();
    let live = cardlife::LiveAPI::new();
    let result = live.lobbies().await;
    assert!(result.is_err());
    /*
    let lobby_info = result.unwrap();
    assert_ne!(lobby_info.games.len(), 0);
    for game in &lobby_info.games {
        println!("LiveGameInfo.to_string() -> `{}`", game.to_string());
    }*/
    Ok(())
}
