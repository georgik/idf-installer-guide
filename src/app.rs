use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // State hook to keep track of the current step
    let step = use_state(|| 0);

    // Total number of steps/screenshots you have
    let total_steps = 12;

    // Callback to handle button click and increment the step
    let on_next = {
        let step = step.clone();
        Callback::from(move |_| {
            let new_step = (*step + 1) % total_steps; // Loop back to the first step after the last one
            step.set(new_step);
        })
    };

    // Callback to handle button click and decrement the step
    let on_previous = {
        let step = step.clone();
        Callback::from(move |_| {
            let new_step = if *step == 0 {
                total_steps - 1 // Loop back to the last step from the first one
            } else {
                *step - 1
            };
            step.set(new_step);
        })
    };

    // Function to get the image path based on the current step
    let get_image_path = |step: usize| -> String {
        let filename = match step {
            0 => "01-idf-installer-license.png",
            1 => "02-idf-installer-pre-installation-check.png",
            2 => "03-idf-installer-download-esp-idf.png",
            3 => "04-idf-installer-version-of-esp-idf.png",
            4 => "05-idf-installer-select-destination-location.png",
            5 => "06-idf-installer-select-components-frameworks.png",
            6 => "07-idf-installer-select-components-drivers.png",
            7 => "08-idf-installer-select-components-chip-targets.png",
            8 => "09-idf-installer-ready-to-install.png",
            9 => "10-idf-installer-downloading.png",
            10 => "11-idf-installer-extracting-esp-idf.png",
            11 => "12-idf-installer-windows-defender.png",
            _ => "01-idf-installer-license.png", // Default case to handle any unexpected value
        }.to_string();
        "images/".to_string() + &filename
    };

    html! {
        <main>
            <h1>{ "ESP-IDF Installation Guide" }</h1>
            <img src={get_image_path(*step)} alt={format!("Step {}", *step + 1)} />
            <br />
            <button onclick={on_previous}>{ "Previous Step" }</button>
            <button onclick={on_next}>{ "Next Step" }</button>
        </main>
    }
}
