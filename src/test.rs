use makepad_widgets::*;


live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;


    pub TestGener = {{TestGener}} {
        <Button> {}
    }
}

#[derive(Widget, Live, LiveHook)]
pub struct TestGener {
    #[deref]
    deref: View,
}

impl Widget for TestGener {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        DrawStep::done()
        
    }

    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        
    }
}

impl WidgetMatchEvent for TestGener {
    fn handle_actions(&mut self, _cx: &mut Cx, _e:&Actions, _scope: &mut Scope) {
        
    }
}

