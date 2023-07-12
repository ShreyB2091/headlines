use eframe::{
  egui::{ self, FontDefinitions, FontData, FontId, Hyperlink, Separator, Label },
  CreationContext,
  epaint::Color32
};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

struct NewsCardData {
  title: String,
  desc: String,
  url: String
}

#[derive(Default)]
pub struct Headlines {
  articles: Vec<NewsCardData>
}

impl Headlines {
  pub fn new(_cc: &CreationContext<'_>) -> Headlines {
    let iter = (0..20).map(|a| NewsCardData {
      title: format!("Title {}", a),
      desc: format!("Description: {}", a),
      url: format!("https://example.com/{}", a)
    });
    
    let mut headlines = Headlines {
      articles: Vec::from_iter(iter)
    };
    headlines.configure_fonts(&_cc.egui_ctx);

    headlines
  }
  
  fn configure_fonts(&mut self, ctx: &egui::Context) {
    let mut font_def = FontDefinitions::default();
    font_def.font_data.insert(
      "MesloLGS".to_owned(),
      FontData::from_static(include_bytes!("../../fonts/MesloLGS_NF_Regular.ttf"))
    );
    font_def
      .families
      .get_mut(&egui::FontFamily::Proportional)
      .unwrap()
      .insert(0, "MesloLGS".to_owned());

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
      (egui::TextStyle::Heading, FontId::new(15.0,egui::FontFamily::Proportional)),
      (egui::TextStyle::Body, FontId::new(10.0, egui::FontFamily::Proportional)),
      (egui::TextStyle::Button, FontId::new(8.0, egui::FontFamily::Proportional)),
      (egui::TextStyle::Monospace,FontId::new(10.0,egui::FontFamily::Proportional))
    ].into();

    ctx.set_fonts(font_def);
    ctx.set_style(style);
  }

  pub fn render_news_cards(&mut self, ui: &mut egui::Ui) {
    for a in &self.articles {
      ui.add_space(PADDING);
      let title = format!("► {}", a.title);
      ui.colored_label(WHITE, title);
      ui.add_space(PADDING);
      let desc = egui::RichText::new(&a.desc).text_style(egui::TextStyle::Button);
      ui.add(Label::new(desc));
      ui.style_mut().visuals.hyperlink_color = CYAN;
      ui.add_space(PADDING);
      ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
        ui.add(Hyperlink::from_label_and_url("Read More »", &a.url));
      });
      ui.add_space(PADDING);
      ui.add(Separator::default());
    }
  }
}
