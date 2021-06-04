#[cfg(all(feature = "robocraft", feature = "convert"))]
use libfj::convert::cubes_to_model;
#[cfg(all(feature = "robocraft", feature = "convert"))]
use libfj::robocraft;

#[cfg(all(feature = "robocraft", feature = "convert"))]
#[tokio::test]
async fn convert_to_obj() -> Result<(), ()> {
    let api = robocraft::FactoryAPI::new();
    let result = api.list().await;
    assert!(result.is_ok());
    let robot = api.get(result.unwrap().response.roboshop_items[0].item_id).await;
    assert!(robot.is_ok());
    let cubes = robot.unwrap();
    let obj = cubes_to_model(robocraft::Cubes::from(cubes.clone()));
    let save_result = obj.save(format!("tests/test-{}.obj", cubes.response.item_id));
    //save_result.unwrap();
    assert!(save_result.is_ok());
    Ok(())
}
