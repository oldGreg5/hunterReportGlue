use yew::prelude::*;
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;

#[derive(Debug, Clone, PartialEq)]
struct Item {
    name: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Category {
    name: String,
    items: Vec<Item>,
}

#[function_component(App)]
fn app() -> Html {
    let categories = use_state(|| Vec::<Category>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None as Option<String>);
    let raw_text = use_state(|| None as Option<String>);

    {
        let categories = categories.clone();
        let loading = loading.clone();
        let error = error.clone();
        let raw_text = raw_text.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("/data/HunterWyniki.txt").send().await;
                match resp {
                    Ok(resp) => {
                        if let Ok(txt) = resp.text().await {
                            raw_text.set(Some(txt.clone()));
                            let parsed = parse_description_file(&txt);
                            categories.set(parsed);
                            loading.set(false);
                        } else {
                            error.set(Some("Failed to read file text".to_string()));
                            loading.set(false);
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch file: {}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }

    html! {
        <div style="max-width: 1000px; margin: 40px auto; font-family: sans-serif;">
            <h1>{"Hunter Report Generator (Rust/Yew)"}</h1>
            if *loading {
                <p>{"Loading descriptions..."}</p>
            } else if let Some(err) = &*error {
                <p style="color: red;">{format!("Error: {}", err)}</p>
            } else {
                <div>
                    <h2>{"Categories loaded:"}</h2>
                    <ul>
                        {for categories.iter().map(|cat| html!{
                            <li>{format!("{} ({} items)", cat.name, cat.items.len())}</li>
                        })}
                    </ul>
                    <details style="margin-top: 2em;">
                        <summary>{"Debug: Raw file content"}</summary>
                        <pre style="background:#eee; padding:1em; max-height:300px; overflow:auto;">{raw_text.as_deref().unwrap_or("(none)")}</pre>
                    </details>
                    <details style="margin-top: 1em;">
                        <summary>{"Debug: Parsed categories"}</summary>
                        <pre style="background:#eee; padding:1em; max-height:300px; overflow:auto;">{format!("{:#?}", &*categories)}</pre>
                    </details>
                </div>
            }
        </div>
    }
}

fn parse_description_file(text: &str) -> Vec<Category> {
    let mut categories = Vec::new();
    let parts = text.split(">>").skip(1);
    for part in parts {
        let lines = part.split('^').skip(1);
        let name = part.split('^').next().unwrap_or("").trim().to_string();
        let mut items = Vec::new();
        for line in lines {
            let mut split = line.split('$');
            let item_name = split.next().unwrap_or("").trim().to_string();
            let item_desc = split.next().unwrap_or("").trim().to_string();
            if !item_name.is_empty() {
                items.push(Item { name: item_name, description: item_desc });
            }
        }
        if !name.is_empty() && !items.is_empty() {
            categories.push(Category { name, items });
        }
    }
    categories
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
