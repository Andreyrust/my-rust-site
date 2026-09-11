use dioxus::prelude::*;

fn main() {
    // Запускаємо наш додаток
    dioxus::launch(App);
}

// Це головний компонент нашого сайту (його зовнішній вигляд)
#[component]
fn App() -> Element {
    rsx! {
        div {
            style: "text-align: center; font-family: sans-serif; margin-top: 100px;",
            h1 { "Мій перший сайт на Rust! 🦀" }
            p { "Вітаю! Якщо ви бачите цей текст, значить все працює правильно." }
        }
    }
}
