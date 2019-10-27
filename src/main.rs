#![recursion_limit = "512"]

use yew::prelude::*;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</p>
                <p>{ "Hello world!" }</pp> // <- typo in closing </p> tag causes build panic
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
