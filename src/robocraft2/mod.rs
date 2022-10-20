//! Robocraft2 APIs for the CRF.
//! Subject to change and breakages as RC2 is still in an early development stage.

mod factory;
pub use factory::FactoryAPI;

mod factory_json;
pub use factory_json::{SearchPayload, SearchResponse, SearchResponseItem, RobotInfo, RobotPrice};

mod portal;
pub use self::portal::{PortalTokenProvider, AccountInfo, PortalCheckResponse, ITokenProvider};
