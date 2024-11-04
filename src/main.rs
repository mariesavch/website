#![allow(non_snake_case)]

use async_std::task::sleep;
use dioxus::prelude::*;
use serde::Deserialize;
use sir::{css, global_css, AppStyle};
use web_time::Instant;

fn main() {
    dioxus::launch(App);
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeatherData {
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Main {
    pub temp: f32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Weather {
    pub description: String,
}

async fn get_weather() -> reqwest::Result<WeatherData> {
    reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=neftekamsk&units=metric&cnt=8&appid=7484f462f852c04cbab6a6a5ad8c9d37")
        .await?
        .json::<WeatherData>()
        .await
}

#[component]
fn App() -> Element {
    let weather = use_resource(move || async move { get_weather().await });
    let mut millis = use_signal(|| 0);

    use_future(move || async move {
        let start = Instant::now();
        loop {
            sleep(std::time::Duration::from_millis(100)).await;
            millis.set(start.elapsed().as_millis() as i64);
        }
    });
    let time = format!(
        "{:02}:{:02}:{:03}",
        millis() / 1000 / 60 % 60,
        millis() / 1000 % 60,
        millis() % 1000
    );

    global_css!(
        "
       :root {
        --rosewater: #ff8389;
        --flamingo: #ff8389;
        --red: #ff8389;
        --maroon: #ff8389;
        --pink: #ff7eb6;
        --mauve: #be95ff;
        --peach: #d44a1c;
        --yellow: #ab8600;
        --green: #08bdba;
        --teal: #33b1ff;
        --sky: #33b1ff;
        --sapphire: #33b1ff;
        --blue: #78a9ff;
        --lavender: #78a9ff;
        --text: #ffffff;
        --subtext1: #f4f4f4;
        --subtext0: #e0e0e0;
        --overlay2: #adadad;
        --overlay1: #949494;
        --overlay0: #7a7a7a;
        --surface2: #4f4f4f;
        --surface1: #383838;
        --surface0: #2e2e2e;
        --base: #161616;
        --mantle: #0d0d0d;
        --crust: #000000;
    } 

    @media (prefers-color-scheme: light) {
        :root {
            --rosewater: #da1e28;
            --flamingo: #da1e28;
            --red: #da1e28;
            --maroon: #da1e28;
            --pink: #d02670;
            --mauve: #8a3ffc;
            --peach: #d44a1c;
            --yellow: #ab8600;
            --green: #007d79;
            --teal: #1192e8;
            --sky: #1192e8;
            --sapphire: #1192e8;
            --blue: #0f62fe;
            --lavender: #0f62fe;
            --text: #000000;
            --subtext1: #404040;
            --subtext0: #474747;
            --overlay2: #575757;
            --overlay1: #595959;
            --overlay0: #737373;
            --surface2: #8c8c8c;
            --surface1: #d1d1d1;
            --surface0: #e6e6e6;
            --base: #ffffff;
            --mantle: #f2f2f2;
            --crust: #ebebeb;
        }
    }

    :root {
        background-color: var(--base);
        color: var(--text);
        line-height: 1.6;
    }

    .underlined {
        all: unset;
        text-decoration-line: underline; 
        text-decoration-thickness: 2px; 
        text-underline-offset: 4px; 
        transition-property: color, background-color, border-color, text-decoration-color, fill, stroke; 
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1); 
        transition-duration: 300ms;
        text-decoration-color: var(--surface2);
    }

    .underlined:hover {
        text-decoration-color: var(--overlay2);
    }

    .underlined:active {
        text-decoration-color: var(--overlay1);
    }

    @media (hover: hover) and (pointer: fine) {
        .animated-list li {
            all: unset;
            transition-property: all;
            transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
            transition-duration: 300ms;
        }
        .animated-list:hover li {
            opacity: 0.5;
        }
        .animated-list:hover li:hover {
            opacity: 1;
        }
    }
    "
    );

    let item = css!(
        "
        display: flex;
        flex-direction: column;
        padding-top: 12px;
        padding-bottom: 12px;
        gap: 4px;"
    );

    rsx! {
        AppStyle {}
        main {
            margin_left: "auto",
            margin_right: "auto",
            max_width: "768px",
            padding_top: "64px",
            padding_left: "24px",
            padding_right: "24px",
            padding_bottom: "80px",
            div {
                h1 { margin_bottom: "4px", font_size: "20px", "marie" }
                div { margin_bottom: "20px", color: "var(--overlay0)",
                    if let Some(Ok(data)) = weather.read().as_ref() {
                        p {
                            "she/her, {data.main.temp.round()}°C {data.weather[0].description.as_str()}, {time}"
                        }
                    }
                }
            }
            ul {
                class: "animated-list",
                all: "unset",
                display: "grid",
                grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "github" }
                        a {
                            class: "underlined",
                            href: "https://github.com/mariesavch",
                            "mariesavch"
                        }
                    }
                }

                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "email" }
                        a {
                            class: "underlined",
                            href: "mailto:mariesavch@icloud.com",
                            "mariesavch@icloud.com"
                        }
                    }
                }
            }
            h2 { font_size: "20px", margin_bottom: "8px", margin_top: "32px", "projects" }
            ul {
                class: "animated-list",
                all: "unset",
                display: "grid",
                grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "view your weather" }
                        a {
                            class: "underlined",
                            href: "https://wtrs.vercel.app",
                            "weather"
                        }
                    }
                }
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "simple todo app" }
                        a {
                            class: "underlined",
                            href: "https://tdwr.vercel.app",
                            "todo"
                        }
                    }
                }
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "information about countries" }
                        a {
                            class: "underlined",
                            href: "https://cntrn.vercel.app",
                            "countryinfo"
                        }
                    }
                }
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "cli information about countries" }
                        a {
                            class: "underlined",
                            href: "https://github.com/mariesavch/countryfetch-rs",
                            "countryfetch-rs"
                        }
                    }
                }
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "information about ip's" }
                        a {
                            class: "underlined",
                            href: "https://ipinf.vercel.app",
                            "ipinfo"
                        }
                    }
                }
                li {
                    div { class: item,
                        span { color: "var(--overlay0)", "track postal shipments" }
                        a {
                            class: "underlined",
                            href: "https://pstr.vercel.app",
                            "tracking"
                        }
                    }
                }
            }
        }
    }
}
