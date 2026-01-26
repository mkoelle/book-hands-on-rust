use bracket_lib::prelude::*;

const FRAME_DURATION: f32 = 75.0;
const GRAVITY: f32 = 0.2;
const FLAP_STRENGTH: f32 = -2.0;
const SCREEN_TOP: i32 = 0;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

enum GameMode {
    Menu,
    Playing,
    End,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    fn moving(&mut self) {
        // Advance player distance
        self.x += 1;
        // Apply gravity
        if self.velocity < 2.0 {
            self.velocity += GRAVITY;
        }
        // Update vertical position
        self.y += self.velocity as i32;
        // Prevent going above the screen
        if self.y < SCREEN_TOP {
            self.y = SCREEN_TOP;
        }
    }
    fn flap(&mut self) {
        self.velocity = FLAP_STRENGTH;
    }
}

impl State {
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.moving();
        }

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => self.player.flap(),
                _ => {}
            }
        }

        self.player.render(ctx);

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You Died!");
        ctx.print_centered(8, "(R) Restart");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Flappy").build()?;

    main_loop(context, State::new())
}
