use maud::{DOCTYPE, html, Markup};
use chrono::Utc;
pub struct TemplateHeaderData<'a> {
    title: &'a str,
    styles: Option<Vec<&'a str>>,
}

// fn fetch_assets<S: AsRef<str>>(path: S) -> String {
//     format!("{}", path.as_ref())
// }

fn fetch_timed_assets<S: AsRef<str>>(path: S) -> String {
    let timestamp = Utc::now().timestamp();
    format!("{}?t={}", path.as_ref(), timestamp)
}

pub fn render_header(data:&TemplateHeaderData) -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta charset="UTF-8";
            title { (data.title) }
            meta name="viewport" content="width=device-width, initial-scale=1";
            link href="https://fonts.googleapis.com/css?family=Open+Sans:300,400,700" rel="stylesheet";
            script src="https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js" crossorigin="anonymous" referrerpolicy="no-referrer" {}
            link rel="stylesheet" href=(fetch_timed_assets("/assets/css/styles.css"));
            @if let Some(styles) = &data.styles {
                @for style in styles {
                    link rel="stylesheet" type="text/css" href=(fetch_timed_assets(format!("/assets/css/{}", style)));
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
                    img.svg src=(fetch_timed_assets("/assets/images/warning.svg")) alt="Warning";
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
                    p.signature {
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
                article {
                    h1 { "About Us" }
                    p { "This is the about page." }
                }
            }
        }
    }
}