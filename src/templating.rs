use maud::{DOCTYPE, html, Markup};
use chrono::Utc;
pub struct TemplateHeaderData<'a> {
    title: &'a str,
    styles: Option<Vec<&'a str>>,
    open_graph: Option<TemplateOpenGraphData<'a>>,
}

pub struct TemplateOpenGraphData<'a> {
    title: &'a str,
    description: &'a str,
    image: Option<&'a str>,
    url: Option<&'a str>,
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
            meta charset="UTF-8";
            title { (data.title) }
            link rel="icon" type="image/x-icon" href=(fetch_timed_assets("/assets/favicon.ico"));
            link rel="icon" type="image/png" href=(fetch_timed_assets("/assets/favicon.png"));
            meta name="viewport" content="width=device-width, initial-scale=1";
            @if let Some(open_graph) = &data.open_graph {
                meta property="og:title" content=(open_graph.title);
                meta property="og:description" content=(open_graph.description);
                @if let Some(image) = open_graph.image {
                    meta property="og:image" content=(image);
                }
                @if let Some(url) = open_graph.url {
                    meta property="og:url" content=(url);
                }
            }
            
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

pub fn navbar(current_path: &str) -> Markup {
    html! {
        nav {
            ul {
                li class=(if current_path == "/" { "active" } else { "" }) {
                    a href="/" { "Home" }
                }
                li class=(if current_path == "/about" { "active" } else { "" }) {
                    a href="/about" { "About" }
                }
            }
        }
    }
}

pub fn render_content(
    current_path: &str,
    header:&TemplateHeaderData,
    body: Markup,
) -> Markup {
    html! {
        (DOCTYPE)
        html {
            (render_header(header))
            body {
                (navbar(&current_path))
                (body)
            }
        }
    }
}


pub fn render_home(current_path: &str) -> Markup {
    render_content(current_path, &TemplateHeaderData{
        title: "Jordan",
        styles: None,
        open_graph: Some(TemplateOpenGraphData{
            title: "Jordan",
            description: "This is the home page.",
            image: None,
            url: None,
        })
    }, html! {
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
    })
}

pub fn render_about(current_path: &str) -> Markup {
    render_content(current_path, &TemplateHeaderData{
        title: "About Me",
        styles: None,
        open_graph: Some(TemplateOpenGraphData{
            title: "About Me",
            description: "This is the about me page.",
            image: None,
            url: None,
        })
    }, html! {
        article {
            h1 { "About Us" }
            p { "This is the about page." }
        }
    })
}