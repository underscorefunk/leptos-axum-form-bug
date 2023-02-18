use std::str::Utf8Error;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;


cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
	use leptos_axum::RequestParts;
    }
}

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
        <form action="/form-action-page" method="post">
            <input type="text" name="secret" id="secret" placeholder="Tell me a secret" />
            <input type="submit" value="Send request" />
        </form>
    }
}

#[cfg(feature = "ssr")]
#[component]
fn FormActionPage(cx: Scope) -> impl IntoView {
    view!{cx, "Server side response. This should display as a result of submitting the form."}
}

#[cfg(not(feature = "ssr"))]
#[component]
fn FormActionPage(cx: Scope) -> impl IntoView {
    view! {cx, "Client side response. We're not doing CSR here."}
}