use yew::prelude::*;
use yew::Properties;
use element::button::*;
use element::button_group::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonPageProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct ButtonPage {
    props: ButtonPageProps,
}

impl Component for ButtonPage {
    type Properties = ButtonPageProps;
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
        html!{
            <div>
                <div>{"基础用法"}</div>
                <div>
                    <Button>{"默认按钮"}</Button>
                    <Button btype=ButtonType::Primary>{"主要按钮"}</Button>
                    <Button btype=ButtonType::Success>{"成功按钮"}</Button>
                    <Button btype=ButtonType::Info>{"信息按钮"}</Button>
                    <Button btype=ButtonType::Warning>{"警告按钮"}</Button>
                    <Button btype=ButtonType::Danger>{"危险按钮"}</Button>
                </div>
                <div>{"朴素按钮"}</div>
                <div>
                    <Button plain=true>{"默认按钮"}</Button>
                    <Button plain=true btype=ButtonType::Primary>{"主要按钮"}</Button>
                    <Button plain=true btype=ButtonType::Success>{"成功按钮"}</Button>
                    <Button plain=true btype=ButtonType::Info>{"信息按钮"}</Button>
                    <Button plain=true btype=ButtonType::Warning>{"警告按钮"}</Button>
                    <Button plain=true btype=ButtonType::Danger>{"危险按钮"}</Button>
                </div>
                <div>{"圆角按钮"}</div>
                <div>
                    <Button round=true>{"默认按钮"}</Button>
                    <Button round=true btype=ButtonType::Primary>{"主要按钮"}</Button>
                    <Button round=true btype=ButtonType::Success>{"成功按钮"}</Button>
                    <Button round=true btype=ButtonType::Info>{"信息按钮"}</Button>
                    <Button round=true btype=ButtonType::Warning>{"警告按钮"}</Button>
                    <Button round=true btype=ButtonType::Danger>{"危险按钮"}</Button>
                </div>
                <div>{"图标"}</div>
                <div>
                    <Button circle=true icon="el-icon-search"></Button>
                    <Button circle=true icon="el-icon-edit" btype=ButtonType::Primary></Button>
                    <Button circle=true icon="el-icon-message" btype=ButtonType::Success></Button>
                    <Button circle=true icon="el-icon-check" btype=ButtonType::Info></Button>
                    <Button circle=true icon="el-icon-star-off" btype=ButtonType::Warning></Button>
                    <Button circle=true icon="el-icon-delete" btype=ButtonType::Danger></Button>
                </div>
                <div>{"禁用状态"}</div>
                <div>
                    <Button disabled=true>{"默认按钮"}</Button>
                    <Button disabled=true btype=ButtonType::Primary>{"主要按钮"}</Button>
                    <Button disabled=true btype=ButtonType::Success>{"成功按钮"}</Button>
                    <Button disabled=true btype=ButtonType::Info>{"信息按钮"}</Button>
                    <Button disabled=true btype=ButtonType::Warning>{"警告按钮"}</Button>
                    <Button disabled=true btype=ButtonType::Danger>{"危险按钮"}</Button>
                </div>
                <div>{"文字按钮"}</div>
                <div>
                    <Button btype=ButtonType::Text>{"文字按钮"}</Button>
                    <Button disabled=true btype=ButtonType::Text>{"文字按钮"}</Button>
                </div>
                <div>{"图标"}</div>
                <div>
                    <Button icon="el-icon-search"></Button>
                    <Button icon="el-icon-edit" btype=ButtonType::Primary></Button>
                    <Button icon="el-icon-message" btype=ButtonType::Success></Button>
                    <Button icon="el-icon-check" btype=ButtonType::Info></Button>
                    <Button icon="el-icon-star-off" btype=ButtonType::Warning></Button>
                    <Button icon="el-icon-delete" btype=ButtonType::Danger>{"删除"}</Button>
                </div>
                <div>{"按钮组"}</div>
                <div>
                    <ButtonGroup>
                        <Button btype=ButtonType::Primary icon="el-icon-arrow-left">{"上一页"}</Button>
                        <Button btype=ButtonType::Primary>
                            {"下一页"}<i class="el-icon-arrow-right el-icon--right"></i>
                        </Button>
                    </ButtonGroup>
                </div>
                <div>{"加载中"}</div>
                <div>
                    <Button loading=true btype=ButtonType::Primary>{"加载中"}</Button>
                </div>
                <div>{"不同尺寸"}</div>
                <div>
                    <Button>{"默认按钮"}</Button>
                    <Button size=ButtonSizeType::Medium>{"中等按钮"}</Button>
                    <Button size=ButtonSizeType::Small>{"小型按钮"}</Button>
                    <Button size=ButtonSizeType::Mini>{"超小按钮"}</Button>
                </div>
            </div>
        }
    }

}