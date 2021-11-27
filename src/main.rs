use gloo_console::console_dbg;
use yew::prelude::*;

fn main() {
    yew::start_app::<Outer>();
}

struct Outer {
    count: usize,
}
impl Component for Outer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 1 }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        self.count += 1;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Inner
                div_count={self.count}
                onclick={ctx.link().callback(|_| ())}
                />
        }
    }
}

struct Inner {
    node_ref: NodeRef,
}
#[derive(Properties, PartialEq)]
struct InnerProps {
    div_count: usize,
    onclick: Callback<()>,
}
impl Component for Inner {
    type Message = ();
    type Properties = InnerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    // Try to focus the input on every render.
    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        // Watch console output to see that after each event, the NodeRef is reset.
        console_dbg!(&self.node_ref);
        if let Some(input) = self.node_ref.cast::<web_sys::HtmlInputElement>() {
            input.focus().unwrap();
        }
    }

    // fn changed(&mut self, _ctx: &Context<Self>) -> bool {
    //     self.node_ref = NodeRef::default();
    //     true
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let divs = (0..ctx.props().div_count).map(|i| {
            html! {
                <div><h6>{ format!("DIV #{}", i) }</h6></div>
            }
        });
        html! {
            <div>
                <h6>{ "OUTER DIV" }</h6>
                { for divs }
                <input ref={self.node_ref.clone()} value="INPUT" />
                <button onclick={ctx.props().onclick.reform(|_| ())}>
                    { "CLICK ME" }
                </button>
            </div>
        }
    }
}
