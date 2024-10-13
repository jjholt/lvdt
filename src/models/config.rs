use serde::Deserialize;
use crate::models::plane::Plane;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) screws: Plane,
    pub(crate) implant: Plane,
}
