use gpui::*;
use gpui_component::{button::*, input::{Input, InputState}, *};

pub struct App {

}

impl Render for App {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .v_flex()
            .child(
                TitleBar::new()
            )
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);
    
    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        let theme = Theme::global_mut(cx);
        
        theme.apply_config(&{if dark_light::detect().unwrap() == dark_light::Mode::Dark { theme.dark_theme.clone() } else { theme.light_theme.clone() }});
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions {
                titlebar: None,
                ..Default::default()
            }, |window, cx| {
                let view = cx.new(|_| App {
                    
                });
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}