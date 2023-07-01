use console::Emoji;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct SpinnerBuilder(ProgressBar);
impl SpinnerBuilder {
    pub fn new<S: AsRef<str>>(inital_msg: S) -> SpinnerBuilder {
        let pb = ProgressBar::new_spinner();

        let msg = inital_msg.as_ref().to_string();
        if !msg.is_empty() {
            pb.set_message(msg);
        }

        pb.enable_steady_tick(Duration::from_millis(120));
        let finish = format!("{}", Emoji("✅", "OK").0);
        pb.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                // For more spinners check out the cli-spinners project:
                // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", &finish]),
        );
        Self(pb)
    }
    pub fn inc(self, val: u64) {
        self.0.inc(val)
    }
    pub fn finish<S: AsRef<str>>(&self, finish_msg: S) {
        let ms = finish_msg.as_ref().to_string();
        self.0.finish_with_message(ms)
    }
    pub fn set_message<S: AsRef<str>>(&self, msg: S) {
        let ms = msg.as_ref().to_string();
        self.0.set_message(ms)
    }
}
