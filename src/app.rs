use crate::{
    icon::*,
    parser::{parse_process, ProcessDetail},
};
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx!(
        div {
            class: "bg-[#000212] bg-[radial-gradient(ellipse_at_center,rgba(0,225,244,0.15),hsla(0,0%,100%,0))] text-[#f7f8f8]",
            // Sidebar
            SideBar {}
            // Content
            MainContent {}
        }
    )
}

pub fn SideBar() -> Element {
    rsx!(
        div {
            class: "h-screen fixed w-60 z-[2] backdrop-blur-md bg-[rgba(255,255,255,0.1)] border-r border-solid border-[rgba(255,255,255,0.05)] left-0 top-0",
            nav {
                class: "flex flex-col gap-4 p-4",
                h2 {
                    class: "font-medium",
                    "Port Manager"
                }
                a {
                    class: "inline-flex items-center w-full bg-[rgba(255,255,255,0.1)] border-r border-solid border-[rgba(255,255,255,0.05)] text-xs px-4 py-2 text-center rounded-md",
                    "Changelog"
                }
            }
        }
    )
}

pub fn MainContent() -> Element {
    let proc_list = parse_process();
    let mut search_input = use_signal(|| "".to_string());

    rsx!(
        div {
            class: "flex flex-col min-h-screen min-w-[1040px] ml-60",
            div {
                class: "flex flex-col w-full border-b border-[rgba(255,255,255,0.15)]",
                div {
                    class: "px-6 pt-7 pb-4 max-w-80",
                    div {
                        class: "relative flex items-center",
                        SearchIcon {}
                        input {
                            r#type: "text",
                            placeholder: "Search...",
                            autocomplete: "off",
                            value: "{search_input}",
                            name: "search_input",
                            class: "block w-full h-10 rounded-md bg-[rgba(255,255,255,0.1)] border-r border-solid border-[rgba(255,255,255,0.05)] p-2 pl-10 pr-2 focus:border-[rgba(255,255,255,0.1)] focus:outline-none focus:ring-0 sm:text-sm",
                            oninput: move |evt| search_input.set(evt.value()),
                        }
                    }
                }
            }
            div {
                class: "grid grid-cols-[repeat(3,1fr)_clamp(230px,20%,270px)] w-full h-[calc(100vh_-_85px)]",
                div {
                    class: "col-span-3 border-r border-[rgba(255,255,255,0.15)]",
                    div {
                        class: "p-6",
                        if !search_input.to_string().is_empty() {
                            SearchResult{lsof_list: proc_list.clone(), search_text: search_input.to_string()}
                        } else {
                            ProcessList {lsof_list: proc_list.clone()}
                        }
                    }
                }
                div {
                    class: "col-span-1",
                    div {
                        class: "flex flex-col gap-4 p-4",
                        h2 {
                            class: "font-medium",
                            "Getting started guide"
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn ProcessList(lsof_list: Vec<ProcessDetail>) -> Element {
    rsx!(
        div {
            class: "relative overflow-x-auto rounded-lg border border-[rgba(255,255,255,0.05)]",
            table {
                class: "w-full text-sm text-left rtl:text-right",
                thead { class: "bg-[rgba(255,255,255,0.1)] border-b border-[rgba(255,255,255,0.05)]",
                    tr {
                        th { class: "px-6 py-3 font-medium", "Process Name" }
                        th { class: "px-6 py-3 font-medium", "PID" }
                        th { class: "px-6 py-3 font-medium", "Type" }
                        th { class: "px-6 py-3 font-medium", "Port" }
                        th { class: "px-6 py-3 font-medium", "" }
                    }
                }
                tbody {
                    class: "text-sm",
                    if !lsof_list.is_empty() {
                        for (i, item) in lsof_list.into_iter().enumerate() {
                            tr { class: "border-b border-[rgba(255,255,255,0.05)]", key: "{i}",
                                td { class: "px-6 py-3 whitespace-nowrap", "{item.process_name}" }
                                td { class: "px-6 py-3 whitespace-nowrap", "{item.pid}" }
                                td { class: "px-6 py-3 whitespace-nowrap", "{item.node}" }
                                td { class: "px-6 py-3 whitespace-nowrap", "{item.name}" }
                                td { class: "px-6 py-3",
                                    a { href: "/", DeleteIcon {} }
                                }
                            }
                        }
                    } else {
                        div { class: "flex items-center justify-center",
                            h4 { class: "font-medium", "Empty process list" }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn SearchResult(lsof_list: Vec<ProcessDetail>, search_text: String) -> Element {
    let item = lsof_list.iter().find(|x| x.name == search_text);
    rsx!(
        div {
            class: "relative overflow-x-auto rounded-lg border border-[rgba(255,255,255,0.05)]",
            table {
                class: "w-full text-sm text-left rtl:text-right",
                thead { class: "bg-[rgba(255,255,255,0.1)] border-b border-[rgba(255,255,255,0.05)]",
                    tr {
                        th { class: "px-6 py-3 font-medium", "Process Name" }
                        th { class: "px-6 py-3 font-medium", "PID" }
                        th { class: "px-6 py-3 font-medium", "Type" }
                        th { class: "px-6 py-3 font-medium", "Port" }
                        th { class: "px-6 py-3 font-medium", "" }
                    }
                }
                tbody {
                    class: "text-sm",
                    if let Some(lsof) = item {
                        tr {
                            class: "border-b border-[rgba(255,255,255,0.05)]",
                            td { class: "px-6 py-3 whitespace-nowrap", "{lsof.process_name}" }
                            td { class: "px-6 py-3 whitespace-nowrap", "{lsof.pid}" }
                            td { class: "px-6 py-3 whitespace-nowrap", "{lsof.node}" }
                            td { class: "px-6 py-3 whitespace-nowrap", "{lsof.name}" }
                            td {
                                class: "px-6 py-3",
                                a {
                                    href: "/",
                                    DeleteIcon {}
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
