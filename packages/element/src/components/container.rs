use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::Properties;
use yew::virtual_dom::VChild;
use derive_more;

use super::aside::Aside;
use super::footer::Footer;
use super::header::Header;
use super::main::Main;

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

#[derive(Clone, PartialEq, derive_more::From)]
pub enum ContainerChild {
    Header(VChild<Header>),
    Footer(VChild<Footer>),
    Aside(VChild<Aside>),
    Main(VChild<Main>),
    Container(VChild<Container>),
}

impl Into<Html> for ContainerChild {
    fn into(self) -> Html {
        match self {
            Self::Header(header) => header.into(),
            Self::Footer(footer) => footer.into(),
            Self::Aside(aside) => aside.into(),
            Self::Main(main) => main.into(),
            Self::Container(container) => container.into(),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<ContainerChild>,

    #[prop_or_default]
    pub direction: Option<ContainerDirectionType>,
}

#[derive(Clone, PartialEq)]
pub struct Container {
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
        let mut children = self.props.children.iter();

        let vertical_class = match &self.props.direction {
            Some(ContainerDirectionType::Vertical) => Some("is-vertical"),
            Some(ContainerDirectionType::Horizontal) => None,
            None => {
                let has_header_footer = children.any(|child| match child {
                    ContainerChild::Header(_) | ContainerChild::Footer(_) => true,
                    _ => false,
                });
                if has_header_footer {
                    Some("is-vertical")
                } else {
                    None
                }
            }
        };
        html!{
            <section class=classes!("el-container", vertical_class)>{for self.props.children.iter()}</section>
        }
    }

    
}