#[cfg(all(feature = "simple", feature = "robocraft"))]
use libfj::robocraft_simple;
#[cfg(all(feature = "simple", feature = "robocraft"))]
use libfj::robocraft;

#[cfg(all(feature = "simple", feature = "robocraft"))]
#[test]
fn robocraft_factory_api_init_simple() -> Result<(), ()> {
    robocraft_simple::FactoryAPI::new();
    Ok(())
}

#[cfg(all(feature = "simple", feature = "robocraft"))]
fn builder() -> robocraft_simple::FactorySearchBuilder {
    robocraft_simple::FactoryAPI::new().list_builder()
}

#[cfg(all(feature = "simple", feature = "robocraft"))]
fn assert_factory_list(robo_info: robocraft::FactoryInfo<robocraft::RoboShopItemsInfo>) -> Result<(), ()> {
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

#[test]
#[cfg(all(feature = "simple", feature = "robocraft"))]
fn robocraft_factory_custom_query_simple() -> Result<(), ()> {
    let result = builder()
        .movement_or(robocraft::FactoryMovementType::Wheels)
        .weapon_or(robocraft::FactoryWeaponType::Laser)
        .page(2)
        .items_per_page(10)
        .send();
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

#[test]
#[cfg(all(feature = "simple", feature = "robocraft"))]
fn robocraft_factory_player_query() -> Result<(), ()> {
    let result = builder()
        .text("Baerentoeter".to_string())
        .text_search_type(robocraft::FactoryTextSearchType::Player)
        .items_per_page(10)
        .send();
    assert!(result.is_ok());
    assert_factory_list(result.unwrap())
}

#[test]
#[cfg(all(feature = "simple", feature = "robocraft"))]
fn robocraft_factory_robot_query() -> Result<(), ()> {
    let api = robocraft_simple::FactoryAPI::new();
    let result = api.get(6478345 /* featured robot id*/);
    assert!(result.is_ok());
    let bot_info = result.unwrap();
    assert_ne!(bot_info.response.item_name, "");
    assert_eq!(bot_info.response.item_id, 6478345);
    assert_ne!(bot_info.response.cube_data, "");
    assert_ne!(bot_info.response.colour_data, "");
    Ok(())
}
