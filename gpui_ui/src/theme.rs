use gpui::Hsla;

pub struct TradingTheme;

impl TradingTheme {
    pub fn background() -> Hsla {
        gpui::rgb(0x000000).into()
    }

    pub fn panel_background() -> Hsla {
        gpui::rgb(0x0a0a0a).into()
    }

    pub fn card_background() -> Hsla {
        gpui::rgb(0x1a1a1a).into()
    }

    pub fn border() -> Hsla {
        gpui::rgb(0x2a2a2a).into()
    }

    pub fn text_primary() -> Hsla {
        gpui::rgb(0xffffff).into()
    }

    pub fn text_secondary() -> Hsla {
        gpui::rgb(0xb0b0b0).into()
    }

    pub fn text_muted() -> Hsla {
        gpui::rgb(0x707070).into()
    }

    pub fn green() -> Hsla {
        gpui::rgb(0x00d094).into()
    }

    pub fn red() -> Hsla {
        gpui::rgb(0xff4a4a).into()
    }

    pub fn blue() -> Hsla {
        gpui::rgb(0x3b82f6).into()
    }

    pub fn yellow() -> Hsla {
        gpui::rgb(0xffa500).into()
    }

    pub fn hover_background() -> Hsla {
        gpui::rgb(0x1e1e1e).into()
    }

    pub fn selected_background() -> Hsla {
        gpui::rgb(0x0d4f3a).into()
    }

    pub fn divider() -> Hsla {
        gpui::rgb(0x1f1f1f).into()
    }

    pub fn status_success() -> Hsla {
        gpui::rgb(0x10b981).into()
    }

    pub fn status_warning() -> Hsla {
        gpui::rgb(0xf59e0b).into()
    }

    pub fn status_error() -> Hsla {
        gpui::rgb(0xef4444).into()
    }
}
