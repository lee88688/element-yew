use yew::prelude::*;
use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct AsideProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub width: String
}

pub struct Aside {
    props: AsideProps,
}

impl Component for Aside {
    type Properties = AsideProps;
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
        // let style = match &self.props.width {
        //     Some(width) => width.to_owned(),
        //     None => "300px".to_owned(),
        // };
        let style = if self.props.width == "" {
            "width: 300px;".to_owned()
        } else {
            format!("width: {};", self.props.width)
        };
        html!{
            <aside class="el-aside" style=style>{for self.props.children.iter()}</aside>
        }
    }

}