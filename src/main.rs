mod reactor;
use reactor::*;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("game_name", "author_name")
           .build()
           .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    layers: Vec<Layer>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {

        set_default_filter(_ctx, FilterMode::Linear);

        let mut ui = MyGame { layers: Vec::new(), };

        let mut layer1 = Layer::new(0., 0.);
        let mut layer2 = Layer::new(0., 0.);

        let mut el1 = Element::new(10.0, 40.0, 100.0, 200.0);
        el1.set_background_color(ColorRGBA::new(255, 255, 0, 255));

        let mut el2 = Element::new(100.0, 10.0, 100.0, 200.0);
        el2.set_background_color(ColorRGBA::new(255, 255, 0, 255));


        layer1.elements.push(el1);
        layer2.elements.push(el2);

        //println!("{:?}", layer2.search_by_key(&mut el2.key).unwrap().key);

        layer2.textnodes.push(TextNode::new("bar foo".to_string(), _ctx, 10., 10.));

        ui.layers.push(layer1);
        ui.layers.push(layer2);

        ui
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let cords = graphics::screen_coordinates(ctx);

        for layer in &mut self.layers {
            layer.calc(cords.w, cords.h);
            layer.draw();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        for layer in &mut self.layers {
            let rgba: &[u8] = &layer.layout[..]; // Layout - rendered context
            let imge_to_render = Image::from_rgba8(ctx, layer.w as u16, layer.h as u16, rgba)?;
            let _result = graphics::draw(ctx, &imge_to_render, DrawParam::default())?;
            for text in &mut layer.textnodes {
                let fragment = TextFragment::new(text.content.clone()).color(BLACK).font(text.font).scale(Scale::uniform(24.));
                let _result = graphics::draw(ctx, &Text::new(fragment), (na::Point2::new(text.x, text.y),))?;
            }
        }
        //thread::sleep(Duration::from_millis(1));
        graphics::present(ctx)
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        // Search el key on click
        for layer in &mut self.layers {
            for el in &mut layer.elements {
                if el.is_in(_x, _y) {
                    println!("{:?}", &mut el.key);
                }
            }
        }
        
    }
}
