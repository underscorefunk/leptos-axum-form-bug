use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path="" view=|cx| view! { cx, <FormPage/> }/>
                <Route path="/form-action-page" view=|cx| view! { cx, <FormActionPage /> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn FormPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Form action="/form-action-page" method="get">
            <input type="text" id="secret" placeholder="Tell me a secret" />
            <input type="submit" value="Send request" />
        </Form>
    }
}

#[component]
fn FormActionPage(cx: Scope) -> impl IntoView {
    let fallback = "No secret was provided".to_string();
    let qm = use_query_map(cx);
    let pm = qm.get();
    let the_secret = pm.get("secret")
        .unwrap_or(&fallback);

    view! { cx,
        <h1>"You submitted a form and we ended up here!"</h1>
        <p>{the_secret}</p>
    }
}
