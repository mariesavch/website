#![allow(non_snake_case)]

use async_std::task::sleep;
use css_in_rs::{make_styles, use_style_provider_quickstart, Classes, EmptyTheme};
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

make_styles! {
    (_theme: EmptyTheme) -> MyClasses {
        ":root" {
            __rosewater: "#ff8389",
            __flamingo: "#ff8389",
            __red: "#ff8389",
            __maroon: "#ff8389",
            __pink: "#ff7eb6",
            __mauve: "#be95ff",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#08bdba",
            __teal: "#33b1ff",
            __sky: "#33b1ff",
            __sapphire: "#33b1ff",
            __blue: "#78a9ff",
            __lavender: "#78a9ff",
            __text: "#ffffff",
            __subtext1: "#f4f4f4",
            __subtext0: "#e0e0e0",
            __overlay2: "#adadad",
            __overlay1: "#949494",
            __overlay0: "#7a7a7a",
            __surface2: "#4f4f4f",
            __surface1: "#383838",
            __surface0: "#2e2e2e",
            __base: "#161616",
            __mantle: "#0d0d0d",
            __crust: "#000000",
        },
    "@media (prefers-color-scheme: light)" {
        ":root" {
            __rosewater: "#da1e28",
            __flamingo: "#da1e28",
            __red: "#da1e28",
            __maroon: "#da1e28",
            __pink: "#d02670",
            __mauve: "#8a3ffc",
            __peach: "#d44a1c",
            __yellow: "#ab8600",
            __green: "#007d79",
            __teal: "#1192e8",
            __sky: "#1192e8",
            __sapphire: "#1192e8",
            __blue: "#0f62fe",
            __lavender: "#0f62fe",
            __text: "#000000",
            __subtext1: "#404040",
            __subtext0: "#474747",
            __overlay2: "#575757",
            __overlay1: "#595959",
            __overlay0: "#737373",
            __surface2: "#8c8c8c",
            __surface1: "#d1d1d1",
            __surface0: "#e6e6e6",
            __base: "#ffffff",
            __mantle: "#f2f2f2",
            __crust: "#ebebeb",
            }
        },
        ":root" {
            background_color: "var(--base)",
            color: "var(--text)",
            line_height: "1.6",
        },
        "@media (hover: hover) and (pointer: fine)" {
            ".animated_list li" {
                all: "unset",
                transition_property: "all",
                transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
                transition_duration: "300ms",
            },
            ".animated_list:hover li" {
                opacity: "0.5",
            },
            ".animated_list:hover li:hover" {
                opacity: "1",
            }
        },
        ".item" {
            display: "flex",
            flex_direction: "column",
            padding_top: "12px",
            padding_bottom: "12px",
            gap: "4px",
        },
        ".underlined" {
            color: "unset",
            text_decoration_line: "underline",
            text_decoration_thickness: "2px",
            text_underline_offset: "4px",
            transition_property: "color, background-color, border-color, text-decoration-color, fill, stroke",
            transition_timing_function: "cubic-bezier(0.4, 0, 0.2, 1)",
            transition_duration: "300ms",
            text_decoration_color: "var(--surface2)",
        },
        ".underlined:hover" {
            text_decoration_color: "var(--overlay2)",
        },
        ".underlined:active" {
            text_decoration_color: "var(--overlay1)",
        }
    }
}

#[component]
fn App() -> Element {
    use_style_provider_quickstart(|| EmptyTheme);
    let cls: &MyClasses = &MyClasses::use_style();

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
                class: &cls.animated_list as &str,
                all: "unset",
                display: "grid",
                grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "github" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://github.com/mariesavch",
                            "mariesavch"
                        }
                    }
                }

                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "email" }
                        a {
                            class: &cls.underlined as &str,
                            href: "mailto:mariesavch@icloud.com",
                            "mariesavch@icloud.com"
                        }
                    }
                }
            }
            h2 { font_size: "20px", margin_bottom: "8px", margin_top: "32px", "projects" }
            ul {
                class: &cls.animated_list as &str,
                all: "unset",
                display: "grid",
                grid_template_columns: "repeat(auto-fit, minmax(300px, 1fr))",
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "view your weather" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://wtrs.vercel.app",
                            "weather"
                        }
                    }
                }
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "simple todo app" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://tdwr.vercel.app",
                            "todo"
                        }
                    }
                }
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "information about countries" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://cntrn.vercel.app",
                            "countryinfo"
                        }
                    }
                }
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "cli information about countries" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://github.com/mariesavch/countryfetch-rs",
                            "countryfetch-rs"
                        }
                    }
                }
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "information about ip's" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://ipinf.vercel.app",
                            "ipinfo"
                        }
                    }
                }
                li {
                    div { class: &cls.item as &str,
                        span { color: "var(--overlay0)", "track postal shipments" }
                        a {
                            class: &cls.underlined as &str,
                            href: "https://pstr.vercel.app",
                            "tracking"
                        }
                    }
                }
            }
        }
    }
}
