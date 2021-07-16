use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq)]
pub enum ButtonSizeType {
    Default,
    Medium,
    Small,
    Mini,
}

impl Default for ButtonSizeType {
    fn default() -> Self {
        ButtonSizeType::Default
    }
}

impl Into<String> for ButtonSizeType {
    fn into(self) -> String {
        match self {
            ButtonSizeType::Default => "",
            ButtonSizeType::Medium => "mediun",
            ButtonSizeType::Small => "small",
            ButtonSizeType::Mini => "mini",
        }.into()
    }
}

#[derive(Clone, PartialEq)]
pub enum ButtonType {
    
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: ButtonSizeType,
}

pub struct Button {
    props: ButtonProps,
}

impl Component for Button {
    type Properties = ButtonProps;
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