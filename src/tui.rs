use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{SelectView, TextView, Dialog, LinearLayout, ListView, EditView, Checkbox};
use cursive::Cursive;


pub fn show_error(app: &mut Cursive, msg: &str) {
   let rout = format!("Error: {}", msg);
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
         .child("signup", Checkbox::new().with_name("Signup")),
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

       }).fixed_width(30));
}
