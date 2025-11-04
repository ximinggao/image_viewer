use makepad_widgets::*;

live_design! {
    use link::widgets::*;

    LEFT_ARROW = dep("crate://self/resources/left_arrow.svg");
    RIGHT_ARROW = dep("crate://self/resources/right_arrow.svg");
    PLACEHOLDER = dep("crate://self/resources/placeholder.png");

    SlideshowButton = <Button> {
        margin: 0,
        width: 50,
        height: Fill,
        draw_bg: {
            color: #FFF0,
            color_down: #FFF2,
        },
        icon_walk: {
            width: 10,
        },
    }

    SlideshowOverlay = <View> {
        left_button = <SlideshowButton> {
            draw_icon: {
                svg_file: (LEFT_ARROW)
            }
        },
        <Filler> {}
        right_button = <SlideshowButton> {
            draw_icon: {
                svg_file: (RIGHT_ARROW)
            }
        },
    }

    Slideshow = <View> {
        flow: Overlay,

        image = <Image> {
            width: Fill,
            height: Fill,
            fit: Biggest,
            source: (PLACEHOLDER)
        },

        overlay = <SlideshowOverlay> {},
    }

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    <Slideshow> {}
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
