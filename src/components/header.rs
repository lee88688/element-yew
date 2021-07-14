use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct HeaderProps {
    #[prop_or_default]
    children: Children,
}

pub struct Header {
    props: HeaderProps,
}

impl Component for Header {
    type Properties = HeaderProps;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        todo!()
    }

}