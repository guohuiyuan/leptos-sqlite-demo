use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    view! {
        cx,
        <div>
            <h1>"Hi from your Leptos WASM!"</h1>
        </div>
    }
}

#[component]
pub fn SimpleCounter(cx: Scope, initial_value: i32, step: i32) -> Element {
    let (value, set_value) = create_signal(cx, initial_value);

    view! { cx,
        <div>
            <button on:click=move |_| set_value(initial_value)>"Clear"</button>
            <button on:click=move |_| set_value.update(|value| *value -= step)>"-"{step}</button>
            <span>"Value: " {move || value().to_string()} "!"</span>
            <button on:click=move |_| set_value.update(|value| *value += step)>"+"{step}</button>
        </div>
    }
}