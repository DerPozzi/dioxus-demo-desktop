#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            background: "blue",
            "Hello World"
        }
        div {
            text_align: "center",
            "Das ist ein Test, muss das neu Kompiliert werden? Okay das Krass, das sind live Änderungen hier..."
        }
        AppComponent{
             name: "Emily".to_string() 
        } 
    })
}

#[component]
fn AppComponent(cx: Scope, name: String) -> Element {
    render!(
        div {
            "Hallo {name}, wie geht es dir?"
        }
    )
}
