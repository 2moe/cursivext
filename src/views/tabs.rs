use cursive::{Cursive, event::Event};
pub use cursive_tabs::{self, TabPanel};

/// Registers global keyboard shortcuts for tab navigation in Cursive UI.
///
/// Binds two alternative shortcuts per tab (0-9):
/// - Alt + [0-9] (where 0 represents the 10th tab)
/// - Ctrl+Shift + [F1-F10] (F1=Tab1, F10=Tab0)
///
/// - tab1
///   - Alt + 1
///   - OR: Ctrl+Shift+F1
/// - tab2
///   - Alt + 2
///   - OR: Ctrl+Shift+F2
/// - ...
/// - tab9
///   - Alt + 9
///   - OR: Ctrl+Shift+F9
///
/// # Q&A
///
/// - Q: What's the tabs_name?
/// - A:
///
/// ```no_run
/// tabPanel::new().with_tab(view1).with_tab(view9).with_name("tabs");
/// ```
/// "tabs" is the tabs_name.
///
/// # Keybind Matrix
///
/// | Tab | Alt+ | Ctrl+Shift+ |
/// |-----|------|-------------|
/// | 1   | 1    | F1          |
/// | ... | ...  | ...         |
/// | 9   | 9    | F9          |
/// | 10   | 0    | F10         |
pub fn add_global_key_shortcuts(siv: &mut Cursive, tabs_name: &'static str) {
  use cursive::event::Key::*;

  for ((i, c), fkey) in ('0'..='9')
    .enumerate()
    .zip([F10, F1, F2, F3, F4, F5, F6, F7, F8, F9])
  {
    for event in [Event::AltChar(c), Event::CtrlShift(fkey)] {
      siv.add_global_callback(event, move |s| {
        s.call_on_name(tabs_name, |t: &mut TabPanel| {
          let order = t.tab_order();
          let len = order.len();
          let name = match i {
            n if n > len || n == 0 => {
              log::warn!("alt+{c}, tabs.len: {len} => get(len - 1)");
              order.get(len - 1).unwrap()
            }
            _ => order
              .get(i - 1)
              .expect("Failed to get tab"),
          };

          t.set_active_tab(name)
        });
      })
    }
  }
}

/// Registers global keyboard shortcuts for sequential tab navigation.
///
/// Binds two alternative shortcuts for each direction:
/// - ​**Previous Tab**:
///   - `Alt+PageUp`
///   - `Ctrl+Alt+PageUp`
/// - ​**Next Tab**:
///   - `Alt+PageDown`
///   - `Ctrl+Alt+PageDown`
///
/// # Keybind Matrix
/// | Direction    | Primary Shortcut | Alternate Shortcut |
/// |--------------|------------------|--------------------|
/// | Previous Tab | Alt+PageUp       | Ctrl+Alt+PageUp    |
/// | Next Tab     | Alt+PageDown     | Ctrl+Alt+PageDown  |
pub fn add_global_tab_switch_key_shortcuts(
  siv: &mut Cursive,
  tabs_name: &'static str,
) {
  use cursive::event::Key::*;
  siv.add_global_callback(Event::CtrlAlt(PageDown), |s| {
    s.call_on_name(tabs_name, |t: &mut TabPanel| t.next());
  });

  siv.add_global_callback(Event::Alt(PageDown), |s| {
    s.call_on_name(tabs_name, |t: &mut TabPanel| t.next());
  });

  siv.add_global_callback(Event::CtrlAlt(PageUp), |s| {
    s.call_on_name(tabs_name, |t: &mut TabPanel| t.prev());
  });

  siv.add_global_callback(Event::Alt(PageUp), |s| {
    s.call_on_name(tabs_name, |t: &mut TabPanel| t.prev());
  });
}
