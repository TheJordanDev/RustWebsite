use maud::{html, Markup};
use chrono::Utc;
pub struct TemplateHeaderData<'a> {
    title: &'a str,
    styles: Option<Vec<&'a str>>,
}

pub fn render_header(data:&TemplateHeaderData) -> Markup {
    let timestamp = Utc::now().timestamp();
    html! {
        head {
            meta charset="UTF-8";
            title { (data.title) }
            link href="https://fonts.googleapis.com/css?family=Open+Sans:300,400,700" rel="stylesheet";
            link rel="stylesheet" href=(format!("{}?v={}", "/assets/css/styles.css", timestamp))
            meta name="viewport" content="width=device-width, initial-scale=1";
            @if let Some(styles) = &data.styles {
                @for style in styles {
                    link rel="stylesheet" type="text/css" href=(format!("{}?v={}", style, timestamp));
                }
            }
        }
    }
}

pub fn render_home() -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            (render_header(&TemplateHeaderData{title: "Home", styles: None}))
            body {
                article {
                    img src="/assets/images/warning.svg" alt="Warning" class="svg";
                    h1 {
                        "We'll be back soon!"
                        br;
                        "Nous serons bientôt de retour!"
                    };
                    div {
                        p {
                            "This page is currently being created, come back later !"
                            br;
                            "Cette page est actuellement en cours de création, revenez plus tard !"
                        }
                    }
                    p class="signature" {
                        "- Jordan"
                    }
                }
            }
        }
    }
}

pub fn render_about() -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            (render_header(&TemplateHeaderData{title: "About", styles: None}))
            body {
                h1 { "About Us" }
                p { "This is the about page." }
            }
        }
    }
}