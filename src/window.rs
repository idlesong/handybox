#[macro_use]
use gtk::prelude::*;
use gtk::{TreeView, TreeStore, ListStore, ScrolledWindow, TreeViewColumn, CellRendererText};
use gio::prelude::*;

use crate::wikipedia::Wikipedia;
use crate::youdao::Youdao;

// use serde_json::{Value};
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt, WebViewExtManual};


pub struct Window {
    pub widget: gtk::ApplicationWindow,
    pub entry: gtk::Entry,
    pub label: gtk::Label,
    pub label2: gtk::Label,
    pub results_list: ResultsList,
    // pub browser: Browser,
    // pub webview: WebView,
    // pub content: String,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ResultsList {
    pub container: gtk::ScrolledWindow,
    pub listbox: gtk::ListBox,
}

pub struct Browser {
    pub container: gtk::ScrolledWindow,
    pub treeview: gtk::TreeView,
    // pub treestore: gtk::TreeStore,
    pub liststore: gtk::ListStore,
}


impl Window {

    pub fn new(app: &gtk::Application) -> Self {

        let widget: gtk::ApplicationWindow = gtk::ApplicationWindow::new(app);
        // let browser = Browser::new();
        let results_list: ResultsList = ResultsList::new();

        widget.set_title("Handybox");
        widget.set_border_width(10);
        widget.set_position(gtk::WindowPosition::Center);
        widget.set_default_size(800, 600);

        results_list.container.set_size_request(100, -1);

        let entry: gtk::Entry  = gtk::Entry::new();
        let label: gtk::Label = gtk::Label::new_with_mnemonic(Some("Basic"));
        let label2: gtk::Label = gtk::Label::new_with_mnemonic(Some("_Web"));

        let context = WebContext::get_default().unwrap();
        let webview: WebView = WebView::new_with_context(&context);
        // webview.load_uri("https://crates.io/");
        // webview.load_uri("https://en.jinzhao.wiki/wiki/GNU");

        let settings = WebViewExt::get_settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
        vbox.add(&entry);
        vbox.pack_start(&label, false, false, 10);
        vbox.pack_start(&label2, false, false, 10);
        vbox.pack_start(&results_list.container, true, true, 0);
        // vbox.pack_start(&webview, true, true, 10);
        // widget.add(&webview);
        widget.add(&vbox);
        widget.show_all();

        // let content: String = String::new();
        Self {
            widget,
            // mybox,
            entry,
            label,
            label2,
            results_list,
            // content,
            // browser,
        }
    }

    pub fn connect_events(mut self) {
	    self.input_changed();
	    self.input_complete();
    }

 //   fn model(&self.content) -> String {
 //   	let content = &self.content;
 //   	content
 //   }

    fn input_changed(&mut self) {
    	let entry = self.entry.clone();
    	let label = self.label.clone();
    	//let mut content = self.content.clone();

        self.entry.connect_changed(move |_| {
            let sentence = entry.get_text().expect("get_text failed");
	        //entry.get_text().expect("get_text failed").chars().collect();
            label.set_text(&sentence);
        });
    }

    fn input_complete(&self) {
        let label = self.label.clone();
    	let label2 = self.label2.clone();
    	let entry = self.entry.clone();
        let results_list = self.results_list.clone();
        // let treestore = self.browser.treestore.clone();
        // let liststore = self.browser.liststore.clone();
    	//let content = self.content.clone();
    	// let webview = self.webview.clone();

        self.entry.connect_activate(move |_| {
                // let body = reqwest::get("https://www.ruby-lang.org")?.text()?;
                // println!("body = {:?}",body);
                let content = entry.get_text().expect("get_text failed").to_string();
                // let resp = Youdao::search(&content).ok().unwrap();
                let resp = Wikipedia::get(&content).ok().unwrap();

                let mut basic = String::from("Basic explains: \n");
                if resp["query"]["pages"].is_array(){
                    // println!("basic dictionary = {:#?}", resp["basic"]["explains"].as_array());
                    for x in resp["query"]["pages"].as_array().unwrap() {
                        basic = basic + &x.to_string() + "\n";
                        // println!("x = {:#?}", x);
                    }
                }

                if resp["query"]["pages"].is_object(){
                    // println!("basic dictionary = {:#?}", resp["basic"]["explains"].as_array());
                    let pages = resp["query"]["pages"].as_object().unwrap();
                    // basic = basic + &x.to_string() + "\n";
                    // let pages_number = pages.len();

                    // let mut results: [&str; pages_number = ["view"; pages_number];
                    let mut results = Vec::new();
                    for (key, value) in pages.iter() {
                        // println!("{}", value);
                        // println!("{}", value["extract"]);
                        results.push(value["extract"].to_string());
                    }

                    // let v: Value = serde_json::from_str(data)?

                    // liststore.clear();
                    // for d in results.iter() {
                    //     let values: [&dyn ToValue; 1] = [&d];
                    //     liststore.set(&liststore.append(), &[0], &values);
                    // }
                    results_list.list_update_rows(results);
                    results_list.container.show_all();
                }

                let mut web = String::from("Web explains: \n");
                // if resp["web"][0]["value"].is_array(){
                //     // println!("basic dictionary = {:#?}", resp["basic"]["explains"].as_array());
                //     for x in resp["web"][0]["value"].as_array().unwrap() {
                //         web = web + &x.to_string() + "\n";
                //         // println!("web x = {:#?}", x);
                //     }
                // }

                // results.list_update_rows();
                // println!("basic = {:?}",basic);
                label.set_text(&basic);
                label2.set_text(&web);
                // webview.load_uri("https://en.jinzhao.wiki/wiki/GNU");
        });
    }

}

impl ResultsList {
    fn new() -> ResultsList {

        let vbox_outer = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let listbox = gtk::ListBox::new();
        vbox_outer.pack_start(&listbox, true, true, 0);

        let container = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        container.add(&vbox_outer);

        ResultsList {container, listbox}
    }

    fn list_update_rows(&self, row_datas: Vec<String>) {
        let mut rows: Vec<gtk::ListBoxRow> = Vec::new();
        for data in &row_datas {
            let label = gtk::Label::new(None);
            label.set_markup(&data);
            label.set_line_wrap(true);
            let row = gtk::ListBoxRow::new();
            row.add(&label);
            self.listbox.add(&row);
        }
    }

}
