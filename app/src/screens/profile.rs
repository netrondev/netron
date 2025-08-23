use leptos::ev::SubmitEvent;
use leptos::prelude::*;

#[component]
pub fn ProfileScreen() -> impl IntoView {
    let (username, set_username) = signal(String::from("Anonymous"));
    let (bio, set_bio) = signal(String::from(""));
    let (peer_id, _set_peer_id) = signal(String::from("Not connected"));
    let (edit_mode, set_edit_mode) = signal(false);

    let (temp_username, set_temp_username) = signal(String::new());
    let (temp_bio, set_temp_bio) = signal(String::new());

    let toggle_edit = move |_| {
        if edit_mode.get() {
            set_edit_mode.set(false);
        } else {
            set_temp_username.set(username.get());
            set_temp_bio.set(bio.get());
            set_edit_mode.set(true);
        }
    };

    let save_profile = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_username.set(temp_username.get());
        set_bio.set(temp_bio.get());
        set_edit_mode.set(false);
    };

    let update_username = move |ev| {
        let value = event_target_value(&ev);
        set_temp_username.set(value);
    };

    let update_bio = move |ev| {
        let value = event_target_value(&ev);
        set_temp_bio.set(value);
    };

    view! {
        <div class="screen profile-screen">
            <div class="profile-header">
                <div class="profile-avatar">"👤"</div>
                <h2>"Profile Settings"</h2>
            </div>

            {move || if edit_mode.get() {
                view! {
                    <div>
                    <form class="profile-form" on:submit=save_profile>
                        <div class="form-group">
                            <label for="username">"Username"</label>
                            <input
                                id="username"
                                type="text"
                                value=move || temp_username.get()
                                on:input=update_username
                                placeholder="Enter your username"
                            />
                        </div>

                        <div class="form-group">
                            <label for="bio">"Bio"</label>
                            <textarea
                                id="bio"
                                on:input=update_bio
                                placeholder="Tell us about yourself"
                                rows="4"
                            >{move || temp_bio.get()}</textarea>
                        </div>

                        <div class="form-actions">
                            <button type="submit" class="save-button">
                                <span>"💾 Save"</span>
                            </button>
                            <button type="button" class="cancel-button" on:click=toggle_edit>
                                "Cancel"
                            </button>
                        </div>
                    </form>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="profile-display">
                        <div class="profile-field">
                            <label>"Username:"</label>
                            <span class="profile-value">{move || username.get()}</span>
                        </div>

                        <div class="profile-field">
                            <label>"Bio:"</label>
                            <span class="profile-value">
                                {move || if bio.get().is_empty() {
                                    "No bio set".to_string()
                                } else {
                                    bio.get()
                                }}
                            </span>
                        </div>

                        <div class="profile-field">
                            <label>"Peer ID:"</label>
                            <span class="profile-value peer-id">{move || peer_id.get()}</span>
                        </div>

                        <button class="edit-button" on:click=toggle_edit>
                            "Edit Profile"
                        </button>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
