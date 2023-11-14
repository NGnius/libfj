#[cfg(feature = "robocraft2")]
use libfj::robocraft2;

#[cfg(feature = "robocraft2")]
async fn builder() -> robocraft2::FactoryAPI {
    let token = robocraft2::PortalTokenProvider::with_username("FJAPIC00L", "P4$$w0rd").await.unwrap();
    robocraft2::FactoryAPI::with_auth(Box::new(token))
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
#[allow(dead_code)]
async fn robocraft2_factory_moderate() -> Result<(), ()> {
    let api = builder().await;
    let robot = api.list().await.unwrap().results.pop().unwrap();
    let result = api.moderate(robocraft2::ModerateRobotPayload {
        approve: false,
        reason: "Automated test".to_owned(),
    }, robot.robot.id).await;
    assert!(result.is_ok());
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
#[allow(dead_code)]
async fn robocraft2_factory_report() -> Result<(), ()> {
    let api = builder().await;
    let robot = api.list().await.unwrap().results.pop().unwrap();
    let result = api.report(robocraft2::ReportRobotPayload {
        reason: "Automated test".to_owned(),
    }, robot.robot.id).await;
    assert!(result.is_ok());
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
#[allow(dead_code)]
async fn robocraft2_factory_default_query() -> Result<(), ()> {
    let api = builder().await;
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
    let api = builder().await;
    let result = api.factory_info().await;
    assert!(result.is_ok());
    let crf_info = unwrap_factory2(result);
    println!("FactoryInfo {:?}", crf_info);
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
#[allow(dead_code)]
async fn robocraft2_factory_upload() -> Result<(), ()> {
    let api = builder().await;

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
#[allow(dead_code)]
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
#[allow(dead_code)]
async fn robocraft2_factory_delete_all_my_bots() -> Result<(), ()> {
    let api = builder().await;
    let my_bots = api.my_published_robots().await.unwrap();
    for bot in my_bots.vehicles {
        let result = api.delete_robot(bot.id).await;
        unwrap_factory2(result);
    }
    Ok(())
}

#[cfg(feature = "robocraft2")]
//#[tokio::test]
#[allow(dead_code)]
async fn robocraft2_factory_unpublish_all_my_bots() -> Result<(), ()> {
    let api = builder().await;
    let my_bots = api.my_published_robots().await.unwrap();
    for bot in my_bots.vehicles {
        let result = api.unpublish_bot(bot.id).await;
        unwrap_factory2(result);
    }
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
