use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;
    view! {
            <button
                on:click=move |_| {
                    // on stable, this is set_count.set(3);
                    set_count.update(|n| *n += 1);
                }
                class:red=move || count.get() % 2 == 1        >
                "Click me: "
                // on stable, this is move || count.get();
                {move || count.get()}
            </button>
            <ProgressBar progress=count/>
            <ProgressBar progress=Signal::derive(double_count)/>
        <p>
            "Double Count: "
            // and again here
            {double_count}
        </p>
        }
}


#[component]
fn ProgressBar( 
    // #[prop(optional)]
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>) -> impl IntoView {
    view! {
        <progress
            max="50"
            // hmm... where will we get this from?
            value=progress
        />
    }
}