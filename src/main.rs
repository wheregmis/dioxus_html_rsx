//! Run with:
//!
//! ```sh
//! dx serve --platform fullstack
//! ```

use dioxus::prelude::*;
#[cfg(feature = "server")]
use std::fs::File;
#[cfg(feature = "server")]
use std::io::Write;
#[cfg(feature = "server")]
use std::process::Command;

const STYLE: &str = manganis::mg!("public/tailwind.css");

fn main() {
    LaunchBuilder::fullstack()
        .with_cfg(server_only!(ServeConfig::builder().incremental(
            IncrementalRendererConfig::default()
                .invalidate_after(std::time::Duration::from_secs(120)),
        )))
        .launch(app);
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    let mut html_input = use_signal(|| String::new());
    let mut rsx_output = use_signal(|| "Your Generated RSX".to_string());

    rsx! {
        head::Link { rel: "stylesheet", href: STYLE }
        div { style: "flex flex-col items-center justify-center h-screen",
            div { class: "flex flex-row items-center justify-center w-full",
                div { class: "flex flex-col items-center justify-center w-full h-screen",
                    textarea {
                        value: "{html_input}",
                        oninput: move |e| html_input.set(e.value().clone()),
                        placeholder: "Your HTML Code",
                        class: "textarea textarea-accent w-full flex-grow"
                    }
                    button {
                        class: "btn m-5",
                        onclick: move |event| {
                            spawn(async move {
                                println!("Clicked! {event:?}");
                                let rsx_response = post_server_data(html_input()).await;
                                match rsx_response {
                                    Ok(rsx_out) => {
                                        rsx_output.set(rsx_out);
                                    }
                                    Err(_) => {
                                        rsx_output.set("Error!".to_string());
                                    }
                                }
                            });
                        },
                        "Convert HTML to RSX"
                    }
                }
                textarea {
                    value: "{rsx_output}",
                    // Assuming rsx_output is a mutable state similar to html_input,
                    // and you want to allow editing the RSX output. If not, you can remove the oninput handler.
                    oninput: move |e| rsx_output.set(e.value().clone()),
                    class: "textarea textarea-accent w-full h-screen"
                }
            }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<String, ServerFnError> {
    println!("Server received: {}", data);

    let output = Command::new("dx")
        .arg("translate")
        .arg("--raw")
        .arg(data) // Or pass the HTML string directly if supported
        .output()?;

    // Step 4: Process the output
    if output.status.success() {
        let rsx_output = String::from_utf8_lossy(&output.stdout);
        println!("RSX Output: {}", rsx_output);
        Ok(rsx_output.to_string())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", error_message);
        Err(ServerFnError::new("Error converting HTML to RSX"))
    }
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
