#[cfg(feature = "robocraft")]
use libfj::robocraft;
#[cfg(feature = "robocraft")]
use libfj::robocraft::ITokenProvider;

#[cfg(feature = "robocraft")]
#[test]
fn robocraft_auth_login() -> Result<(), ()> {
    let token_maybe = robocraft::AuthenticatedTokenProvider::with_email("melon.spoik@gmail.com", "P4$$w0rd");
    assert!(token_maybe.is_ok());
    let token_maybe = robocraft::AuthenticatedTokenProvider::with_username("FJAPIC00L", "P4$$w0rd");
    assert!(token_maybe.is_ok());
    let token_p = token_maybe.unwrap();
    let raw_token_maybe = token_p.token();
    assert!(raw_token_maybe.is_ok());
    println!("Token: {}", raw_token_maybe.unwrap());
    Ok(())
}

#[cfg(feature = "robocraft")]
#[test]
fn robocraft_account() -> Result<(), ()> {
    let token_maybe = robocraft::AuthenticatedTokenProvider::with_username("FJAPIC00L", "P4$$w0rd");
    assert!(token_maybe.is_ok());
    let token_provider = token_maybe.unwrap();
    let account_maybe = token_provider.get_account_info();
    assert!(account_maybe.is_ok());
    let account = account_maybe.unwrap();
    assert_eq!(account.display_name, "FJAPIC00L");
    assert_eq!(account.created_date, "2019-01-18T14:48:09");
    Ok(())
}
