use makepad_widgets::*;
use std::path::{Path, PathBuf};

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
        grab_key_focus: false
    }

    SlideshowOverlay = <View> {
        cursor: Arrow,
        capture_overload: true,

        left_button = <SlideshowButton> {
            draw_icon: {
                svg_file: (LEFT_ARROW)
            }
        }
        <Filler> {}
        right_button = <SlideshowButton> {
            draw_icon: {
                svg_file: (RIGHT_ARROW)
            }
        }
    }

    Slideshow = <View> {
        flow: Overlay,

        image = <Image> {
            width: Fill,
            height: Fill,
            fit: Biggest,
            source: (PLACEHOLDER)
        }

        overlay = <SlideshowOverlay> {}
    }

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <View> {
                    slideshow = <Slideshow> {}
                }
            }
        }
        placeholder: (PLACEHOLDER)
    }
}

#[derive(Default)]
struct State {
    image_paths: Vec<PathBuf>,
    current_image_index: usize,
}

impl State {
    fn num_images(&self) -> usize {
        self.image_paths.len()
    }
}

#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,

    #[live]
    placeholder: LiveDependency,

    #[rust]
    state: State,
}

impl App {
    fn go_to_previous_image(&mut self, cx: &mut Cx) {
        if self.state.current_image_index > 0 {
            self.set_current_image(cx, self.state.current_image_index - 1);
        }
    }

    fn go_to_next_image(&mut self, cx: &mut Cx) {
        if self.state.current_image_index + 1 < self.state.num_images() {
            self.set_current_image(cx, self.state.current_image_index + 1);
        }
    }

    fn load_image_paths(&mut self, cx: &mut Cx, dir: &Path) {
        self.state.image_paths.clear();

        for entry in dir.read_dir().unwrap() {
            let path = entry.unwrap().path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "png" || extension == "jpg" || extension == "jpeg" {
                        self.state.image_paths.push(path);
                    }
                }
            }
        }

        self.set_current_image(cx, 0);
    }

    fn set_current_image(&mut self, cx: &mut Cx, image_index: usize) {
        self.state.current_image_index = image_index;

        let image = self.ui.image(id!(slideshow.image));
        if let Some(path) = self.state.image_paths.get(image_index) {
            image.load_image_file_by_path_async(cx, &path).unwrap();
        } else {
            let placeholder = self.placeholder.as_str();
            image.load_image_dep_by_path(cx, placeholder).unwrap();
        }

        self.ui.redraw(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.load_image_paths(cx, "./images".as_ref());
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(left_button)).clicked(&actions) {
            self.go_to_previous_image(cx);
        }
        if self.ui.button(id!(right_button)).clicked(&actions) {
            self.go_to_next_image(cx);
        }

        if let Some(event) = self.ui.view(id!(overlay)).key_down(&actions) {
            match event.key_code {
                KeyCode::ArrowLeft => self.go_to_previous_image(cx),
                KeyCode::ArrowRight => self.go_to_next_image(cx),
                _ => {}
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
