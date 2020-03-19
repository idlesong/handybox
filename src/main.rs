use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;

mod config;
mod window;
mod youdao;
mod wikipedia;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("handybox", config::LOCALEDIR);
    textdomain("handybox");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/handybox.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("im.idlesong.handybox"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = Window::new();

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();

        window.connect_events();        
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
