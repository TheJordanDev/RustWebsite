use maud::{html, Markup};

pub fn render_home() -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                title { "Home" }
            }
            body {
                h1 { "Welcome to the Home Page" }
            }
        }
    }
}