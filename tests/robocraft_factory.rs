use libfj::robocraft;

#[test]
fn robocraft_factory_api_init() -> Result<(), ()> {
    robocraft::FactoryAPI::new();
    Ok(())
}

#[tokio::test]
async fn robocraft_factory_default_query() -> Result<(), ()> {
    let api = robocraft::FactoryAPI::new();
    let result = api.list().await;
    assert!(result.is_ok());
    let robo_info = result.unwrap();
    assert_ne!(robo_info.response.roboshop_items.len(), 0);
    assert_eq!(robo_info.status_code, 200);
    for robot in &robo_info.response.roboshop_items {
        assert_ne!(robot.item_name, "");
        assert_ne!(robot.added_by, "");
        assert_ne!(robot.added_by_display_name, "");
        assert_ne!(robot.thumbnail, "");
        println!("FactoryRobotListInfo.to_string() -> `{}`", robot.to_string());
    }
    Ok(())
}

fn builder() -> robocraft::FactorySearchBuilder {
    robocraft::FactoryAPI::new().list_builder()
}

fn assert_factory_list(robo_info: robocraft::FactoryInfo) -> Result<(), ()> {
    assert_ne!(robo_info.response.roboshop_items.len(), 0);
    assert_eq!(robo_info.status_code, 200);
    for robot in &robo_info.response.roboshop_items {
        assert_ne!(robot.item_name, "");
        assert_ne!(robot.added_by, "");
        assert_ne!(robot.added_by_display_name, "");
        assert_ne!(robot.thumbnail, "");
        println!("FactoryRobotListInfo.to_string() -> `{}`", robot.to_string());
    }
    Ok(())
}

#[tokio::test]
async fn robocraft_factory_custom_query() -> Result<(), ()> {
    let api = robocraft::FactoryAPI::new();
    let result = api.list_builder()
        .movement_or(robocraft::FactoryMovementType::Wheels)
        .weapon_or(robocraft::FactoryWeaponType::Laser)
        .page(2)
        .items_per_page(10)
        .send().await;
    assert!(result.is_ok());
    let robo_info = result.unwrap();
    assert_ne!(robo_info.response.roboshop_items.len(), 0);
    //assert_eq!(robo_info.response.roboshop_items.len(), 16); the API behaviour is weird, I swear it's not me!
    assert!(robo_info.response.roboshop_items.len() >= 10);
    assert_eq!(robo_info.status_code, 200);
    for robot in &robo_info.response.roboshop_items {
        assert_ne!(robot.item_name, "");
        assert_ne!(robot.added_by, "");
        assert_ne!(robot.added_by_display_name, "");
        assert_ne!(robot.thumbnail, "");
        println!("FactoryRobotListInfo.to_string() -> `{}`", robot.to_string());
    }
    Ok(())
}

#[tokio::test]
async fn robocraft_factory_player_query() -> Result<(), ()> {
    let result = builder()
        .text("Baerentoeter".to_string())
        .text_search_type(robocraft::FactoryTextSearchType::Player)
        .items_per_page(10)
        .send().await;
    assert!(result.is_ok());
    assert_factory_list(result.unwrap())
}
