// Image Widget
// Draws an image in a specified bounding area.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use piston_window::*;

use crate::core::callbacks::*;
use crate::widget::config::*;
use crate::widget::widget::*;

/// This is the `ImageWidget`, which draws an image on the screen.
pub struct ImageWidget {
    config: Configurable,
    callbacks: CallbackStore,
    image: Box<G2dTexture>,
    image_size: crate::core::point::Size,
}

/// Implementation of the constructor for the `ImageWidget`.  Creates a new image object to be
/// displayed on the screen, given the image filename.
impl ImageWidget {
    /// Creates a new `TextWidget` object, requiring the current `PistonWindow`'s factory object
    /// (which can be cloned), the name of the font (filename in the `assets` directory), the
    /// text to display, and the font size in which to use.
    pub fn new(factory: &mut GfxFactory, image_name: String) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let texture = Texture::from_path(
            factory,
            &assets.join(image_name),
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();

        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
            image: Box::new(texture.clone()),
            image_size: crate::core::point::Size {
                w: texture.clone().get_size().0 as i32,
                h: texture.clone().get_size().1 as i32,
            },
        }
    }
}

/// Implementation of the `BoxWidget` object with the `Widget` traits implemented.
/// This implementation is similar to the `CanvasWidget`, but incorporates a drawable box inside
/// the widget.  Base widget is the `CanvasWidget`.
///
/// This is basically just a box with a fill color.  Use this to draw other things like buttons,
/// text widgets, and so on, if you need anything with a drawable border.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::main::*;
/// # use pushrod::widget::widget::*;
/// # use pushrod::widget::text_widget::*;
/// # fn main() {
/// let mut window: PistonWindow = WindowSettings::new("Pushrod Window", [800, 600])
///       .opengl(OpenGL::V3_2)
///       .resizable(false)
///       .build()
///       .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));
///    let factory: GfxFactory = window.factory.clone();
///    let mut prod: Pushrod = Pushrod::new(window);
///    let mut text_widget = TextWidget::new(
///       prod.get_factory(),
///       "OpenSans-Regular.ttf".to_string(),
///       "Welcome to Pushrod!".to_string(),
///       32,
///    );
///
///    text_widget.set_origin(8, 8);
///    text_widget.set_size(400, 40);
///    text_widget.set_color([0.75, 0.75, 1.0, 1.0]);
///    text_widget.set_text_color([0.0, 0.0, 1.0, 1.0]);
///    prod.widget_store.add_widget(Box::new(text_widget));
/// # }
/// ```
impl Widget for ImageWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }

    /// Draws the contents of the widget.
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size = self.get_size();
        let transform = c.transform.scale(
            size.w as f64 / self.image_size.w as f64,
            size.h as f64 / self.image_size.h as f64,
        );

        Image::new().draw(&*self.image, clip, transform, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
