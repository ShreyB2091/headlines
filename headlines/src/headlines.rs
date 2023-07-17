use eframe::{
  egui::{ self, FontDefinitions, FontData, FontId, Hyperlink, Separator, Label, TopBottomPanel },
  CreationContext,
  epaint::Color32
};
use serde::{ Serialize, Deserialize };
use newsapi::NewsAPI;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
const COL: Color32 = Color32::from_rgb(100, 178, 200);

fn fetch_news(api_key: &str, articles: &mut Vec<NewsCardData>) {
  if let Ok(response) = NewsAPI::new(api_key).fetch() {
    let resp_articles = response.articles();
    for a in resp_articles.iter() {
      let news = NewsCardData {
        title: a.title().to_string(),
        url: a.url().to_string(),
        desc: a.desc().map(|s| s.to_string()).unwrap_or("...".to_string())
      };
      articles.push(news);
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
  pub dark_mode: bool,
  pub api_key: String
}

impl Default for HeadlinesConfig {
  fn default() -> Self {
    Self {
      dark_mode: true,
      api_key: String::new()
    }
  }
}

pub struct NewsCardData {
  pub title: String,
  pub desc: String,
  pub url: String
}

#[derive(Default)]
pub struct Headlines {
  pub articles: Vec<NewsCardData>,
  pub config: HeadlinesConfig,
  pub api_key_initialized: bool
}

impl Headlines {
  pub fn new(_cc: &CreationContext<'_>) -> Headlines {

    let config: HeadlinesConfig = confy::load("headlines", None).unwrap_or_default();
    
    let mut headlines = Headlines {
      articles: vec![],
      config,
      api_key_initialized: false
    };

    fetch_news(&headlines.config.api_key, &mut headlines.articles);
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
      (egui::TextStyle::Heading, FontId::new(18.0,egui::FontFamily::Proportional)),
      (egui::TextStyle::Body, FontId::new(10.0, egui::FontFamily::Proportional)),
      (egui::TextStyle::Button, FontId::new(8.0, egui::FontFamily::Proportional)),
      (egui::TextStyle::Monospace,FontId::new(8.0,egui::FontFamily::Proportional)),
      (egui::TextStyle::Name("Controls".into()), FontId::new(12.0,egui::FontFamily::Proportional)),
      (egui::TextStyle::Name("Theme".into()), FontId::new(15.0,egui::FontFamily::Proportional)),
    ].into();

    ctx.set_fonts(font_def);
    ctx.set_style(style);
  }

  pub fn render_news_cards(&mut self, ui: &mut egui::Ui) {
    for a in &self.articles {
      ui.add_space(PADDING);
      let title = format!("► {}", a.title);
      if self.config.dark_mode {
        ui.colored_label(WHITE, title);
      } else {
        ui.colored_label(BLACK, title);
      }
      ui.add_space(PADDING);
      let desc = egui::RichText::new(&a.desc).text_style(egui::TextStyle::Button);
      ui.add(Label::new(desc));
      if self.config.dark_mode {
        ui.style_mut().visuals.hyperlink_color = CYAN;
      } else {
        ui.style_mut().visuals.hyperlink_color = COL;
      }
      ui.add_space(PADDING);
      ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
        ui.add(Hyperlink::from_label_and_url("Read More »", &a.url));
      });
      ui.add_space(PADDING);
      ui.add(Separator::default());
    }
  }

  pub(crate) fn render_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
          let logo = egui::RichText::new("🌐").text_style(egui::TextStyle::Heading);
          ui.add(Label::new(logo));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
          let close = egui::RichText::new("❌").text_style(egui::TextStyle::Name("Controls".into()));
          let refresh = egui::RichText::new("🔃").text_style(egui::TextStyle::Name("Controls".into()));
          let theme = egui::RichText::new({
            if self.config.dark_mode {
              "☀"
            } else {
              "☽"
            }
          }).text_style(egui::TextStyle::Name("Theme".into()));
          let close_btn = ui.add(egui::Button::new(close));
          let refresh_btn = ui.add(egui::Button::new(refresh));
          let theme_btn = ui.add(egui::Button::new(theme));

          if close_btn.clicked() {
            frame.close();
          }
          if theme_btn.clicked() {
            self.config.dark_mode = !self.config.dark_mode;
          }
        });
      })
    });
  }

  pub fn render_config(&mut self, ctx: &egui::Context) {
    egui::Window::new("Configuration").show(ctx, |ui| {
      ui.add(Label::new("Enter your API KEY for newsapi.org"));
      let text_input = ui.text_edit_singleline(&mut self.config.api_key);
      if text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if let Err(e) = confy::store("headlines", None, HeadlinesConfig {
          dark_mode: self.config.dark_mode,
          api_key: self.config.api_key.to_string()
        }) {
          tracing::error!("Failed saving App State: {}", e);
        }

        self.api_key_initialized = true;
        
        tracing::error!("API KEY set");
      }
      ui.add(Label::new("If you haven't registered for the API KEY, head over to"));
      ui.add(Hyperlink::new("https://newsapi.org"));
    });
  }
}
