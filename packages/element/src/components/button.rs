use yew::prelude::*;
use yew::Properties;
use element_macros;

use crate::EnumStringName;

#[derive(Clone, PartialEq, element_macros::EnumStringName)]
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

#[derive(Clone, PartialEq, element_macros::EnumStringName)]
pub enum ButtonType {
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
    Text,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Default
    }
}

#[derive(Clone, PartialEq, element_macros::EnumStringName)]
pub enum ButtonNativeType {
    Button,
    Submit,
    Reset,
}

impl Default for ButtonNativeType {
    fn default() -> Self {
        ButtonNativeType::Button
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: ButtonSizeType,

    #[prop_or_default]
    pub btype: ButtonType,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub round: bool,

    #[prop_or_default]
    pub circle: bool,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub icon: String,

    #[prop_or_default]
    pub autofocus: bool,

    #[prop_or_default]
    pub native_type: ButtonNativeType,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

pub struct Button {
    props: ButtonProps,
}

impl Component for Button {
    type Properties = ButtonProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        return self.props != props
    }

    fn view(&self) -> Html {
        let disabled = self.props.disabled || self.props.loading;
        let classes = classes!(
            "el-button",
            format!("el-button--{}", self.props.btype.string_name()),
            format!("el-button--{}", self.props.size.string_name()),
            if disabled { "is-disabled" } else { "" },
            if self.props.loading { "is-loading" } else {""},
            if self.props.plain {"is-plain"} else {""},
            if self.props.round {"is-round"} else {""},
            if self.props.circle {"is-circle"} else {""},
        );

        let mut button_children = vec![];
        if self.props.loading {
            button_children.push(html!{
                <i class="el-icon-loading"></i>
            });
        }
        if self.props.icon != "" && !self.props.loading {
            button_children.push(html!{
                <i class=self.props.icon.clone()></i>
            });
        }

        if !self.props.children.is_empty() {
            button_children.push(html!{
                <span>{for self.props.children.iter()}</span>
            });
        }

        html!{
            <button
                class=classes
                onclick=self.props.onclick.clone()
                disabled=disabled
                autofocus=self.props.autofocus
                type=self.props.native_type.string_name()
            >
                {for button_children}
            </button>
        }
    }

}