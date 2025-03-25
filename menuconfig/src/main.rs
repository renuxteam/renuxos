use cursive::theme::{Color, BaseColor, Style};
use cursive::views::{Checkbox, Dialog, LinearLayout, ListView, TextView, SelectView};
use cursive::Cursive;

fn welcome(s: &mut Cursive)
{
    let renux_version = TextView::new("Version: Renux OS 0.1.0 Aurora(DEV)")
        .style(Style::from(Color::Light(BaseColor::Black)));
    let layout = LinearLayout::vertical()
        .child(renux_version);
    s.add_fullscreen_layer(layout);

    s.add_layer(Dialog::around(TextView::new("Hello, Welcome to Renux Menuconfig!"))
        .title("Menuconfig")
        .button("Ok", |s| {
            s.pop_layer();
            main_menu(s);
        }));
}
fn main_menu(s: &mut Cursive)
{
    let mut menu = SelectView::new();
    menu.add_item("Drivers on Kernel","general");


    let mut list = ListView::new();

    list.add_child("1. Enable VGA Driver", Checkbox::new());


    s.add_layer(
        Dialog::around(list)
            .title("Main Menu")
            .button("Save", |s|{
                s.add_layer(Dialog::info("Configuration Saved!"));
            })
            .button("Back", |s| {
                s.pop_layer();
                welcome(s);
            })
            .button("Quit", |s| s.quit()),
        
    )
    
}
fn main()
{
    let mut siv = cursive::default();
    welcome(&mut siv);
    siv.run();
}