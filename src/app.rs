use log::*;
use yew::{prelude::*};
use crate::components::{col::Col, row::{Row, RowType}};

// const KEY: &str = "yew.todomvc.self";

pub struct App {
}

pub enum Msg {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {        
        App {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div>
                <Row gutter=20 dtype=RowType::Flex>
                    <Col span=2>{1}</Col>
                    <Col span=3 offset=6>{2}</Col>
                </Row>
            </div>
        }
    }
}
