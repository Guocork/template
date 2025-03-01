use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub MyButton = <Button> {
        width: 150.0,
        height: 50.0,
        text: "Click me",
        draw_text: {
            text_style: {font_size: 18}
            color: #fff
        }

        draw_bg: {
            instance color: #f7af8d,
            instance color_hover: #f08e60,
            instance color_pressed: #e96020
            instance hover: 0.0
            instance pressed: 0.0

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    8.0
                )

                // sdf.stroke_keep(
                //     mix(
                //         mix(
                //             mix((#xFFFFFF20), (#x00000066), pow(self.pos.y, .2)),
                //             mix((#xFFFFFF66), (#x00000066), pow(self.pos.y, 0.25)),
                //             self.hover
                //         ),
                //         mix((#x000000AA), (#xFFFFFF20), pow(self.pos.y, 3)),
                //         self.pressed
                //     ),
                //     1.
                // );
                sdf.fill(
                    mix(
                        mix(self.color, self.color_hover, self.hover),
                        self.color_pressed,
                        self.pressed
                    )
                );

                return sdf.result
            }
        }
    }
}