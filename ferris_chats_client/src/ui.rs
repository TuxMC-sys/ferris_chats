use cursive::style::{BorderStyle, Palette};
use cursive::traits::With;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;

fn ui(){
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
    siv.run();
}