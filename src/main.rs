use dioxus::prelude::*;
use dioxus_rsx_rosetta::Dom;

const MAIN_CSS: Asset = asset!("/public/tailwind.css");

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    let mut html_input = use_signal(String::new);
    let mut rsx_output = use_signal(|| "Your Generated RSX".to_string());

    rsx! {
        link { rel: "stylesheet", href: MAIN_CSS }
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
                        onclick: move |_| {
                            spawn(async move {
                                let dom = Dom::parse(&html_input()).unwrap();
                                let rsx_response_callbody = dioxus_rsx_rosetta::rsx_from_html(&dom);
                                rsx_output
                                    .set(dioxus_autofmt::write_block_out(&rsx_response_callbody).unwrap());
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
