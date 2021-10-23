use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::virtual_dom::vnode::VNode::VText;
use crate::svg_builder::*;

enum Msg {
    Generate,
    Update,
    Save
}

struct ArtPiece {
    link: ComponentLink<Self>,
    art: Html,
}

impl Component for ArtPiece {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            art: create_random_spot(81,100, 10.0).unwrap().get_html()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        self.art.clone()
    }
}

struct Home {
    link: ComponentLink<Self>
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Welcome to the Spot Art Homepage!" }</h1>
                <ArtPiece />
            </div>
        }
    }
}

pub fn start() {
    yew::start_app::<Home>();
}