use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TreeView, TreeStore,
          TreeViewColumn, CellRendererText};

fn make_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    column.set_expand(true);
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Perf Viewer")
        .build();

    let tree_view = TreeView::builder()
        .build();
    let tree_store = TreeStore::new(&[String::static_type()]);

    tree_view.set_model(Some(&tree_store));
    tree_view.set_headers_visible(false);
    make_text_column(&tree_view);

    //for i in 0..10 {
    //    let iter = tree_store.insert_with_values(None, None, &[(0, &format!("Hello {}", i))]);

    //    for _ in 0..i {
    //        tree_store.insert_with_values(Some(&iter), None, &[(0, &"I'm a child node")]);
    //    }
    //}
    let gpu = tree_store.insert_with_values(None, None, &[(0, &"GPU Stats")]);
    tree_store.insert_with_values(Some(&gpu), None, &[(0, &"GPU Stats appear here...")]);

    let pane = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    pane.set_size_request(-1, -1);
    pane.append(&tree_view);

    window.set_child(Some(&pane));
    window.show();
}

fn main() {
    let app = Application::builder()
        .application_id("org.gtk.perf_viewer")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
