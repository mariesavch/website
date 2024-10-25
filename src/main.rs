#![allow(non_snake_case)]

use async_std::task::sleep;
use dioxus::prelude::*;
use serde::Deserialize;
use web_time::Instant;

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

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
        main { class: "mx-auto max-w-3xl px-6 pb-20",
            div { class: "pt-16",
                div {
                    h1 { class: "mb-2 text-xl font-bold", "marie" }
                    div { class: "text-overlay0 mb-5",
                        if let Some(Ok(data)) = weather.read().as_ref() {
                            p {
                                "she/her, {data.main.temp.round()}°C {data.weather[0].description.as_str()}, {time}"
                            }
                        } else {
                            p { "loading..." }
                        }
                    }
                }
                ul { class: "animated-list grid grid-cols-1 sm:grid-cols-2",
                    li {
                        div { class: "flex py-3 flex-col gap-1",
                            span { class: "text-overlay0", "github" }
                            a {
                                class: "underlined",
                                href: "https://github.com/mariesavch",
                                "mariesavch"
                            }
                        }
                    }

                    li {
                        div { class: "flex py-3 flex-col gap-1",
                            span { class: "text-overlay0", "email" }
                            a {
                                class: "underlined",
                                href: "mailto:mariesavch@icloud.com",
                                "mariesavch@icloud.com"
                            }
                        }
                    }
                }
            }
        }
    }
}
