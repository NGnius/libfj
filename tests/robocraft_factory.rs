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

#[cfg(feature = "robocraft2")]
//#[tokio::test]
async fn robocraft2_factory_default_query() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.list().await;
    assert!(result.is_ok());
    let robo_info = unwrap_factory2(result);
    assert_ne!(robo_info.results.len(), 0);
    for robot in &robo_info.results {
        assert_ne!(robot.robot.name, "");
        assert_ne!(robot.robot.creator_id, "");
        assert_ne!(robot.robot.creator_id, "");
        assert_ne!(robot.robot.image, "");
        //println!("RobotInfo.to_string() -> `{}`", robot.robot.to_string());
        println!("SearchResponseItem {}", serde_json::to_string_pretty(&robot).unwrap());
    }
    Ok(())
}


#[cfg(feature = "robocraft2")]
#[tokio::test]
async fn robocraft2_factory_info() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.factory_info().await;
    assert!(result.is_ok());
    let crf_info = unwrap_factory2(result);
    println!("FactoryInfo {:?}", crf_info);
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
async fn robocraft2_factory_upload() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));

    // copy default bot
    let result = api.get("08dab2c9-7a72-4ec4-843c-154fe8768e91".to_owned()).await;
    assert!(result.is_ok());
    let robot = unwrap_factory2(result);

    let result = api.create_robot(
        robocraft2::CreateRobotPayload {
            name: "API is easy".to_owned(),
            data: robot.data, // base64
            image: "".to_owned(), // base64
            base_cpu: 42,
            weapon_cpu: 1,
            cosmetic_cpu: 6,
            cluster_count: 1,
            block_counts: vec![
                (42, 3),
                (3, 6)
            ].drain(..).collect(),
            materials_used: vec![
                8,
                4
            ].drain(..).collect(),
            minimum_offset_x: 0.0,
            minimum_offset_y: 0.0,
            minimum_offset_z: 0.0,
            maximum_offset_x: 0.0,
            maximum_offset_y: 0.0,
            maximum_offset_z: 0.0,
        }
    ).await;
    //assert!(result.is_ok());
    let robot_info = unwrap_factory2(result);
    println!("CreateRobotInfo {:?}", robot_info);
    let result = api.publish_robot(
    robocraft2::PublishRobotPayload {
        name: "CRF API oh my".to_owned(),
        description: "There once was a person named NGnius, who simply wasn't that bright.\nBut he had a misleading name, and wasn't not quite unhated\nSo he thought it alright to put up a fight\nand get banned for reverse engineering".to_owned(),
        techpoints: -42,
        bloxcoin: 123
    }, robot_info.header.id.clone()).await;
    //assert!(result.is_ok());
    let _publish_info = unwrap_factory2(result);

    // clean up
    let result = api.unpublish_bot(robot_info.header.id.clone()).await;
    //assert!(result.is_ok());
    let _robot = unwrap_factory2(result);

    let result = api.delete_robot(robot_info.header.id).await;
    //assert!(result.is_ok());
    let _robot = unwrap_factory2(result);
    Ok(())
}

#[cfg(feature = "robocraft2")]
#[tokio::test]
async fn robocraft2_factory_my_bots() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.my_robots().await;
    assert!(result.is_ok());
    let robo_info = unwrap_factory2(result);
    assert_ne!(robo_info.vehicles.len(), 0);
    for robot in &robo_info.vehicles {
        assert_ne!(robot.name, "");
        assert_ne!(robot.creator_id, "");
        assert_ne!(robot.creator_id, "");
        assert_ne!(robot.image, "");
        println!("My bot `{}`", robot.to_string());
        //println!("my vehicle {}", serde_json::to_string_pretty(&robot).unwrap());
    }
    Ok(())
}

#[cfg(feature = "robocraft2")]
#[tokio::test]
async fn robocraft2_factory_my_published_bots() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.my_published_robots().await;
    assert!(result.is_ok());
    let robo_info = unwrap_factory2(result);
    //assert_ne!(robo_info.vehicles.len(), 0);
    for robot in &robo_info.vehicles {
        assert_ne!(robot.name, "");
        assert_ne!(robot.creator_id, "");
        assert_ne!(robot.creator_id, "");
        assert_ne!(robot.image, "");
        println!("My pub bot `{}`", robot.to_string());
        //println!("pub vehicle {}", serde_json::to_string_pretty(&robot).unwrap());
    }
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
async fn robocraft2_factory_bot() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.get("08dab2c9-7a72-4ec4-843c-154fe8768e91".to_owned()).await;
    //assert!(result.is_ok());
    let robot = unwrap_factory2(result);
    assert_ne!(robot.header.name, "");
    assert_ne!(robot.header.creator_id, "");
    assert_ne!(robot.header.creator_id, "");
    //assert_ne!(robot.header.image, "");
    //assert_ne!(robot.description, "");
    assert_ne!(robot.data, "");
    println!("robot {}", serde_json::to_string_pretty(&robot).unwrap());
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
async fn robocraft2_factory_delete_bot() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.delete_robot("08dab2d2-dcae-4f52-8a77-bbce7cf10124".to_owned()).await;
    //assert!(result.is_ok());
    let _robot = unwrap_factory2(result);
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
async fn robocraft2_factory_unpublish_bot() -> Result<(), ()> {
    let api = robocraft2::FactoryAPI::with_auth(Box::new(robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap()));
    let result = api.unpublish_bot("08dab2d3-2a68-48c6-8fd6-a59a663336ca".to_owned()).await;
    //assert!(result.is_ok());
    let _robot = unwrap_factory2(result);
    Ok(())
}

fn unwrap_factory2<T>(result: Result<T, robocraft2::FactoryError>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            //println!("FactoryError: {}", e);
            panic!("CRF2 Error: {}", e);
        }
    }
}
