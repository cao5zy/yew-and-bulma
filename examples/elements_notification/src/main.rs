use yew::prelude::*;
use yew_and_bulma::{elements::notification::Notification, helpers::color::Color};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Notification>{"This is a notification with no modifiers."}</Notification>
        <Notification color={Color::Primary}>{"This is a notification with the primary color."}</Notification>
        <Notification color={Color::Link}>{"This is a notification with the link color."}</Notification>
        <Notification light=true color={Color::Primary}>{"This is a notification with the light primary color."}</Notification>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
