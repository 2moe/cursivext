use cursive::{
  self, Vec2, View,
  direction::Direction,
  event::{Event, EventResult, Key, MouseButton, MouseEvent},
  theme::PaletteStyle,
  utils::markup::StyledString,
  view::CannotFocus,
  views::{self, DummyView, LinearLayout, TextView},
};

#[derive(Default)]
pub struct Checkbox(pub views::Checkbox);

impl core::ops::DerefMut for Checkbox {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl core::ops::Deref for Checkbox {
  type Target = views::Checkbox;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Checkbox {
  /// Constructs new checkbox.
  ///
  /// # Example
  ///
  /// ```
  /// use cursivext::views::Checkbox;
  ///
  /// let checkbox = Checkbox::new();
  /// ```
  pub fn new() -> Self {
    Self(views::Checkbox::new())
  }

  /// Creates horizontally aligned checkbox with label.
  ///
  /// Typical UI pattern combining checkbox, spacing and text label
  ///
  /// # Parameter
  ///
  /// - label
  ///   - Text displayed next to checkbox (supports styled strings)
  ///
  /// # Example
  ///
  /// ```
  /// use cursivext::{cursive, views::Checkbox};
  /// use cursive::{With, views::Panel};
  ///
  /// let checkbox = Checkbox::wrap_linear_layout("good?").wrap_with(Panel::new);
  ///
  /// cursive::default()
  ///   .with(|siv| siv.add_layer(checkbox));
  ///   // .run();
  ///
  ///   // => [✔︎] good?
  /// ```
  pub fn wrap_linear_layout<S: Into<StyledString>>(label: S) -> LinearLayout {
    LinearLayout::horizontal()
      .child(Checkbox::new())
      .child(DummyView)
      .child(TextView::new(label))
  }

  fn draw_internal(&self, printer: &cursive::Printer) {
    printer.print((0, 0), "[ ]");
    if self.is_checked() {
      printer.print((1, 0), "✔︎");
    }
  }
}

impl View for Checkbox {
  fn required_size(&mut self, _: Vec2) -> Vec2 {
    Vec2::new(3, 1)
  }

  fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
    self
      .is_enabled()
      .then(EventResult::consumed)
      .ok_or(CannotFocus)
  }

  fn draw(&self, printer: &cursive::Printer) {
    match self.is_enabled() && printer.enabled {
      true => printer
        .with_selection(printer.focused, |printer| self.draw_internal(printer)),
      _ => printer.with_style(PaletteStyle::Secondary, |printer| {
        self.draw_internal(printer)
      }),
    }
  }

  fn on_event(&mut self, event: Event) -> EventResult {
    if !self.is_enabled() {
      return EventResult::Ignored;
    }
    match event {
      Event::Key(Key::Enter) | Event::Char(' ') => self.toggle(),
      Event::Mouse {
        event: MouseEvent::Release(MouseButton::Left),
        position,
        offset,
      } if position.fits_in_rect(offset, (3, 1)) => self.toggle(),
      _ => EventResult::Ignored,
    }
  }
}

#[cfg(test)]
mod tests {
  use cursive::{With, views::Panel};

  use super::*;

  #[ignore]
  #[test]
  fn test_show_checkbox() {
    let checkbox = Checkbox::wrap_linear_layout("good?").wrap_with(Panel::new);

    cursive::default()
      .with(|siv| siv.add_layer(checkbox))
      .run()
  }
}
