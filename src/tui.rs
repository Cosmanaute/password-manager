use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{SelectView, TextView, Dialog, ListView, EditView, Checkbox};
use cursive::Cursive;
use std::fs::{File, self};
use std::path::Path;
mod hcrypto;
use std::io::prelude::*;
use std::io::Write;
use cursive::theme::PaletteStyle;

fn usage(app: &mut Cursive) {
   let text = format!("▲ and ▼ ---------→ Move
                     \nEnter -----------→ submit
                     \nCtrl + Backspace → Backspace
                     \nEsc -------------→ Quit");
   app.add_layer(Dialog::around(TextView::new(text).h_align(HAlign::Left)).title("Usage").button("OK", |s| {s.pop_layer();}));
}

pub fn notify(app: &mut Cursive, msg: &str, status: &str) {
   let text = format!("{}: {}", status, msg);
   app.add_layer(Dialog::around(TextView::new(text)).title(status).button("OK", |s| {s.pop_layer();}))
}

pub fn start(app: &mut Cursive) {
   let mut menu = SelectView::new().h_align(HAlign::Center);
   menu.add_item("Login / Register", "0");
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
      .title("Login / Register")
      .content(
         ListView::new()
         .child(" Username → ", EditView::new().style(PaletteStyle::TitleSecondary).with_name("username"))
         .child("Signature → ", EditView::new().secret().with_name("password"))
         .child(" Register →", Checkbox::new().with_name("signup")),
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
         if info.username.chars().all(char::is_whitespace) || info.username.is_empty() {
            notify(s, "Username cannot be None", "Error");
         }
         verify_signature_login(s, info);
       })
      .fixed_width(40));
}

fn signup(app: &mut Cursive, info: &SigninDetails, fp: &str) {
      let mut file = File::create(&fp).expect("Could not create file!"); 
      let hashed_password = hcrypto::hash(&info.password);
      file.write_all(hashed_password.as_bytes()).expect("Could not write to file!");
      notify(app, "User created!", "Info");
}

fn verify_signature_login(app: &mut Cursive, info: SigninDetails) {
   let fp = format!("secure/signatures/{}.txt", info.username);

   if Path::new(&fp).exists() == false && info.signup == true {
         signup(app, &info, &fp);
   }
   else if Path::new(&fp).exists() == false && info.signup == false {
         notify(app, "Incorrect username or signature", "Error");
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
            notify(app, "Incorrect username or signature", "Error");
        }
    }
    else {
        notify(app, "Incorrect username or signature", "Error");
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

   app.add_layer(Dialog::around(menu).title("Groups")
      .button("DELETE", |s| s.quit())
      .button("ADD", move |s| {
         add_group(s, &temp_user);}).min_width(30).min_height(8));
}

fn add_group(app: &mut Cursive, username: &str) {    
   let user = String::from(username);
   app.add_layer(Dialog::new()
       .title("Add Group")
       .content(ListView::new()
       .child("New group → ", EditView::new().with_name("newgroup")),
      )
         .button("Cancel", |s| {s.pop_layer();})
         .button("ADD", move |s| {
            let new_group = s.call_on_name("newgroup", |t: &mut EditView| t.get_content()).unwrap();
            if new_group.chars().all(char::is_whitespace) {
                s.pop_layer();
                notify(s, "Name Cannot Be None", "Error");
            }
            else {
               let fp = format!("secure/vault/{}/{}", &user, new_group);
               if Path::new(&fp).is_dir() == false {
                   fs::create_dir(&fp).expect("Could not create file");
                   s.pop_layer();
                   s.pop_layer();
                   notify(s, "Group Created", "Success");
                   groups(s, user.as_str());
               }
               else {
                    notify(s, "Group Already Exists!", "Error");    
               }
         }
      }).min_width(30).min_height(8)
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
   let title = format!("Vault -> {} ", &group);
   let fp = format!("secure/vault/{}/{}", username, group);
   let entries = fs::read_dir(&fp).unwrap();
   let mut menu = SelectView::new().h_align(HAlign::Center);
   
   for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path(); 

      let dir_name = path.file_name().unwrap().to_str().unwrap();
      menu.add_item_str(dir_name);
   }
   let user = String::from(username);
   let temp_user = String::from(user.clone());
   let group = String::from(group);
   let temp_group = String::from(group.clone());
   menu.set_on_submit(move |s, option: &str| {
      select_user(s, option, &temp_group, &user);
   });

   app.add_layer(Dialog::around(menu).title(title).button("Back", |s| {s.pop_layer();})
      .button("Add", move |s| {add_user(s, &group, &temp_user)}).min_width(30).min_height(8));
}

fn select_user(app: &mut Cursive, selected: &str, group: &str, username: &str) {
   let fp = format!("secure/vault/{}/{}", username, group);
   let entries = fs::read_dir(&fp).unwrap();
   
   for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path();
      let user = path.file_name().unwrap().to_str().unwrap();
      if user == selected {
         enter_signture(app, &fp.as_str(), &user, &username);
      }
   }
}

fn verify_signature(input: &str, username: &str) -> bool {
   let fp = format!("secure/signatures/{}.txt", username);
   let mut file = fs::File::open(&fp).expect("Could not open file"); 
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();

   let hashed_input = hcrypto::hash(&input); 
   contents.as_str() == hashed_input.as_str()
}

fn enter_signture(app: &mut Cursive, fp: &str, user: &str, username: &str) {
   let signature_name = String::from(username.clone());
   app.add_layer(Dialog::new()
      .title("Verify Signature")
      .content(
         ListView::new()
         .child("Signature: ", EditView::new().secret().with_name("signature")),
      )
      .button("Cancel", |s| {s.pop_layer();})
      .button("Verify", move |s| {
         let signature = s.call_on_name("signature", |t: &mut EditView| t.get_content()).unwrap();
         if verify_signature(&signature, &signature_name) {

         }
         else {
            notify(s, "Incorrect Signature", "Error");
         }

      }).fixed_width(30)
   )
}

fn show_user(app: &mut Cursive, fp: &str, user: &str) {

}

fn add_user(app: &mut Cursive, group: &str, username: &str) {
   let title = format!("{} -> Add User", &group);
   let group = String::from(group);
   let user = String::from(username);
   app.add_layer(Dialog::new()
       .title(title)
       .content(ListView::new()
       .child("New user → ", EditView::new().with_name("newuser"))
       .child("Password → ", EditView::new().secret().with_name("password")),
      )
         .button("Cancel", |s| {s.pop_layer();})
         .button("Add", move |s| {
            let new_user = s.call_on_name("newuser", |t: &mut EditView| t.get_content()).unwrap();
            let password = s.call_on_name("password", |t: &mut EditView| t.get_content()).unwrap();

            if new_user.chars().all(char::is_whitespace) || new_user.as_str() == "priv_key" || new_user.as_str() == "pub_key" {
               s.pop_layer();
               notify(s, "Name cannot be whitespace", "Error");
            }
            else {
               let fp = format!("secure/vault/{}/{}/{}", &user, &group, &new_user);
               if !Path::new(&fp).is_dir() {
                  fs::create_dir(&fp).expect("Could not create dir");
                  let fp = format!("secure/vault/{}/{}/{}/{}.txt", &user, &group, &new_user, &new_user);
                  let mut file = fs::File::create(&fp).expect("Could not create file");

                  let key = hcrypto::hash(&user);
                  let encrypted_password = hcrypto::encrypt(&key, &password);
                  file.write_all(&encrypted_password.as_bytes()).expect("Could not write to file");
                  
                   s.pop_layer();
                   s.pop_layer();

                   vault(s, &group, user.as_str());
               }
               else {
                    notify(s, "User already exists", "Error");    
               }
            }
      }).min_width(30).min_height(10)
   );
}




