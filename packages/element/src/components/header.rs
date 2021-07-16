use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct HeaderProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub height: String,
}

pub struct Header {
    props: HeaderProps,
}

impl Component for Header {
    type Properties = HeaderProps;
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
        let style = if self.props.height == "" {
            "height: 60px".to_owned()
        } else {
            format!("height: {};", self.props.height)
        };
        html!{
            <header class="el-header" style=style>{for self.props.children.iter()}</header>
        }
    }

}