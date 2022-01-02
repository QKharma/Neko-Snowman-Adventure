use bevy::{prelude::*, window::WindowMode};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_startup_system(window_resize_system.system());
  }
}

fn window_resize_system(mut windows: ResMut<Windows>) {
  let window = windows.get_primary_mut().unwrap();
  window.set_resolution(1920.0, 1080.0);
  window.set_mode(WindowMode::Windowed);
  window.set_resizable(false);
  println!("Window size: {},{}", window.width(), window.height());
}