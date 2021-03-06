// Imports from other crates
use gtk::prelude::{ToggleButtonExt, WidgetExt};

// Imports from other modules
use super::{GestureModel, Model, Msg, TapMotion, Win};

impl relm::Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &relm::Relm<Self>, _: Self::ModelParam) -> Model {
        Model {
            gesture: GestureModel::new(),
        }
    }

    /// Regularly poll events from the wayland event queue
    fn subscriptions(&mut self, relm: &relm::Relm<Self>) {
        relm::interval(relm.stream(), 100, || Msg::PollEvents);
    }

    /// Handles all received messages
    fn update(&mut self, event: Msg) {
        match event {
            // The GestureModel converts  'GestureSignal's to 'Interaction's the keyboard can understand. The coordinates are converted to relative coordinates
            // and the new values are sent to the keyboard as input
            Msg::GestureSignal(x, y, gesture_signal) => {
                let ((x, y), interaction) =
                    self.model
                        .gesture
                        .convert_to_interaction(x, y, gesture_signal);
                let (x, y) = self.get_rel_coordinates(x, y);
                self.keyboard.input(x, y, interaction);
            }
            // If a button was clicked, activate it or deactivate it and give haptic feedback
            Msg::ButtonInteraction(layout, view, key_id, tap_motion) => {
                info! {
                    "Trying to interact with '{}' key", key_id
                };
                if let Some((button, _)) = self.widgets.buttons.get(&(layout, view, key_id)) {
                    // Activate/Deactivate it (visual feedback of the button press)
                    button.set_active(tap_motion == TapMotion::Press);
                    // Give haptic feedback
                    self.ui_manager
                        .haptic_feedback(tap_motion == TapMotion::Press);
                } else {
                    error!("UI does not know the key id and can't handle the ButtonInteraction");
                }
            }
            // Open the popover of the specified button
            Msg::OpenPopup(key_id) => {
                let (layout, view) = self.ui_manager.current_layout_view.clone();
                if let Some((button, popover)) = self.widgets.buttons.get(&(layout, view, key_id)) {
                    button.set_active(false);
                    if let Some(popover) = popover {
                        popover.show_all();
                    } else {
                        error!("The button does not have a popup to open");
                    }
                } else {
                    error!("UI does not know the key id and can't open the popover");
                }
            }
            // Tell the keyboard to submit the text
            Msg::SubmitText(text, append_space) => self.keyboard.submit_text(text, append_space),
            // Update the labels of the buttons for the suggestions
            #[cfg(feature = "suggestions")]
            Msg::Suggestions(suggestions) => {
                self.update_suggestions(suggestions);
            }
            Msg::SetVisibility(new_visibility) => {
                self.ui_manager.change_visibility(new_visibility);
            }
            // Have the UIManager handle the change of hint/purpose
            Msg::HintPurpose(content_hint, content_purpose) => self
                .ui_manager
                .change_hint_purpose(content_hint, content_purpose),
            // Have the UIManager handle the change of the layout/view
            Msg::ChangeUILayoutView(layout, view) => {
                let _ = self.ui_manager.change_layout_view(&layout, view); // Result not relevant
            }
            // Notify the keyboard about a change of the layout/view
            Msg::ChangeKBLayoutView(layout, view) => {
                self.keyboard.active_view = (layout, view);
            }
            // Have the UIManager handle the change of the orientation
            Msg::ChangeUIOrientation(mode) => self.ui_manager.change_orientation(mode),
            // Tell the keyboard to fetch the wayland events
            Msg::PollEvents => {
                self.keyboard.fetch_events();
            }
            #[cfg(feature = "gesture")]
            // Draw the path of the gesture
            Msg::UpdateDrawBuffer => {
                self.draw_path();
            }
            // Quit the program
            Msg::Quit => gtk::main_quit(),
        }
    }
}
