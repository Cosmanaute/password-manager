use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{SelectView, TextView, Dialog, ListView, EditView, Checkbox};
use cursive::Cursive;
use cursive::menu;
use std::fs::{File, self};
use std::path::{Path, PathBuf};
mod hcrypto;
use std::io::prelude::*;
use std::io::Write;
use cursive::View;

fn usage(app: &mut Cursive) {
   let text = format!("Arrowkeys to move up and down,\nEnter to submit.\nCtrl + Backspace to Backspace.");
   app.add_layer(Dialog::around(TextView::new(text)).title("Usage").button("OK", |s| {s.pop_layer();}))
}

pub fn notify(app: &mut Cursive, msg: &str, status: &str) {
   let rout = format!("{}: {}", status, msg);
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
         _=> notify(s, "An error occured!", "Error"),
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
      .button("Cancel", |s| {s.pop_layer();})
      .button("Continue", |s| {
         let username = s.call_on_name("username", |t: &mut EditView| t.get_content()).unwrap();
         let password = s.call_on_name("password", |t: &mut EditView| t.get_content()).unwrap();
         let signup = s.call_on_name("signup", |t: &mut Checkbox| t.is_checked()).unwrap();

         let info = SigninDetails {
            username: &username,
            password: &password,
            signup,
         };

         if Path::new("secure").is_dir() == false {
            fs::create_dir("secure").expect("Could not create folder");
         }
         if Path::new("secure/signatures").is_dir() == false {
            fs::create_dir("secure/signatures").expect("Could not create folder");
         }
         if Path::new("secure/vault").is_dir() == false {
            fs::create_dir("secure/vault").expect("Could not create folder");
         }
         check_pass(s, info);
       })
      .fixed_width(30));
}

fn signup(app: &mut Cursive, info: &SigninDetails, fp: &str) {
      let mut file = File::create(&fp).expect("Could not create file!"); 
      let hashed_password = hcrypto::hash(&info.password);
      file.write_all(hashed_password.as_bytes()).expect("Could not write to file!");
      notify(app, "User created!", "Info");
}

fn check_pass(app: &mut Cursive, info: SigninDetails) {
    let fp = format!("secure/signatures/{}.txt", info.username);

    if Path::new(&fp).exists() == false && info.signup == true {
         signup(app, &info, &fp);
    }
    else if Path::new(&fp).exists() == false && info.signup == false {
         notify(app, "Incorrect username or password!", "Error");
    }
    else if Path::new(&fp).exists() == true && info.signup == true {
         notify(app, "User already exists!", "Error");
    }
    else if Path::new(&fp).exists() == true && info.signup == false {
        let mut file = File::open(&fp).expect("Error opening file!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        if contents.as_str() == hcrypto::hash(&info.password).as_str() {
            app.pop_layer();
            let path = format!("secure/vault/{}", info.username);
            if Path::new(&path.as_str()).is_dir() == false {
                fs::create_dir(&path).expect("Could not create folder"); 
            }
            let username = info.username.clone();
            groups(app, username);
        }
        else {
            notify(app, "Incorrect username or password", "Error");
        }
    }
    else {
        notify(app, "Incorrect username or password", "Error");
    }
}

fn groups(app: &mut Cursive, username: &str) {
   let mut menu = SelectView::new().h_align(HAlign::Center);
   let dir = format!("secure/vault/{}", username);
   let entries = fs::read_dir(&dir).unwrap();
   
   for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path();

      if path.is_dir() {
         let dir_name = path.file_name().unwrap().to_str().unwrap();
         menu.add_item_str(dir_name);
      }
   }
   
   let user = String::from(username);
   let temp_user = String::from(user.clone());
   menu.set_on_submit(move |s, option: &str| {
      select_group(s, option, &user);
   });

   app.add_layer(Dialog::around(menu).title("Vault - Groups")
      .button("Add Group", move |s| {
         add_group(s, &temp_user);})); 
}

fn add_group(app: &mut Cursive, username: &str) {    
   let user = String::from(username);
   app.add_layer(Dialog::new()
       .title("Add Group")
       .content(ListView::new()
       .child("Group: ", EditView::new().with_name("newgroup")),
      )
         .button("Add", move |s| {
            let new_group = s.call_on_name("newgroup", |t: &mut EditView| t.get_content()).unwrap();
            let fp = format!("secure/vault/{}/{}", &user, new_group);
            if Path::new(&fp).is_dir() == false {
                let file = fs::create_dir(&fp).expect("Could not create file");
                s.pop_layer();
                notify(s, "Group Created", "Success");
         }
            else {
                notify(s, "Group Already Exists!", "Error");    
         }
      })
         );
}

fn select_group(app: &mut Cursive, selected: &str, username: &str) {
   let dir = format!("secure/vault/{}", username);
   let entries = fs::read_dir(&dir).unwrap();
   
   for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path();
      let group = path.file_name().unwrap().to_str().unwrap();
      if group == selected {
         vault(app, &group, &username);
      }
   }
}

fn vault(app: &mut Cursive, group: &str, username: &str) {
   let path = format!("secure/vault/{}/{}", username, group);
   let entries = fs::read_dir(&path).unwrap();
   let mut menu = SelectView::new().h_align(HAlign::Center);
   
   for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path(); 

      let dir_name = path.file_name().unwrap().to_str().unwrap();
      menu.add_item_str(dir_name);
   }
   app.add_layer(Dialog::around(menu).title("Vault").button("Back", |s| {s.pop_layer();})
      .button("Add User", |s| s.quit()).fixed_width(30));
}




