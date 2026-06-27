use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Item {
    name: String,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Category {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WizardState {
    full_name: String,
    test_date: String,
    manual_notes: String,
    burdens: Vec<String>,
    burdens_extra: String,
    toxic_metals: String,
    deficiencies: String,
    frequencies: String,
    selected_pathogens: Vec<String>,
    recommendations: String,
}

impl Default for WizardState {
    fn default() -> Self {
        let today = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();
        let date_only = today.get(0..10).unwrap_or("").to_string();
        Self {
            full_name: "".to_string(),
            test_date: date_only,
            manual_notes: "".to_string(),
            burdens: vec![],
            burdens_extra: "".to_string(),
            toxic_metals: "".to_string(),
            deficiencies: "".to_string(),
            frequencies: "".to_string(),
            selected_pathogens: vec![],
            recommendations: "".to_string(),
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let categories = use_state(|| Vec::<Category>::new());
    let wizard_state = use_state(WizardState::default);
    let current_step = use_state(|| 0usize);
    let search_text = use_state(|| "".to_string());
    let loading = use_state(|| true);
    let app_mode = use_state(|| 0usize);
    let dark_mode = use_state(|| false);

    {
        let categories = categories.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("/hunterReport/HunterWyniki.txt").send().await;
                if let Ok(resp) = resp {
                    if let Ok(txt) = resp.text().await {
                        categories.set(parse_description_file(&txt));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let on_prev = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                current_step.set(*current_step - 1);
            }
        })
    };

    let on_next = {
        let current_step = current_step.clone();
        let wizard_state = wizard_state.clone();
        Callback::from(move |_| {
            if *current_step == 0 {
                if wizard_state.full_name.trim().is_empty() || wizard_state.test_date.trim().is_empty() {
                    gloo::dialogs::alert("Imię i nazwisko oraz data są wymagane");
                    return;
                }
            }
            if *current_step < 7 {
                current_step.set(*current_step + 1);
            }
        })
    };

    let on_reset = {
        let current_step = current_step.clone();
        let wizard_state = wizard_state.clone();
        Callback::from(move |_| {
            wizard_state.set(WizardState::default());
            current_step.set(0);
        })
    };

    let on_toggle_dark = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let new_val = !*dark_mode;
            dark_mode.set(new_val);
            if let Some(body) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.body())
            {
                if new_val { let _ = body.class_list().add_1("dark"); }
                else       { let _ = body.class_list().remove_1("dark"); }
            }
        })
    };

    let on_mode_wizard = {
        let app_mode = app_mode.clone();
        Callback::from(move |_: MouseEvent| app_mode.set(0))
    };
    let on_mode_patogeny = {
        let app_mode = app_mode.clone();
        Callback::from(move |_: MouseEvent| app_mode.set(1))
    };

    let on_copy = {
        let wizard_state = wizard_state.clone();
        let categories = categories.clone();
        Callback::from(move |_: MouseEvent| {
            let selected = (*wizard_state).selected_pathogens.clone();
            let desc_map: std::collections::HashMap<String, String> = (*categories).iter()
                .flat_map(|c| c.items.iter())
                .map(|i| (i.name.clone(), i.description.clone()))
                .collect();

            let mut html_parts = Vec::new();
            let mut text_parts = Vec::new();
            for name in &selected {
                let desc = desc_map.get(name).cloned().unwrap_or_default();
                if desc.is_empty() {
                    html_parts.push(format!("<p><b>{}</b></p>", escape_html(name)));
                    text_parts.push(name.clone());
                } else {
                    html_parts.push(format!("<p><b>{}</b> - {}</p>", escape_html(name), escape_html(&desc)));
                    text_parts.push(format!("{name} - {desc}"));
                }
            }
            let html_content = html_parts.join("");
            let text_content = text_parts.join("\n");

            let write_fn = js_sys::Function::new_with_args(
                "htmlContent, textContent",
                r#"
                return (async () => {
                    const htmlBlob = new Blob([htmlContent], { type: 'text/html' });
                    const textBlob = new Blob([textContent], { type: 'text/plain' });
                    const item = new ClipboardItem({ 'text/html': htmlBlob, 'text/plain': textBlob });
                    await navigator.clipboard.write([item]);
                })();
                "#,
            );
            let html_val = wasm_bindgen::JsValue::from_str(&html_content);
            let text_val = wasm_bindgen::JsValue::from_str(&text_content);
            if let Ok(promise) = write_fn.call2(&wasm_bindgen::JsValue::NULL, &html_val, &text_val) {
                if let Ok(promise) = promise.dyn_into::<js_sys::Promise>() {
                    wasm_bindgen_futures::spawn_local(async move {
                        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                    });
                }
            }
        })
    };

    let on_download = {
        let wizard_state = wizard_state.clone();
        Callback::from(move |_| {
            let state = (*wizard_state).clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/hunterReport/api/docx")
                    .json(&state)
                    .unwrap()
                    .send()
                    .await;
                if let Ok(resp) = resp {
                    if resp.ok() {
                        let blob = resp.binary().await.unwrap();
                        let array = js_sys::Uint8Array::from(&blob[..]);
                        let parts = js_sys::Array::new();
                        parts.push(&array);
                        let blob_obj = web_sys::Blob::new_with_u8_array_sequence(&parts).unwrap();
                        let url = web_sys::Url::create_object_url_with_blob(&blob_obj).unwrap();
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        let a = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                        let file_name = format!("{}_Vega_Test_{}.docx", state.full_name, state.test_date)
                            .replace(" ", "_");
                        a.set_href(&url);
                        a.set_download(&file_name);
                        document.body().unwrap().append_child(&a).unwrap();
                        a.click();
                        document.body().unwrap().remove_child(&a).unwrap();
                        web_sys::Url::revoke_object_url(&url).unwrap();
                    } else {
                        gloo::dialogs::alert("Nie udało się wygenerować raportu");
                    }
                }
            });
        })
    };

    let steps = vec![
        "Vega Test",
        "Zgłaszane dolegliwości",
        "Wskazanie na obciążenia",
        "Metale toksyczne",
        "Niedobory",
        "Częstotliwości",
        "Opisy patogenów",
        "Zalecenia",
    ];

    let step_icons = vec![
        "👤", "📋", "⚖️", "🧪", "💊", "〰️", "🔬", "📝",
    ];

    let step = *current_step;

    html! {
        <>
            <header>
                <div id="nav-bar">
                    <div class="header-brand">
                        <span class="header-logo">{"🌿"}</span>
                        <div class="header-text">
                            <h1>{"Hunter Report"}</h1>
                            <p class="header-subtitle">{"Generator Raportów Naturopatycznych"}</p>
                        </div>
                    </div>
                    <div class="header-controls">
                        <button
                            class="dark-toggle"
                            onclick={on_toggle_dark}
                            title={if *dark_mode { "Tryb jasny" } else { "Tryb ciemny" }}
                        >{if *dark_mode { "☀" } else { "🌙" }}</button>
                        <div class="mode-switch">
                            <button
                                class={if *app_mode == 0 { "mode-btn mode-btn--active" } else { "mode-btn" }}
                                onclick={on_mode_wizard}
                            >{"Kreator"}</button>
                            <button
                                class={if *app_mode == 1 { "mode-btn mode-btn--active" } else { "mode-btn" }}
                                onclick={on_mode_patogeny}
                            >{"Patogeny"}</button>
                        </div>
                    </div>
                </div>
            </header>
            <main>
                if *loading {
                    <div class="loading-state">
                        <div class="loading-spinner"></div>
                        <p class="loading-text">{"Ładowanie danych naturopatycznych..."}</p>
                    </div>
                } else if *app_mode == 0 {
                    <div class="wizard">
                        <div class="wizard-header">
                            <div class="wizard-step-info">
                                <div class="wizard-step-icon">{step_icons[step]}</div>
                                <div>
                                    <div class="wizard-progress">{format!("Krok {} z {}", step + 1, steps.len())}</div>
                                    <div class="wizard-title">{steps[step]}</div>
                                    if !wizard_state.full_name.is_empty() && step > 0 {
                                        <div class="wizard-patient">{format!("Pacjent: {}", wizard_state.full_name)}</div>
                                    }
                                </div>
                            </div>
                            <div class="wizard-progress-bar">
                                {for (0..steps.len()).map(|i| {
                                    let cls = if i < step { "progress-dot completed" }
                                              else if i == step { "progress-dot active" }
                                              else { "progress-dot" };
                                    html! { <div class={cls}></div> }
                                })}
                            </div>
                        </div>

                        <div class="wizard-body">
                            <div class="wizard-content">
                                {render_step(step, &wizard_state, &categories, &search_text)}
                            </div>
                            <div class="wizard-nav">
                                <button
                                    type="button"
                                    class="wizard-button-secondary"
                                    disabled={step == 0}
                                    onclick={on_prev}
                                >{"← Wstecz"}</button>
                                if step < 7 {
                                    <button
                                        type="button"
                                        class="wizard-button"
                                        onclick={on_next}
                                    >{"Dalej →"}</button>
                                } else {
                                    <>
                                        <button
                                            type="button"
                                            class="wizard-button"
                                            onclick={on_download}
                                        >{"⬇ Pobierz .docx"}</button>
                                        <button
                                            type="button"
                                            class="wizard-button-secondary"
                                            onclick={on_reset}
                                        >{"↺ Nowy raport"}</button>
                                    </>
                                }
                            </div>
                        </div>
                    </div>
                } else {
                    {render_patogeny(&wizard_state, &categories, &search_text, on_copy)}
                }
            </main>
            <footer>
                <p>{"© 2026 Hunter Report · Naturopatyczny Generator Raportów · Rust/Yew"}</p>
            </footer>
        </>
    }
}

fn render_step(
    step: usize,
    state: &UseStateHandle<WizardState>,
    categories: &UseStateHandle<Vec<Category>>,
    search_text: &UseStateHandle<String>,
) -> Html {
    match step {
        0 => html! {
            <section class="wizard-step wizard-step--active">
                <div class="wizard-field">
                    <label>{"Imię i nazwisko pacjenta"}</label>
                    <input
                        type="text"
                        placeholder="np. Jan Kowalski"
                        value={state.full_name.clone()}
                        oninput={
                            let state = state.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                let mut s = (*state).clone();
                                s.full_name = input.value();
                                state.set(s);
                            })
                        }
                    />
                </div>
                <div class="wizard-field">
                    <label>{"Data badania"}</label>
                    <input
                        type="date"
                        value={state.test_date.clone()}
                        oninput={
                            let state = state.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                let mut s = (*state).clone();
                                s.test_date = input.value();
                                state.set(s);
                            })
                        }
                    />
                </div>
            </section>
        },
        1 => html! {
            <section class="wizard-step wizard-step--active">
                <textarea
                    class="wizard-textarea"
                    placeholder="Opisz zgłaszane przez pacjenta dolegliwości i objawy, np. bóle głowy, zmęczenie, problemy trawienne..."
                    value={state.manual_notes.clone()}
                    oninput={
                        let state = state.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            let mut s = (*state).clone();
                            s.manual_notes = input.value();
                            state.set(s);
                        })
                    }
                />
            </section>
        },
        2 => {
            let burdens = vec![
                "stomatologiczne", "pasożytnicze", "bakteryjne", "wirusowe",
                "grzybicze", "mykotoksyczne", "geopatyczne", "elektrosmogiem",
            ];
            let current_burdens: HashSet<_> = state.burdens.iter().cloned().collect();
            html! {
                <section class="wizard-step wizard-step--active">
                    <div class="wizard-checkboxes">
                        {for burdens.into_iter().map(|b| {
                            let checked = current_burdens.contains(b);
                            let state = state.clone();
                            let b_name = b.to_string();
                            let b_span = b.to_string();
                            html! {
                                <label class="wizard-checkbox">
                                    <input
                                        type="checkbox"
                                        checked={checked}
                                        onclick={Callback::from(move |_| {
                                            let mut s = (*state).clone();
                                            if s.burdens.contains(&b_name) {
                                                s.burdens.retain(|x| x != &b_name);
                                            } else {
                                                s.burdens.push(b_name.clone());
                                            }
                                            state.set(s);
                                        })}
                                    />
                                    <span>{b_span}</span>
                                </label>
                            }
                        })}
                    </div>
                    <div class="wizard-field" style="margin-top: 1rem;">
                        <label>{"Dodatkowe informacje"}</label>
                        <input
                            type="text"
                            placeholder="Inne obciążenia lub uwagi..."
                            value={state.burdens_extra.clone()}
                            oninput={
                                let state = state.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    let mut s = (*state).clone();
                                    s.burdens_extra = input.value();
                                    state.set(s);
                                })
                            }
                        />
                    </div>
                </section>
            }
        },
        3 => html! {
            <section class="wizard-step wizard-step--active">
                <textarea
                    class="wizard-textarea"
                    placeholder="Wymień wykryte metale toksyczne, np. ołów (Pb), rtęć (Hg), kadm (Cd), arsen (As), aluminium (Al)..."
                    value={state.toxic_metals.clone()}
                    oninput={
                        let state = state.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            let mut s = (*state).clone();
                            s.toxic_metals = input.value();
                            state.set(s);
                        })
                    }
                />
            </section>
        },
        4 => html! {
            <section class="wizard-step wizard-step--active">
                <textarea
                    class="wizard-textarea"
                    placeholder="Wymień wykryte niedobory, np. witamina D3, witamina B12, żelazo, magnez, cynk, kwasy omega-3..."
                    value={state.deficiencies.clone()}
                    oninput={
                        let state = state.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            let mut s = (*state).clone();
                            s.deficiencies = input.value();
                            state.set(s);
                        })
                    }
                />
            </section>
        },
        5 => html! {
            <section class="wizard-step wizard-step--active">
                <textarea
                    class="wizard-textarea"
                    placeholder="Podaj wykryte częstotliwości patogenne z badania Vega Test..."
                    value={state.frequencies.clone()}
                    oninput={
                        let state = state.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            let mut s = (*state).clone();
                            s.frequencies = input.value();
                            state.set(s);
                        })
                    }
                />
            </section>
        },
        6 => {
            let selected_pathogens: HashSet<_> = state.selected_pathogens.iter().cloned().collect();
            let filter = search_text.to_lowercase();
            html! {
                <section class="wizard-step wizard-step--active">
                    <div class="wizard-step7-layout layout-container">
                        <div class="left-column">
                            <div class="search-container">
                                <textarea
                                    class="search-input"
                                    placeholder="Wyszukaj lub wklej listę patogenów (każdy w nowej linii)..."
                                    oninput={
                                        let search_text = search_text.clone();
                                        let state = state.clone();
                                        let categories = categories.clone();
                                        Callback::from(move |e: InputEvent| {
                                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                                            let val = input.value();
                                            if val.contains('\n') {
                                                let mut s = (*state).clone();
                                                let lines: Vec<String> = val.lines()
                                                    .map(|l| l.trim().to_lowercase())
                                                    .filter(|l| !l.is_empty())
                                                    .collect();
                                                for line in lines {
                                                    for cat in &*categories {
                                                        for item in &cat.items {
                                                            if item.name.to_lowercase() == line
                                                                || item.name.to_lowercase().contains(&line)
                                                            {
                                                                if !s.selected_pathogens.contains(&item.name) {
                                                                    s.selected_pathogens.push(item.name.clone());
                                                                }
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                                state.set(s);
                                            }
                                            search_text.set(val);
                                        })
                                    }
                                />
                            </div>
                            <div class="wizard-selected">
                                {format!("Wybrano: {}", state.selected_pathogens.len())}
                            </div>
                        </div>
                        <div class="right-column">
                            <div class="boxes-table">
                                {for categories.iter().map(|cat| {
                                    let items: Vec<_> = cat.items.iter().filter(|i| {
                                        filter.is_empty() || i.name.to_lowercase().contains(&filter)
                                    }).collect();
                                    if items.is_empty() && !filter.is_empty() {
                                        return html! {};
                                    }
                                    let cat_name = cat.name.clone();
                                    html! {
                                        <div class="boxes">
                                            <h3>{cat_name}</h3>
                                            {for items.into_iter().map(|item| {
                                                let checked = selected_pathogens.contains(&item.name);
                                                let state = state.clone();
                                                let name = item.name.clone();
                                                let display_name = item.name.clone();
                                                html! {
                                                    <label class="item-label">
                                                        <input
                                                            type="checkbox"
                                                            checked={checked}
                                                            onclick={Callback::from(move |_| {
                                                                let mut s = (*state).clone();
                                                                if s.selected_pathogens.contains(&name) {
                                                                    s.selected_pathogens.retain(|x| x != &name);
                                                                } else {
                                                                    s.selected_pathogens.push(name.clone());
                                                                }
                                                                state.set(s);
                                                            })}
                                                        />
                                                        <span>{display_name}</span>
                                                    </label>
                                                }
                                            })}
                                        </div>
                                    }
                                })}
                            </div>
                        </div>
                    </div>
                </section>
            }
        },
        7 => html! {
            <section class="wizard-step wizard-step--active">
                <textarea
                    class="wizard-textarea"
                    placeholder="Wpisz zalecenia terapeutyczne: suplementacja, dieta, styl życia, terapie wspierające..."
                    value={state.recommendations.clone()}
                    oninput={
                        let state = state.clone();
                        Callback::from(move |e: InputEvent| {
                            let input: HtmlTextAreaElement = e.target_unchecked_into();
                            let mut s = (*state).clone();
                            s.recommendations = input.value();
                            state.set(s);
                        })
                    }
                />
            </section>
        },
        _ => html! {},
    }
}

fn render_patogeny(
    state: &UseStateHandle<WizardState>,
    categories: &UseStateHandle<Vec<Category>>,
    search_text: &UseStateHandle<String>,
    on_copy: Callback<MouseEvent>,
) -> Html {
    let selected_pathogens: HashSet<_> = state.selected_pathogens.iter().cloned().collect();
    let filter = search_text.to_lowercase();

    let on_reset_selection = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            let mut s = (*state).clone();
            s.selected_pathogens = vec![];
            state.set(s);
        })
    };

    let desc_map: std::collections::HashMap<String, String> = (**categories).iter()
        .flat_map(|c| c.items.iter())
        .map(|i| (i.name.clone(), i.description.clone()))
        .collect();

    html! {
        <div class="patogeny-layout layout-container">
            <div class="left-column">
                <div class="search-container">
                    <textarea
                        class="search-input"
                        placeholder="Wyszukaj lub wklej listę patogenów (każdy w nowej linii)..."
                        oninput={
                            let search_text = search_text.clone();
                            let state = state.clone();
                            let categories = categories.clone();
                            Callback::from(move |e: InputEvent| {
                                let input: HtmlTextAreaElement = e.target_unchecked_into();
                                let val = input.value();
                                if val.contains('\n') {
                                    let mut s = (*state).clone();
                                    let lines: Vec<String> = val.lines()
                                        .map(|l| l.trim().to_lowercase())
                                        .filter(|l| !l.is_empty())
                                        .collect();
                                    for line in lines {
                                        for cat in (*categories).iter() {
                                            for item in &cat.items {
                                                if item.name.to_lowercase() == line
                                                    || item.name.to_lowercase().contains(&line)
                                                {
                                                    if !s.selected_pathogens.contains(&item.name) {
                                                        s.selected_pathogens.push(item.name.clone());
                                                    }
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    state.set(s);
                                }
                                search_text.set(val);
                            })
                        }
                    />
                </div>
                <div class="wizard-selected">
                    {format!("Wybrano: {}", state.selected_pathogens.len())}
                </div>
                <div class="selected-list">
                    {for state.selected_pathogens.iter().map(|name| {
                        let desc = desc_map.get(name).cloned().unwrap_or_default();
                        html! {
                            <div class="selected-item">
                                <span class="selected-item-name">{name}</span>
                                if !desc.is_empty() {
                                    <span class="selected-item-desc">{&desc}</span>
                                }
                            </div>
                        }
                    })}
                </div>
                <div class="patogeny-actions">
                    <button
                        class="wizard-button"
                        onclick={on_copy}
                        disabled={state.selected_pathogens.is_empty()}
                    >{"⎘ Kopiuj"}</button>
                    <button
                        class="wizard-button-secondary"
                        onclick={on_reset_selection}
                        disabled={state.selected_pathogens.is_empty()}
                    >{"↺ Wyczyść"}</button>
                </div>
            </div>
            <div class="right-column">
                <div class="boxes-table">
                    {for (**categories).iter().map(|cat| {
                        let items: Vec<_> = cat.items.iter().filter(|i| {
                            filter.is_empty() || i.name.to_lowercase().contains(&filter)
                        }).collect();
                        if items.is_empty() && !filter.is_empty() {
                            return html! {};
                        }
                        let cat_name = cat.name.clone();
                        html! {
                            <div class="boxes">
                                <h3>{cat_name}</h3>
                                {for items.into_iter().map(|item| {
                                    let checked = selected_pathogens.contains(&item.name);
                                    let state = state.clone();
                                    let name = item.name.clone();
                                    let display_name = item.name.clone();
                                    html! {
                                        <label class="item-label">
                                            <input
                                                type="checkbox"
                                                checked={checked}
                                                onclick={Callback::from(move |_| {
                                                    let mut s = (*state).clone();
                                                    if s.selected_pathogens.contains(&name) {
                                                        s.selected_pathogens.retain(|x| x != &name);
                                                    } else {
                                                        s.selected_pathogens.push(name.clone());
                                                    }
                                                    state.set(s);
                                                })}
                                            />
                                            <span>{display_name}</span>
                                        </label>
                                    }
                                })}
                            </div>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn parse_description_file(text: &str) -> Vec<Category> {
    let mut categories = Vec::new();
    let parts = text.split(">>").skip(1);
    for part in parts {
        let mut sections = part.split('^');
        let name = sections.next().unwrap_or("").trim().to_string();
        let mut items = Vec::new();
        for section in sections {
            let mut split = section.split('$');
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

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
