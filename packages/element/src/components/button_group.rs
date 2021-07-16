use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonGroupProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct ButtonGroup {
    props: ButtonGroupProps,
}

impl Component for ButtonGroup {
    type Properties = ButtonGroupProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="el-button-group">{for self.props.children.iter()}</div>
        }
    }

}