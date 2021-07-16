// use yew::{prelude::*};
use yew_router::{prelude::*};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/container"]
    Contianer,
    #[to = "/button"]
    Button,
}

