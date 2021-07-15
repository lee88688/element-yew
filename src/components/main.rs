use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct MainProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Main {
    props: MainProps,
}

impl Component for Main {
    type Properties = MainProps;
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
        html! {
            <main class="el-main">{for self.props.children.iter()}</main>
        }
    }

}