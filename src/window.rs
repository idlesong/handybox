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
    pub results_list: ResultsList,
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

        let context = WebContext::get_default().unwrap();
        let webview: WebView = WebView::new_with_context(&context);

        let settings = WebViewExt::get_settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 4);
        vbox.add(&entry);
        vbox.pack_start(&results_list.container, true, true, 0);
        // vbox.pack_start(&webview, true, true, 10);
        // widget.add(&webview);
        widget.add(&vbox);
        widget.show_all();

        Self {
            widget,
            entry,
            results_list,
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
    	// let label = self.label.clone();
    	//let mut content = self.content.clone();

        self.entry.connect_changed(move |_| {
            let sentence = entry.get_text().expect("get_text failed");
	        //entry.get_text().expect("get_text failed").chars().collect();
            // label.set_text(&sentence);
        });
    }

    fn input_complete(&self) {
    	let entry = self.entry.clone();
        let results_list = self.results_list.clone();
    	// let webview = self.webview.clone();

        self.entry.connect_activate(move |_| {
                // let body = reqwest::get("https://www.ruby-lang.org")?.text()?;
                // println!("body = {:?}",body);
                let content = entry.get_text().expect("get_text failed").to_string();
                let resp_youdao = Youdao::search(&content).ok().unwrap();
                let resp = Wikipedia::get(&content).ok().unwrap();

                let mut results = Vec::new();

                let mut basic = String::from("Basic explains: \n");
                if resp_youdao["basic"]["explains"].is_array(){
                    // println!("basic dictionary = {:#?}", resp["basic"]["explains"].as_array());
                    for x in resp_youdao["basic"]["explains"].as_array().unwrap() {
                        basic = basic + &x.to_string() + "\n";
                        // println!("x = {:#?}", x);
                    }
                }
                results.push(basic.to_string());

                let mut web_explains = String::from("Web explains: \n");
                if resp_youdao["web"].is_array(){
                    let explains = resp_youdao["web"].as_array().unwrap();
                    for explain in explains {
                        let mut explain_text = explain["key"].to_string();
                        for x in explain["value"].as_array().unwrap() {
                                explain_text = explain_text + &x.to_string() + "; ";
                            }
                        web_explains = web_explains + &explain_text + "\n";
                    }
                }
                results.push(web_explains.to_string());

                // let mut results = Vec::new();
                if resp["query"]["pages"].is_object(){
                    let pages = resp["query"]["pages"].as_object().unwrap();
                    for (key, value) in pages.iter() {
                        let fullurl = value["fullurl"].to_string();
                        let title = value["title"].to_string();
                        let extract = value["extract"].to_string();
                        let full_text = format!("<a href={}> {}</a> \n {}", fullurl, title, extract);
                        results.push(full_text);
                    }
                }
                results_list.list_update_rows(results);
                results_list.container.show_all();
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
