use maud::{html, Markup};
use chrono::Utc;
pub struct TemplateHeaderData<'a> {
    title: &'a str,
    styles: Option<Vec<&'a str>>,
}

fn fetch_assets<S: AsRef<str>>(path: S) -> String {
    let timestamp = Utc::now().timestamp();
    format!("{}?v={}", path.as_ref(), timestamp)
}

pub fn render_header(data:&TemplateHeaderData) -> Markup {
    html! {
        head {
            meta charset="UTF-8";
            title { (data.title) }
            link href="https://fonts.googleapis.com/css?family=Open+Sans:300,400,700" rel="stylesheet";
            link rel="stylesheet" href=(fetch_assets("/assets/css/styles.css"))
            meta name="viewport" content="width=device-width, initial-scale=1";
            @if let Some(styles) = &data.styles {
                @for style in styles {
                    link rel="stylesheet" type="text/css" href=(fetch_assets(format!("/assets/css/{}", style)));
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
                    img src=(fetch_assets("/assets/images/warning.svg")) alt="Warning" class="svg";
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