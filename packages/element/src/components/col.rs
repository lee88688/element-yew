use yew::prelude::*;
use yew::Properties;

fn create_default_tag() -> String {
    String::from("div")
}

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    /// should inherit from parent
    #[prop_or_default]
    pub gutter: i32,

    #[prop_or_else(create_default_tag)]
    pub tag: String,

    #[prop_or_default]
    pub span: Option<i32>,

    #[prop_or_default]
    pub offset: Option<i32>,

    #[prop_or_default]
    pub pull: Option<i32>,

    #[prop_or_default]
    pub push: Option<i32>,

    #[prop_or_default]
    pub children: Children,
}

pub struct Col {
    props: ColProps,
}

impl Component for Col {
    type Properties = ColProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props == props
    }

    fn view(&self) -> Html {
        let mut class_list = vec![String::from("el-col")];

        if let Some(span) = self.props.span {
            class_list.push(format!("el-col-{}", span))
        }

        if let Some(offset) = self.props.offset {
            class_list.push(format!("el-col-offset-{}", offset))
        }

        if let Some(pull) = self.props.pull {
            class_list.push(format!("el-col-pull-{}", pull))
        }

        if let Some(push) = self.props.push {
            class_list.push(format!("el-col-pull-{}", push))
        }

        let style = if self.props.gutter > 0 {
            let padding = self.props.gutter / 2;
            format!("padding-left: {}px; padding-right: {}px;", padding, padding)
        } else {
            format!("")
        };

        html!{
            <@{self.props.tag.to_owned()} class=class_list style=style>
                {for self.props.children.iter()}
            </@>
        }
    }
}