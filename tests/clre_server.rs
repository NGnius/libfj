use libfj::cardlife;

#[test]
fn clre_server_init() -> Result<(), ()> {
    assert!(cardlife::CLreServer::new("http://localhost:5030").is_ok());
    Ok(())
}

#[tokio::test]
async fn clre_server_game() -> Result<(), ()> {
    let server = cardlife::CLreServer::new("http://localhost:5030").unwrap();
    let result = server.game_info().await;
    assert!(result.is_ok());
    let game_info = result.unwrap();
    assert_eq!(game_info.admin_password, "");
    assert_eq!(game_info.game_host_type, 1);
    println!("GameInfo.to_string() -> `{}`", game_info.to_string());
    Ok(())
}

#[tokio::test]
async fn clre_server_status() -> Result<(), ()> {
    let server = cardlife::CLreServer::new("http://localhost:5030").unwrap();
    let result = server.status_info().await;
    assert!(result.is_ok());
    let status_info = result.unwrap();
    assert_eq!(status_info.status, "Online");
    assert_eq!(status_info.max_players, 10);
    assert_eq!(status_info.player_count, status_info.online_players.len());
    if status_info.online_players.len() != 0 {
        for player in &status_info.online_players {
            assert_ne!(player.name, "");
            assert_ne!(player.id, "");
            assert!(!player.is_dev);
            println!("PlayerStatusInfo.to_string() -> `{}`", player.to_string());
        }
    }
    Ok(())
}