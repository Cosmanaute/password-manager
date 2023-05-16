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

//formater text og lager en ny layer med en dialog. Den har knapp OK.
//Ved å trykke ok popper den en layer-

pub fn notify(app: &mut Cursive, msg: &str, status: &str) {
   let text = format!("{}: {}", status, msg);
   app.add_layer(Dialog::around(TextView::new(text)).title(status).button("OK", |s| {s.pop_layer();}))
}

//en slags status melding som printer ut om det er error eller ikke.

pub fn start(app: &mut Cursive) {
   let mut menu = SelectView::new().h_align(HAlign::Center); //oppretter en visning som er på midten
   menu.add_item("Login / Register", "0"); //adder ting til visningen
   menu.add_item("Usage", "1"); 
   menu.add_item("Exit", "2");

   //bruker en method som tar in en closure. Sjekker hvilken rad brukeren velger og executer den
   //funksjonen
   menu.set_on_submit(|s, option: &str| {
      match option { //bruker match her for å sjekke hvilken
         "0" => login(s),
         "1" => usage(s),
         "2" => s.quit(), 
         _=> notify(s, "An error occured!", "Error"), //hvis noe galt skjer, blir denne funksjonen
         //kalt.
      };
   });
   app.add_layer(Dialog::around(menu).title("Menu").fixed_width(30)); //legger på selve laget
}

struct SigninDetails<'a> { //lager en struct for signindetails med lifetime 'a
   username: &'a str,
   password: &'a str,
   signup: bool,
}

fn login(app: &mut Cursive) {
   app.add_layer(Dialog::new()
      .title("Login / Register")
      .content(
         ListView::new()
            .child(" Username → ", EditView::new().style(PaletteStyle::TitleSecondary).with_name("username")) //en input som tar inn username
            .child("Signature → ", EditView::new().secret().with_name("password")) //en input som
            //tar passord
            .child(" Register →", Checkbox::new().with_name("signup")), //en checkbox som sjekker om man vil registrere eller ikke
      )
      .button("Cancel", |s| {s.pop_layer();})
      .button("Continue", |s| {
         let username = s.call_on_name("username", |t: &mut EditView| t.get_content()).unwrap(); //henter
         //brukernavn fra inputen
         let password = s.call_on_name("password", |t: &mut EditView| t.get_content()).unwrap();
         //henter passord fra inuten
         let signup = s.call_on_name("signup", |t: &mut Checkbox| t.is_checked()).unwrap();

         let info = SigninDetails { //legger det inn i et struct
            username: &username,
            password: &password,
            signup,
         };

         if Path::new("secure").is_dir() == false { //sjekker om mappestrukturen finnes, ellers oppretter
            //mappene
            fs::create_dir("secure").expect("Could not create folder");
         }
         if Path::new("secure/signatures").is_dir() == false {
            fs::create_dir("secure/signatures").expect("Could not create folder");
         }
         if Path::new("secure/vault").is_dir() == false {
            fs::create_dir("secure/vault").expect("Could not create folder");
         }
         if info.username.chars().all(char::is_whitespace) || info.username.is_empty() { //sjekker
            //om username er tomt eller ikke
            notify(s, "Username cannot be None", "Error");
         }
         verify_signature_login(s, info); //går videre og sjekker innlogging
      })
      .fixed_width(40));
}

fn signup(app: &mut Cursive, info: &SigninDetails, fp: &str) {
   if info.password.len() < 3 {
      notify(app, "Signature must be atleast 3 characters", "Error");
   } //sjekker at signaturen er lengre enn 3
   else {
      let mut file = File::create(&fp).expect("Could not create file!"); //opretter mappen
      let hashed_password = hcrypto::hash(&info.password); //kaller hashing funksjonen med en
      //referanse til info.password
      file.write_all(hashed_password.as_bytes()).expect("Could not write to file!"); //skriver til
      //filen
      notify(app, "User created!", "INFO");
   }
}

fn verify_signature_login(app: &mut Cursive, info: SigninDetails) {
   let fp = format!("secure/signatures/{}.txt", info.username); //formaterer filepath
   if Path::new(&fp).exists() == false && info.signup == true { //sjekker om filepathen eksisterer
      //og signup checkboksen er på
      signup(app, &info, &fp); //og kaller signup
   }
   else if Path::new(&fp).exists() == false && info.signup == false {
      notify(app, "Incorrect username or signature", "Error"); 
   }
   else if Path::new(&fp).exists() == true && info.signup == true {
      notify(app, "User already exists!", "Error");
   }
   else if Path::new(&fp).exists() == true && info.signup == false { //hvis den ikke eksisterer og
      //signup er av, logger man inn
      let mut file = File::open(&fp).expect("Error opening file!"); //åpner filen
      let mut contents = String::new(); //lager en tom string
      file.read_to_string(&mut contents).unwrap(); //leser fra filen til stringen

      if contents.as_str() == hcrypto::hash(&info.password).as_str() { //sjekker om passord gitt er
         //passordet i filen
         app.pop_layer();
         let path = format!("secure/vault/{}", info.username);
         if Path::new(&path.as_str()).is_dir() == false {
            fs::create_dir(&path).expect("Could not create folder"); 
         }
         let username = info.username.clone(); //kloner info.username
         groups(app, username); //kaller på group
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
   let mut menu = SelectView::new().h_align(HAlign::Center); //oppretter en view
   let dir = format!("secure/vault/{}", username); 
   let entries = fs::read_dir(&dir).unwrap(); //leser mappene i filepathen
   for entry in entries { //looper gjennom mappene
      let entry = entry.unwrap(); 
      let path = entry.path();

      if path.is_dir() { //legger mappene til i viewet.
         let dir_name = path.file_name().unwrap().to_str().unwrap();
         menu.add_item_str(dir_name);
      }
   }

   let user = String::from(username);
   let add_user = String::from(user.clone()); //koner variable
   let del_user = String::from(user.clone()); // --
   menu.set_on_submit(move |s, option: &str| { //sjekker hva som blir valgt og sender det. move
      //brukes for å tvinge funksjonen til å eige argumentene
      select_group(s, option, &user); 
   });

   app.add_layer(Dialog::around(menu).title("Groups")
      .button("DELETE", move |s| { //legger til en knapp for delete
         delete_group(s, &del_user) //kaller funksjonen med en kopi av username
      })
      .button("ADD", move |s| { //legger til en knapp for add
         add_group(s, &add_user);}).min_width(30).min_height(8)); // --
}

fn delete_group(app: &mut Cursive, user: &str) {
   let signature_user = String::from(user.clone());
   app.add_layer(Dialog::new() //lager ny dialog
      .title("Delete Group")
      .content(ListView::new() //leggeer til et en listevisning
         .child("Delete group → ", EditView::new().with_name("delgroup")), // legger input for
         // deleting group til i listen
      )
      .button("Cancel", |s| {s.pop_layer();}) // hvis brukeren trykker cancel
      .button("DELETE", move |s| {

         let del_group = s.call_on_name("delgroup", |t: &mut EditView| t.get_content()).unwrap(); // henter
         // data fra delgroup
         if del_group.chars().all(char::is_whitespace) { // sjekker at det ikke er tomt
            s.pop_layer();
            notify(s, "Cannot Be None", "Error");
         }
         else {
            let fp = format!("secure/vault/{}/{}", &signature_user, del_group); // formaterer
            if Path::new(&fp).is_dir() { // er directory
               let return_user = signature_user.clone(); // kopierer
               let msg = format!("All contents of {} will be deleted!\nDo you want to proceed?", &del_group); //formaterer
               s.add_layer(Dialog::around(TextView::new(msg).h_align(HAlign::Center)).title("Delete Group") // nytt lag med text msg
                  .button("Cancel", move |s| {s.pop_layer();}) // popper et lagg om trykker cancel
                  .button("Confirm", move |s| { 
                     fs::remove_dir_all(&fp).expect("Could not remove folder"); // sletter
                     // directory
                     s.pop_layer();
                     s.pop_layer();
                     s.pop_layer();
                     groups(s, &return_user); // går tilbake til groups
                     notify(s, "Group Deleted", "Success");
                  }).min_width(30).min_height(8));
            }
            else {
               notify(s, "Group Does Not Exists!", "Error");    
            }
         }
      }).min_width(30).min_height(8)
   );
}

fn add_group(app: &mut Cursive, username: &str) {    
   let user = String::from(username); // cloner string
   app.add_layer(Dialog::new()
      .title("Add Group") // tittel
      .content(ListView::new() // list view
         .child("New group → ", EditView::new().with_name("newgroup")), // input med navn newgroup
      )
      .button("Cancel", |s| {s.pop_layer();})
      .button("ADD", move |s| {
         let new_group = s.call_on_name("newgroup", |t: &mut EditView| t.get_content()).unwrap(); // henter
         // data fra newgroup
         if new_group.chars().all(char::is_whitespace) {
            s.pop_layer();
            notify(s, "Name Cannot Be None", "Error");
         }
         else {
            let fp = format!("secure/vault/{}/{}", &user, new_group); // formaterer
            if !Path::new(&fp).is_dir() { // sjekker at det ikke er et directory 
               fs::create_dir(&fp).expect("Could not create file"); // lager directory
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
   let fp = format!("secure/vault/{}", username); 
   let entries = fs::read_dir(&fp).unwrap(); // leser directory. unwrap passer på at vi får
   // resultatet

   for entry in entries { // looper gjennom dir
      let entry = entry.unwrap(); 
      let path = entry.path(); // lager en path med path method
      let group = path.file_name().unwrap().to_str().unwrap(); // gir dataen til group med filnavn
      // og som en string
      if group == selected { // sjekker hvilken brukeren valgte
         vault(app, &group, &username);
      }
   }
}

fn vault(app: &mut Cursive, group: &str, username: &str) {
   let title = format!("{} → vault ", &group);
   let fp = format!("secure/vault/{}/{}", username, group);
   let entries = fs::read_dir(&fp).unwrap();
   let mut menu = SelectView::new().h_align(HAlign::Center);

   //looper gjennom directory
   for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path(); 

      let dir_name = path.file_name().unwrap().to_str().unwrap();
      // legger dir_name til i menu på select view
      menu.add_item_str(dir_name);
   }
   // kloner variabler til senere
   let user = String::from(username);
   let temp_user = String::from(user.clone());
   let group = String::from(group);
   let temp_group = String::from(group.clone());
   let delete_username = user.clone();
   let delete_groupname = String::from(group.clone());

   // sjekker hvilken som er valgt
   menu.set_on_submit(move |s, option: &str| {
      select_user(s, option, &temp_group, &user);
   });

   app.add_layer(Dialog::around(menu).title(title).button("Back", |s| {s.pop_layer();})
      .button("DELETE", move |s| {delete_user(s,  &delete_groupname, &delete_username);}) // |s| er
      // for closure og er selve appen
      .button("Add", move |s| {add_user(s, &group, &temp_user)}).min_width(30).min_height(8));
}

fn select_user(app: &mut Cursive, selected: &str, group: &str, username: &str) {
   let fp = format!("secure/vault/{}/{}", username, group);
   let entries = fs::read_dir(&fp).unwrap();
// looper gjennom og sjekker hvilken brukeren valgte som tidligere i koden
for entry in entries {
      let entry = entry.unwrap();
      let path = entry.path();
      let user = path.file_name().unwrap().to_str().unwrap();
      if user == selected {
         enter_signture(app, &fp.as_str(), &user, &username);
      }
   }
}

fn delete_user(app: &mut Cursive, group: &str, signature_username: &str) {
   let signature_user = String::from(signature_username.clone());
   let group = String::from(group.clone());
   app.add_layer(Dialog::new()
      .title("Delete User")
      .content(ListView::new()
         .child("Delete User → ", EditView::new().with_name("deluser")),
      )
      .button("Cancel", |s| {s.pop_layer();})
      .button("DELETE", move |s| {

         //henter dataen
         let del_user = s.call_on_name("deluser", |t: &mut EditView| t.get_content()).unwrap();
         // sjekker at den ikke er tom
         if del_user.chars().all(char::is_whitespace) {
            s.pop_layer();
            notify(s, "Cannot Be None", "Error");
         }
         else {
            let fp = format!("secure/vault/{}/{}/{}", &signature_user, &group, &del_user);
            if Path::new(&fp).is_dir() {
               let return_user = String::from(signature_user.clone());
               let return_group = String::from(group.clone());
               let msg = format!("Are you sure you want to delete {}?", &del_user);
               s.add_layer(Dialog::around(TextView::new(msg).h_align(HAlign::Center)).title("Delete Group")
                  .button("Cancel", move |s| {s.pop_layer();})
                  .button("Confirm", move |s| {
                     // sletter dir
                     fs::remove_dir_all(&fp).expect("Could not remove folder");
                     s.pop_layer();
                     s.pop_layer();
                     s.pop_layer();
                     vault(s, &return_group, &return_user);
                     notify(s, "User Deleted", "Success");
                  }).min_width(30).min_height(8));
            }
            else {
               notify(s, "User Does Not Exists!", "Error");    
            }
         }
      }).min_width(30).min_height(8)
   );
}

fn verify_signature(input: &str, username: &str) -> bool {
   let fp = format!("secure/signatures/{}.txt", username);
   let mut file = fs::File::open(&fp).expect("Could not open file"); // åpner fil
   let mut contents = String::new(); // lager en tom string (buffer)
   file.read_to_string(&mut contents).unwrap(); // leser til buffer

   let hashed_input = hcrypto::hash(&input); // hasher input
   return contents.as_str() == hashed_input.as_str() // sjekker om hashed input er i fil
}

fn enter_signture(app: &mut Cursive, fp: &str, user: &str, username: &str) {
   let signature_name = String::from(username.clone());
   let user = String::from(user.clone());
   let fp = format!("{}/{}", &fp, &user);
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
            s.pop_layer();
            show_user(s, &fp, &user.as_str());
         }
         else {
            notify(s, "Incorrect Signature", "Error");
         }

      }).fixed_width(30)
   )
}

fn show_user(app: &mut Cursive, fp: &str, user: &str) {
   let fp = format!("{}/{}.txt", &fp, &user); // formaterer path
   let mut file = fs::File::open(&fp).expect("Could not open file");
   let mut encrypted_password = String::new(); // lager buffer
   file.read_to_string(&mut encrypted_password).expect("Could not read file"); // leser til buffer

   let key = hcrypto::hash(&user); // hasher user til key
   let decrypted_password = hcrypto::decrypt(&key, &encrypted_password); // decrypterer passord

   let title = format!("Vault → {}", &user);
   app.add_layer(Dialog::new().title(title).content( // lager et lag som viser resultat
      ListView::new()
         .child("Username → ", TextView::new(user)) // viser username
         .child("Password → ", TextView::new(decrypted_password)) // viser passord
   ).button("Close", |s| {s.pop_layer();}).min_width(30).min_height(8))
}

fn add_user(app: &mut Cursive, group: &str, username: &str) {
   let title = format!("{} -> Add User", &group);
   let group = String::from(group); // gjør om fra &str til String
   let user = String::from(username); // --
   app.add_layer(Dialog::new()
      .title(title)
      .content(ListView::new()
         .child("New user → ", EditView::new().with_name("newuser")) // input
         .child("Password → ", EditView::new().secret().with_name("password")), // input
      )
      .button("Cancel", |s| {s.pop_layer();})
      .button("Add", move |s| {
         let new_user = s.call_on_name("newuser", |t: &mut EditView| t.get_content()).unwrap(); // henter
         // data
         let password = s.call_on_name("password", |t: &mut EditView| t.get_content()).unwrap();

         if new_user.chars().all(char::is_whitespace) { // sjekker om den er tom
            s.pop_layer();
            notify(s, "Name cannot be whitespace", "Error");
         }
         else {
            let fp = format!("secure/vault/{}/{}/{}", &user, &group, &new_user);
            if !Path::new(&fp).is_dir() { // sjekker at bruker ikke finnes
               fs::create_dir(&fp).expect("Could not create dir"); // lager dir
               let fp = format!("secure/vault/{}/{}/{}/{}.txt", &user, &group, &new_user, &new_user); // formaterer
               let mut file = fs::File::create(&fp).expect("Could not create file"); // lager txt
               // fil

               let key = hcrypto::hash(&new_user); // hasker user til key
               let encrypted_password = hcrypto::encrypt(&key, &password); // krypterer paassord
               file.write_all(&encrypted_password.as_bytes()).expect("Could not write to file"); // skriver
               // passord til fil
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




