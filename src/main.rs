extern crate diesel;
extern crate sample_proj;

extern crate gio;
extern crate gtk;

use self::diesel::prelude::*;
use self::sample_proj::*;
use self::models::*;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use std::rc::Rc;

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Number,
    Severity,
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("treeview");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(280, 250);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
    window.add(&vbox);

    let label = gtk::Label::new(Some(
        "Show SQLite table.",
    ));
    vbox.add(&label);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let model = Rc::new(create_model());
    let treeview = gtk::TreeView::with_model(&*model);
    treeview.set_vexpand(true);

    sw.add(&treeview);

    add_columns(&model, &treeview);

    window.show_all();

    let model = model.clone();
}

struct Data {
    number: i32,
    severity: String,
}

fn create_model() -> gtk::ListStore {

    let col_types: [glib::Type; 2] = [
        glib::Type::I32,
        glib::Type::String,
    ];

    use sample_proj::schema::memos::dsl::*;

    let connection = establish_connection();
    let results = memos
        .limit(5)
        .load::<Memo>(&connection)
        .expect("Error loading memos");

    println!("Displaying {} memos", results.len());
    println!("id | comment");

    let mut data: Vec<Data> = Vec::new();

    for memo in results {
        println!("{} | {}", memo.id, memo.comment);
        data.push(Data { number: memo.id, severity: memo.comment.to_string(),});

    }

    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 2] = [0, 1];

    for (d_idx, d) in data.iter().enumerate() {

        let values: [&dyn ToValue; 2] = [
            &d.number,
            &d.severity,
        ];
        store.set(&store.append(), &col_indices, &values);
    }

    store
}

fn add_columns(model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {

    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("id");
        column.add_attribute(&renderer, "text", Columns::Number as i32);
        column.set_sort_column_id(Columns::Number as i32);
        treeview.append_column(&column);
    }

    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("comment");
        column.add_attribute(&renderer, "text", Columns::Severity as i32);
        column.set_sort_column_id(Columns::Severity as i32);
        treeview.append_column(&column);
    }

}

fn main() {

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.list-store"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());


}
