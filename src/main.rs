use glutin_window::GlutinWindow;
use opengl_graphics::{
    Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings,
};
use piston::event_loop::{EventSettings, Events};
use piston::{EventLoop, RenderEvent, WindowSettings};

use crate::renderer::{Renderer,RenderSettings};
use crate::algorithms::{Algorithms};
use crate::main_cotroller::{MainController};

mod renderer;
mod algorithms;
mod main_cotroller;

static FONT: &str = "assets/fonts/Roboto-Regular.ttf";

fn main() {
    //create the window
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sorting Algorithm Visualisation", (640, 480))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .vsync(true);
    let mut window: GlutinWindow =
        settings.build().expect("Could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    //create value to store old window size
    let mut old_window_size : [f64;2] = [0.0,0.0];
    //starting list 
    let starting_values = vec![10,20,30,40,50,60,70,20,90];

    //create render
    let mut render = Renderer::new(100,0,starting_values.clone(),RenderSettings::new());
    //create controller
    let mut contoller = MainController::new(&starting_values);
    //create sorting algorithm
    let mut sort_algorithms = Algorithms::new(starting_values.clone());

    //text 
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new(FONT, (), texture_settings)
        .expect(&format!("failed to load font `{}`", FONT));

    //main loop
    while let Some(e) = events.next(&mut window) {
        contoller.event(&e,  &mut sort_algorithms);
        //update render
        render.update_values(&contoller.display_values,&contoller.updated_values, &contoller.sorted, contoller.hud_values.clone());
        //render events
        if let Some(args) = e.render_args() {
            //resize the chart if resized
            if args.window_size != old_window_size{
                old_window_size = args.window_size;
                render.update_size(args.window_size);
            }
            
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                //clear background 
                clear([0.3; 4], g);
                //draw values
                render.draw( glyphs,&c, g);
            });
        }
        
    }

}
