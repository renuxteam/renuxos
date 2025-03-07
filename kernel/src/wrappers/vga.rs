// Import vga.c in drivers/gpu/c/
unsafe extern "C" {
  fn vga_clear_screen();
  fn vga_print(text: *const u8);
}

pub fn clear_screen()
{
  unsafe {
    vga_clear_screen();
  }
}

pub fn print_text(text: &str)
{
  unsafe {
    vga_print(text.as_ptr());
  }
}
