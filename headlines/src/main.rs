mod headlines;

use eframe::{ egui, run_native, NativeOptions, App };
use headlines::{ Headlines, PADDING };

impl App for Headlines {
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

    if self.config.dark_mode {
      ctx.set_visuals(egui::Visuals::dark());
    } else {
      ctx.set_visuals(egui::Visuals::light());
    }

    if !self.api_key_initialized {
      self.render_config(ctx);
    } else {
      self.render_top_panel(ctx, frame);
      render_footer(ctx);
      egui::CentralPanel::default().show(ctx, |ui| {
        render_header(ui);
        egui::ScrollArea::vertical().show(ui, |ui| {
          self.render_news_cards(ui);
        });
      });
    }
  }
}

fn render_footer(ctx: &egui::Context) {
  egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
    ui.vertical_centered(|ui| {
      ui.add_space(PADDING);
      let api_source = egui::RichText::new("API Source: newsapi.org").text_style(egui::TextStyle::Monospace);
      ui.add(egui::Label::new(api_source));
      let text = egui::RichText::new("Made with egui").text_style(egui::TextStyle::Monospace);
      ui.add(egui::Hyperlink::from_label_and_url(text, "https://github.com/emilk/egui"));
      let repo = egui::RichText::new("ShreyB2091/headlines").text_style(egui::TextStyle::Monospace);
      ui.add(egui::Hyperlink::from_label_and_url(repo, "https://github.com/ShreyB2091/headlines"));
      ui.add_space(PADDING);
    })
  });
}

fn render_header(ui: &mut egui::Ui) {
  ui.vertical_centered(|ui| {
    ui.heading("HEADLINES");
  });
  let sep = egui::Separator::default().spacing(10.0);
  ui.add(sep);
}

fn main() -> eframe::Result<()> {
  tracing_subscriber::fmt::init();
  let app = "Headlines";
  let mut native_options = NativeOptions::default();
  native_options.initial_window_size = Some(egui::Vec2::new(540., 640.));

  run_native(app, native_options, Box::new(|_cc| Box::new(Headlines::new(_cc))))
}