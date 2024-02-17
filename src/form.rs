use leptos::*;
use leptos_router::ActionForm;

pub async fn http_get(uri: &str) -> anyhow::Result<String> {
    Ok(reqwest::get(uri).await?.text().await?)
}

#[server(AddToDo, "/api")]
pub async fn add_todo(name: String) -> Result<String, ServerFnError> {
    Ok(format!("from server: {name}"))
}

#[component]
pub fn ActionFormButton() -> impl IntoView {
    let add_todo = create_server_action::<AddToDo>();

    let value = add_todo.value();
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <div class="flex-col gap-2">
            <ActionForm
                action=add_todo
                class="flex-clo gap-2 items-center mt-2"
                on:submit=move|_| {
                    logging::log!("on submit...");
                }
            >
                <span class="text-gray-500">"Name: "</span>
                <input
                    type="text"
                    class="block round-md border-2 text-gray-900 pl-1 py-1 pr-1"
                    prop:placeholder="Input name"
                    name="name"
                />
                <input
                    type="submit"
                    value="Submit"
                    class="block round-md border-2 pl-1 py-1 pr-1 mt-2 rounded-md"
                />
            </ActionForm>
            <p>{value}</p>
        </div>
    }
}
