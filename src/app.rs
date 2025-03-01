use makepad_widgets::*;

use crate::button_view;

live_design!{
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::button_view::MyButton;

    ICON_LOGO = dep("crate://self/assets/images/logo_makepad.png");

    App = {{App}} {
        ui: <Window> {
            body = <View>{
                flow: Down,
                spacing: 25.0, 
                align: {x: 0.5, y: 0.5},
                show_bg: true,
                draw_bg: {
                    color: #fff
                }

                logo_image = <Image> {
                    width: 600.0,
                    height: 100.0,
                    source: (ICON_LOGO),
                }

                intro = <Label> {
                    text: "A secure, flexible, and high-performance GUI framework for Rust",
                    draw_text: {
                        text_style: {font_size: 13}
                        color: #f08e60
                    }
                }

                click_button = <MyButton> {}
                
                countnumber = <Label> {
                    text: "0",
                    draw_text: {
                        text_style: {font_size: 20}
                        color: #000
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    counter: u32
}

impl LiveRegister for App {
    fn live_register(cx:&mut Cx) {
        makepad_widgets::live_design(cx);
        button_view::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(click_button)).clicked(actions) {
            self.counter += 1;
            self.ui.label(id!(countnumber)).set_text(cx, &self.counter.to_string());
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}