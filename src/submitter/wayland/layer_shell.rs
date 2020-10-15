pub fn make_overlay_layer(window: &gtk::Window) {
    // Before the window is first realized, set it up to be a layer surface
    gtk_layer_shell::init_for_window(window);

    // Order above normal windows
    gtk_layer_shell::set_layer(window, gtk_layer_shell::Layer::Overlay);

    // The margins are the gaps around the window's edges
    // Margins and anchors can be set like this...
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Left, 0);
    gtk_layer_shell::set_margin(window, gtk_layer_shell::Edge::Right, 0);
    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Right, true);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Top, false);
    gtk_layer_shell::set_anchor(window, gtk_layer_shell::Edge::Bottom, true);
    info!("The window is now a layer-shell overlay");
}
