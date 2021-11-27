use gloo_console::console_dbg;
use yew::prelude::*;

fn main() {
    yew::start_app::<Outer>();
}

#[function_component(Outer)]
fn outer() -> Html {
    let counter = use_state(|| 1usize);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {
        <Inner
            div_count={*counter}
            onclick={onclick}
            />
    }
}

#[derive(Properties, PartialEq)]
struct InnerProps {
    div_count: usize,
    onclick: Callback<()>,
}

#[function_component(Inner)]
fn inner(props: &InnerProps) -> Html {
    let node_ref = use_state(NodeRef::default);
    let node_ref_inner = node_ref.clone();
    use_effect(move || {
        console_dbg!(&node_ref_inner);
        if let Some(input) = node_ref_inner.cast::<web_sys::HtmlInputElement>() {
            input.focus().unwrap();
        }
        || ()
    });
    let divs = (0..props.div_count).map(|i| {
        html! {
            <div><h6>{ format!("DIV #{}", i) }</h6></div>
        }
    });
    html! {
        <div>
            <h6>{ "OUTER DIV" }</h6>
            { for divs }
            <input ref={NodeRef::clone(&node_ref)} value="INPUT" />
            <button onclick={props.onclick.reform(|_| ())}>
                { "CLICK ME" }
            </button>
        </div>
    }
}
