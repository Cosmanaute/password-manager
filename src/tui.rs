use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{SelectView, TextView, Dialog, ListView, EditView, Checkbox};
use cursive::Cursive;
use std::fs::File;
use std::path::Path;
mod hcrypto;
use std::io::prelude::*;
use std::io::Write;

fn usage(app: &mut Cursive) {
   
}

pub fn show_msg(app: &mut Cursive, msg: &str, status: &str) {
   let rout = format!("{}: {}", status, msg);
   app.add_layer(Dialog::around(TextView::new(rout)).button("OK", |s| {s.pop_layer();}));
}

pub fn start(app: &mut Cursive) {
   let mut menu = SelectView::new().h_align(HAlign::Center);
   menu.add_item("Login", "0");
   menu.add_item("Usage", "1");
   menu.add_item("Exit", "2");

   menu.set_on_submit(|s, option: &str| {
      match option {
         "0" => login(s),
         "1" => usage(s),
         "2" => s.quit(),
         _=> show_msg(s, "An error occured!", "Error"),
      };
   });
   app.add_layer(Dialog::around(menu).title("Menu").fixed_width(30));
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
         .child("Password: ", EditView::new().secret().with_name("password"))
         .child("signup", Checkbox::new().with_name("signup")),
      )
      .button("Continue", |s| {
         let username = s
             .call_on_name("username", |t: &mut EditView| t.get_content()).unwrap();
         let password = s
             .call_on_name("password", |t: &mut EditView| t.get_content()).unwrap();
         let signup = s
             .call_on_name("signup", |t: &mut Checkbox| t.is_checked()).unwrap();
         let info = SigninDetails {
            username: &username,
            password: &password,
            signup,
         };
         check_pass(s, &info);
       })
      .button("Cancel", |s| {s.pop_layer();})
      .fixed_width(30));
}

fn signup(app: &mut Cursive, info: &SigninDetails, fp: &str) {
      let mut file = File::create(&fp).expect("Could not create file!"); 
      let hashed_password = hcrypto::hash(&info.password);
      file.write_all(hashed_password.as_bytes()).expect("Could not write to file!");
      show_msg(app, "User created!", "Info");
}

fn check_pass(app: &mut Cursive, info: &SigninDetails) {
   let fp = format!("master{}.txt", info.username);
   if Path::new(&fp).exists() == false {
      if info.signup == true {
         signup(app, info, &fp);
      }
      else {
         show_msg(app, "Incorrect username or password!", "Error");
      }
   }
   else {   
      let mut file = File::open(&fp).expect("Could not open file");
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      if contents.as_str() == hcrypto::hash(&info.password).as_str() {
         std::mem::drop(contents);
         dashboard(app);
      }
      else {
         show_msg(app, "Incorrect username or password", "Error");
         std::mem::drop(contents);
      }
   }
}

fn dashboard(app: &mut Cursive) {
    show_msg(app, "Logged in!", "Success"); 
}



