use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());

    view! {
        cx,
        <div id="root">
        <Router>
            <nav>
                <A exact=true href="/">"SimpleCounter"</A>
                <A href="about">"About"</A>
                <A href="settings">"Settings"</A>
            </nav>
            <main>
                <Routes>
                    <Route
                        path="/"
                        element=move |cx| view! { cx,  <SimpleCounter initial_value=3 step=2/> }
                    >
                    </Route>
                    <Route
                        path="/about"
                        element=move |cx| view! { cx,  <div>"this is about"</div> }
                    >
                    </Route>
                    <Route
                        path="/settings"
                        element=move |cx| view! { cx,  <div>"this is about"</div> }
                    >
                    </Route>
                </Routes>
            </main>
        </Router>
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
