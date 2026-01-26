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
    obstacle: Obstacle,
    score: i32,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
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

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, SCREEN_HEIGHT - 10),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        // Draw top part of obstacle
        for y in 0..(self.gap_y - half_size) {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('#'));
        }
        // Draw bottom part of obstacle
        for y in (self.gap_y + half_size)..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('#'));
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let within_x = player.x == self.x;
        let hit_top = player.y < (self.gap_y - half_size);
        let hit_bottom = player.y > (self.gap_y + half_size);
        //result of collision
        within_x && (hit_top || hit_bottom)
    }
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
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

        ctx.print(0, 0, "Press Space to flap!");
        ctx.print(0, 1, format!("Score: {}", self.score));

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.moving();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap()
        }

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        self.obstacle.render(ctx, self.player.x);
        self.player.render(ctx);

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You Died!");
        ctx.print_centered(6, format!("Final Score: {}", self.score));
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
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
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
