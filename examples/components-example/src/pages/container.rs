use log::*;
use yew::{prelude::*};
use element::{header::Header, container::Container, footer::Footer, aside::Aside, main::Main};

// const KEY: &str = "yew.todomvc.self";

pub struct ContainerPage {
}

impl Component for ContainerPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {        
        ContainerPage {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div>
                <Container>
                    <Header height="400px">{"Header"}</Header>
                    <Main>{"Main"}</Main>
                </Container>

                <Container>
                    <Header>{"Header"}</Header>
                    <Main>{"Main"}</Main>
                    <Footer height="400px">{"Footer"}</Footer>
                </Container>

                <Container>
                    <Aside width="200px">{"Aside"}</Aside>
                    <Main>{"Main"}</Main>
                </Container>
                
                <Container>
                    <Header>{"Header"}</Header>
                    <Container>
                        <Aside width="200px">{"Aside"}</Aside>
                        <Main>{"Main"}</Main>
                    </Container>
                </Container>
                
                <Container>
                    <Header>{"Header"}</Header>
                    <Container>
                    <Aside width="200px">{"Aside"}</Aside>
                    <Container>
                        <Main>{"Main"}</Main>
                        <Footer>{"Footer"}</Footer>
                    </Container>
                    </Container>
                </Container>
                
                <Container>
                    <Aside width="200px">{"Aside"}</Aside>
                    <Container>
                    <Header>{"Header"}</Header>
                    <Main>{"Main"}</Main>
                    </Container>
                </Container>
                
                <Container>
                    <Aside>{"Aside"}</Aside>
                    <Container>
                    <Header>{"Header"}</Header>
                    <Main>{"Main"}</Main>
                    <Footer>{"Footer"}</Footer>
                    </Container>
                </Container>
            </div>
        }
    }
}
