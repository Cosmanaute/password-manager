use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{SelectView, TextView, Dialog, LinearLayout, ListView, EditView, Checkbox, CircularFocus};
use cursive::{Cursive, With};
use std::fs::File;
mod hcrypto;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::os::unix::prelude::FileExt;


pub fn show_error(app: &mut Cursive, msg: &str) {
   let rout = format!("Info: {}", msg);
   app.add_layer(Dialog::around(TextView::new(rout)).button("OK", |s| s.quit()));
}

pub fn start(app: &mut Cursive) {
   let mut menu = SelectView::new().h_align(HAlign::Center);
   menu.add_item("Login", "0");
   menu.add_item("Usage", "1");
   menu.add_item("Exit", "2");

   menu.set_on_submit(|s, option: &str| {
      s.pop_layer();
      match option {
         "0" => login(s),
         "2" => s.quit(),
         _=> show_error(s, "An error occured!"),
      };
   });
   app.add_layer(Dialog::around(menu).title("Menu"));
}

struct SigninDetails<'a> {
   username: &'a str,
   password: &'a str,
   signup: bool,
}

fn login(app: &mut Cursive) {
    app.add_layer(Dialog::new()
      .title("Login")
      .content(
         ListView::new()
         .child("Username: ", EditView::new().with_name("username"))
         .child("Password: ", EditView::new().with_name("password"))
         .child("signup", Checkbox::new().with_name("signup")),
      )
      .button("Continue", |s| {
         let username = s
             .call_on_name("username", |t: &mut EditView| t.get_content())
             .unwrap();
         let password = s
             .call_on_name("password", |t: &mut EditView| t.get_content())
             .unwrap();
         let signup = s
             .call_on_name("signup", |t: &mut Checkbox| t.is_checked())
             .unwrap();
         let options = SigninDetails {
            username: &username,
            password: &password,
            signup,
         };
         check_pass(s, &options);
       }).fixed_width(30));
}

fn signup(app: &mut Cursive, info: &SigninDetails, fp: &str) {
      let mut file = File::create(&fp).expect("Could not create file!"); 
      let hashed_password = hcrypto::hash(&info.password);
      file.write_all(info.username.as_bytes()).expect("Could not write to file!");
      file.write_all(hashed_password.as_bytes()).expect("Could not write to file!");
      show_error(app, "User created!");
   
}

fn check_pass(app: &mut Cursive, info: &SigninDetails) {
   let fp = format!("master{}.txt", info.username);
   if info.signup == true {
      signup(app, info, &fp);
   }
   else {
      let mut file = File::open(&fp).expect("Could not open file");
      let output = file.read();
   }

}
