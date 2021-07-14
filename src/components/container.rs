use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum ContainerDirectionType {
    Vertical,
    Horizontal,
}

impl Default for ContainerDirectionType {
    fn default() -> Self {
        ContainerDirectionType::Horizontal
    }
}

#[derive(Clone, PartialEq, Properties)]
struct ContainerProps {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    direction: ContainerDirectionType,
}

struct Container {
    props: ContainerProps,
}

impl Component for Container {
    type Properties = ContainerProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        // todo: get class name according to children's type
        let vertical_class = if self.props.direction == ContainerDirectionType::Vertical {
            Some("is-vertical")
        } else {
            None
        };
        html!{
            <section class=classes!("el-container", vertical_class)>{for self.props.children.iter()}</section>
        }
    }

    
}