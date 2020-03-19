use gtk::prelude::*;

use crate::wikipedia::Wikipedia;
use crate::youdao::Youdao;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    pub entry: gtk::Entry,
    pub label: gtk::Label,
    pub label2: gtk::Label,
    pub content: String,
}

impl Window {

    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/im/idlesong/handybox/window.ui");
        let widget: gtk::ApplicationWindow = builder.get_object("window").expect("Failed to find the window object");

	let entry: gtk::Entry = builder.get_object("entry1").expect("Failed to find entry");
	let label: gtk::Label = builder.get_object("label1").expect("Failed to find label");
	let label2: gtk::Label = builder.get_object("label2").expect("Failed to find label");

        let content: String = String::new();

        Self {
            widget,
            entry,
            label,
            label2,
            content,
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
    	//let content = self.content.clone();

        self.entry.connect_activate(move |_| {
                // let body = reqwest::get("https://www.ruby-lang.org")?.text()?;
                // println!("body = {:?}",body);
                let content = entry.get_text().expect("get_text failed").to_string();
                // let resp = Youdao::search(&content).ok().unwrap();
                let resp = Wikipedia::get(&content).ok().unwrap();
                // println!("basic = {:#?}", resp["basic"]["explains"]);
                // println!("basic = {:#?}", resp["basic"]["explains"].as_array().unwrap());
                // println!("web = {:#?}", resp["web"][0]["value"]);
                // println!("resp = {:#?}", resp.ok().unwrap()["basic"]["explains"]);
                // Youdao::search("test") => Success;

                let mut basic = String::from("Basic explains: \n");
                // if resp["basic"]["explains"].is_array(){
                //     // println!("basic dictionary = {:#?}", resp["basic"]["explains"].as_array());
                //     for x in resp["basic"]["explains"].as_array().unwrap() {
                //         basic = basic + &x.to_string() + "\n";
                //         // println!("x = {:#?}", x);
                //     }
                // }

                let mut web = String::from("Web explains: \n");
                // if resp["web"][0]["value"].is_array(){
                //     // println!("basic dictionary = {:#?}", resp["basic"]["explains"].as_array());
                //     for x in resp["web"][0]["value"].as_array().unwrap() {
                //         web = web + &x.to_string() + "\n";
                //         // println!("web x = {:#?}", x);
                //     }
                // }

                // println!("basic = {:?}",basic);
                label.set_text(&basic);
                label2.set_text(&web);
        });
    }

}
