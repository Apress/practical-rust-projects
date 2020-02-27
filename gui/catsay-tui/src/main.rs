extern crate cursive;

use cursive::traits::Identifiable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::Cursive;

struct CatsayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out the form for the cat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_id("message"))
                    .child("Dead?", Checkbox::new().with_id("dead")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_id("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let is_dead = s
                    .call_on_id("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = CatsayOptions {
                    message: &message,
                    dead: is_dead,
                };
                result_step(s, &options)
            }),
    );
}

// TODO: change this to options
fn result_step(siv: &mut Cursive, options: &CatsayOptions) {
    let eye = if options.dead { "x" } else { "o" };

    let cat_text = format!(
        "{msg}
 \\
  \\
    /\\_/\\
   ( {eye} {eye} )
   =( I )=",
        msg = options.message,
        eye = eye
    );

    siv.pop_layer();
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}

fn main() {
    let mut siv = Cursive::default();

    input_step(&mut siv);

    siv.run();
}
