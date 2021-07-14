use yew::prelude::*;
use yew::Properties;
use crate::components::col::Col;

#[derive(Clone, PartialEq)]
pub enum RowType {
    Default,
    Flex,
}

impl Default for RowType {
    fn default() -> Self {
        RowType::Default
    }
}

#[derive(Clone, PartialEq)]
pub enum RowJustifyType {
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween
}

impl Default for RowJustifyType {
    fn default() -> Self {
        RowJustifyType::Start
    }
}

#[derive(Clone, PartialEq)]
pub enum RowAlignType {
    Top,
    Middle,
    Bottom
}

impl Default for RowAlignType {
    fn default() -> Self {
        RowAlignType::Top
    }
}

fn create_default_tag() -> String {
    String::from("div")
}

#[derive(Properties, Clone, PartialEq)]
pub struct RowProps {
    #[prop_or_default]
    pub gutter: i32,

    #[prop_or_default]
    pub dtype: RowType,

    #[prop_or_default]
    pub justify: RowJustifyType,

    #[prop_or_default]
    pub align: RowAlignType,

    #[prop_or_else(create_default_tag)]
    pub tag: String,

    #[prop_or_default]
    pub children: ChildrenWithProps<Col>
}

pub struct Row {
    props: RowProps,
}

impl Component for Row {
    type Properties = RowProps;
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
        let tag_name = format!("{}", self.props.tag);
        let justify_class = match self.props.justify {
            RowJustifyType::Start => "",
            RowJustifyType::Center => "is-justify-center",
            RowJustifyType::End => "is-justify-end",
            RowJustifyType::SpaceAround => "is-justify-space-around",
            RowJustifyType::SpaceBetween => "is-justify-space-between",
        };
        let align_class = match self.props.align {
            RowAlignType::Top => "is-align-top",
            RowAlignType::Middle => "is-align-middle",
            RowAlignType::Bottom => "is-align-bottom",
        };
        let type_class = match self.props.dtype {
            RowType::Default => None,
            RowType::Flex => Some("el-row--flex")
        };
        let style = if self.props.gutter != 0 {
            format!("margin-left: {}px; margin-right: {}px", -self.props.gutter / 2, -self.props.gutter / 2)
        } else {
            format!("")
        };

        let children = self.props.children.iter().map(|mut item| {
            item.props.gutter = self.props.gutter;
            item
        });

        html! {
            <@{tag_name} class=classes!("el-row", type_class, align_class, justify_class) style=style>
                {for children}
            </@>
        }
    }
}
