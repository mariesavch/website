#![allow(non_snake_case)]

use async_std::task::sleep;
use dioxus::prelude::*;
use serde::Deserialize;
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

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
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
                    div { class: "item",
                        span { color: "var(--overlay0)", "github" }
                        a {
                            class: "underlined",
                            href: "https://github.com/mariesavch",
                            "mariesavch"
                        }
                    }
                }

                li {
                    div { class: "item",
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
                    div { class: "item",
                        span { color: "var(--overlay0)", "view your weather" }
                        a {
                            class: "underlined",
                            href: "https://wtrs.vercel.app",
                            "weather"
                        }
                    }
                }
                li {
                    div { class: "item",
                        span { color: "var(--overlay0)", "simple todo app" }
                        a {
                            class: "underlined",
                            href: "https://tdwr.vercel.app",
                            "todo"
                        }
                    }
                }
                li {
                    div { class: "item",
                        span { color: "var(--overlay0)", "information about countries" }
                        a {
                            class: "underlined",
                            href: "https://cntrn.vercel.app",
                            "countryinfo"
                        }
                    }
                }
                li {
                    div { class: "item",
                        span { color: "var(--overlay0)", "cli information about countries" }
                        a {
                            class: "underlined",
                            href: "https://github.com/mariesavch/countryfetch-rs",
                            "countryfetch-rs"
                        }
                    }
                }
                li {
                    div { class: "item",
                        span { color: "var(--overlay0)", "information about ip's" }
                        a {
                            class: "underlined",
                            href: "https://ipinf.vercel.app",
                            "ipinfo"
                        }
                    }
                }
                li {
                    div { class: "item",
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
