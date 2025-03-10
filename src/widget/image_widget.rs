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

use graphics::*;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};

use crate::widget::config::*;
use crate::widget::widget::*;

/// Draws an image.
pub struct ImageWidget {
    config: Configurable,
    image: Texture,
    image_size: crate::core::point::Size,
}

impl ImageWidget {
    /// Constructor.  Requires the name of the image to be drawn.  The image is loaded locally from
    /// the `assets/` directory of your application.
    pub fn new(image_name: String) -> Self {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let texture =
            Texture::from_path(&assets.join(image_name), &TextureSettings::new()).unwrap();
        let texture_width = texture.get_width() as i32;
        let texture_height = texture.get_height() as i32;

        Self {
            config: Configurable::new(),
            image: texture,
            image_size: crate::core::point::Size {
                w: texture_width,
                h: texture_height,
            },
        }
    }
}

impl Widget for ImageWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn draw(&mut self, c: Context, g: &mut GlGraphics, clip: &DrawState) {
        let size = self.config().get_size(CONFIG_BODY_SIZE);
        let transform = c.transform.scale(
            size.w as f64 / self.image_size.w as f64,
            size.h as f64 / self.image_size.h as f64,
        );

        Image::new().draw(&self.image, clip, transform, g);

        // Then clear invalidation.
        self.clear_invalidate();
    }
}
