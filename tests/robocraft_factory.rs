#[cfg(feature = "robocraft")]
use libfj::robocraft;
#[cfg(feature = "robocraft")]
use libfj::robocraft2;
#[cfg(feature = "robocraft")]
use std::convert::From;

#[cfg(feature = "robocraft")]
#[test]
fn robocraft_factory_api_init() -> Result<(), ()> {
    robocraft::FactoryAPI::new();
    Ok(())
}

#[cfg(feature = "robocraft")]
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

#[cfg(feature = "robocraft")]
fn builder() -> robocraft::FactorySearchBuilder {
    robocraft::FactoryAPI::new().list_builder()
}

#[cfg(feature = "robocraft")]
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

#[cfg(feature = "robocraft")]
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

#[cfg(feature = "robocraft")]
#[tokio::test]
async fn robocraft_factory_player_query() -> Result<(), ()> {
    let result = builder()
        .text("Zalera57".to_string()) // there is a featured robot by this user, so this should never fail
        .text_search_type(robocraft::FactoryTextSearchType::Player)
        .items_per_page(10)
        .send().await;
    assert!(result.is_ok());
    assert_factory_list(result.unwrap())
}

#[cfg(feature = "robocraft")]
#[tokio::test]
async fn robocraft_factory_robot_query() -> Result<(), ()> {
    let api = robocraft::FactoryAPI::new();
    let result = api.get(6478345 /* featured robot id */).await;
    assert!(result.is_ok());
    let bot_info = result.unwrap();
    assert_ne!(bot_info.response.item_name, "");
    assert_eq!(bot_info.response.item_id, 6478345);
    assert_ne!(bot_info.response.cube_data, "");
    assert_ne!(bot_info.response.colour_data, "");
    Ok(())
}

#[cfg(feature = "robocraft")]
#[tokio::test]
async fn robocraft_factory_robot_cubes() -> Result<(), ()> {
    let api = robocraft::FactoryAPI::new();
    let result = api.get(6478345 /* featured robot id */).await;
    assert!(result.is_ok());
    let bot_info = result.unwrap();
    let cubes = robocraft::Cubes::from(bot_info.clone());
    println!("cube count: {} or {}", cubes.provided_len, cubes.len());
    /*for c in cubes.into_iter() {
        println!("Cube.to_string() -> `{}`", c.to_string());
    }*/
    let (cube_d, colour_d) = cubes.dump();
    let cube_str = base64::encode(&cube_d);
    let colour_str = base64::encode(&colour_d);
    assert_eq!(cube_str, bot_info.response.cube_data);
    assert_eq!(colour_str, bot_info.response.colour_data);
    Ok(())
}
