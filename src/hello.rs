use leptos::*;

#[server(SayHello)]
pub async fn say_hello(name: String) -> Result<String, ServerFnError> {
    let count = name.parse::<i32>().unwrap_or_default();
    if count % 2 == 0 {
        Ok(format!("hello,{name}"))
    } else {
        Err(ServerFnError::ServerError("Internal Error".to_string()))
    }
}

#[component]
pub fn HelloButton() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (msg, set_msg) = create_signal(String::default());

    let content = std::fs::read_to_string("/tmp/weather.dat").unwrap_or_default();

    view! {
        <div class="flex gap-2">
            <button
                class="border-2"
                on:click=move |_| {
                    let name = format!("{}", count.get());
                    spawn_local(async move {
                        let msg = say_hello(name).await;
                        set_msg.set(format!("{:?}", msg));
                        set_count.update(|v| *v += 1);
                    });
                }
            >

                "Hello Button"
            </button>
            <p>{msg}</p>
            <p>{content}</p>
        </div>
    }
}
