#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 {
            "Hello World"
        }
        div {
            text_align: "center",
            "Das ist ein Test, muss das neu Kompiliert werden? Okay das Krass, das sind live Änderungen hier..."
        }
        AppComponent{
             name: "Emily".to_string() 
        }
        MyComponen{}
    })
}

fn MyComponen(cx: Scope) -> Element {
    render!(
            div { "Das ist ein Test ob ich mit vim arbeiten kann." }
    )
}

#[component]
fn AppComponent(cx: Scope, name: String) -> Element {
    render!(
        div {
            onmouseenter: move |event| {
                println!("{:?}", event)
            },
            "Hallo {name}, wie geht es dir?"
        }
    )
}
