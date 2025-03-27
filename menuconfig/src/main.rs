use cursive::Cursive;
use cursive::theme::{BaseColor, Color, Palette, PaletteColor, Style, Theme};
use cursive::views::{Checkbox, Dialog, LinearLayout, ListView, SelectView, TextView};
use std::io::{BufRead, BufReader, Read};
use std::process::Command;

// Function to configure the theme (light or dark mode)
fn set_theme(s: &mut Cursive, dark_mode: bool)
{
  let mut theme = Theme::default();
  let palette = if dark_mode {
    // Dark mode configuration
    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Primary] = Color::Light(BaseColor::White);
    palette[PaletteColor::Secondary] = Color::Light(BaseColor::Blue);
    palette
  } else {
    // Light mode configuration
    let mut palette = Palette::default();
    palette[PaletteColor::Background] = Color::Light(BaseColor::White);
    palette[PaletteColor::View] = Color::Light(BaseColor::White);
    palette[PaletteColor::Primary] = Color::Dark(BaseColor::Black);
    palette[PaletteColor::Secondary] = Color::Dark(BaseColor::Blue);
    palette
  };
  theme.palette = palette;
  s.set_theme(theme);
}

// Welcome screen
fn welcome(s: &mut Cursive)
{
  let renux_version = TextView::new("Version: Renux OS 0.1.0 Aurora(DEV)")
    .style(Style::from(PaletteColor::Secondary)); // Use the secondary color for the version text

  let layout = LinearLayout::vertical().child(renux_version);

  s.add_fullscreen_layer(layout);

  s.add_layer(
    Dialog::around(TextView::new("Hello, Welcome to Renux Menuconfig!"))
      .title("Menuconfig")
      .button("Ok", |s| {
        s.pop_layer();
        main_menu(s);
      }),
  );
}

// Compiler flags menu
fn compiler_flags_menu(s: &mut Cursive)
{
  let mut list = ListView::new();
  list.add_child("1. Enable Debug Symbols", Checkbox::new());
  list.add_child("2. Optimize for Size (-C opt-level=z)", Checkbox::new());
  list.add_child("3. Optimize for Speed (-C opt-level=3)", Checkbox::new());
  list.add_child("4. Enable SCCACHE to increase speed", Checkbox::new());

  s.add_layer(
    Dialog::around(list)
      .title("Compiler Flags on rustc")
      .button("Back", |s| {
        s.pop_layer();
        main_menu(s);
      }),
  );
}

// Main menu
fn main_menu(s: &mut Cursive)
{
  let mut menu = SelectView::new();
  menu.add_item("Drivers on Kernel", "general");

  let mut list = ListView::new();
  list.add_child("1. Enable VGA Driver", Checkbox::new());

  s.add_layer(
    Dialog::around(list)
      .title("Main Menu")
      .button("Compiler Flags", |s| {
        compiler_flags_menu(s); // Opens the compiler flags menu
      })
      .button("Quit", |s| s.quit()),
  );
}

fn main()
{
  let mut siv = cursive::default();
  set_theme(&mut siv, true); // Sets the initial theme to light mode
  welcome(&mut siv);
  siv.run();
}
