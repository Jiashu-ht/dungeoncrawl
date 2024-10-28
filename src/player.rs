use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Player { position }
    }

    /// 将player渲染到界面上
    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    /// 更新内部状态
    ///
    /// 根据按键方向更新位置
    #[rustfmt::skip]
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left    | VirtualKeyCode::A => Point::new(-1, 0),
                VirtualKeyCode::Right   | VirtualKeyCode::D => Point::new(1, 0),
                VirtualKeyCode::Up      | VirtualKeyCode::W => Point::new(0, -1),
                VirtualKeyCode::Down    | VirtualKeyCode::S => Point::new(0, 1),
                _ => Point::zero()
            };
            let new_position = self.position + delta;
            if map.can_enter_tile(&new_position) {
                self.position = new_position;
            }
        }
    }
}