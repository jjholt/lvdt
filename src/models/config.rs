use crate::models::plane::Plane;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) screws: Plane,
    pub(crate) implant: Plane,
}
