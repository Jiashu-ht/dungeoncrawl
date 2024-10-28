use std::vec;

use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    /// 默认瓦片均为地板
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// 设置瓦片类型
    ///
    /// # Examples
    /// ```
    /// let mut map = Map::new();
    /// let in_map = Point::new(1, 1);
    /// assert_eq!(map.get_tile_type(&in_map), TileType::Floor);
    ///
    /// map.set_tile_type(&in_map, TileType::Wall);
    /// assert_eq!(map.get_tile_type(&in_map), TileType::Wall);
    /// ```
    #[allow(unused)]
    pub fn set_tile_type(&mut self, point: &Point, tile_type: TileType) {
        let idx = map_point_idx(point);
        self.tiles[idx] = tile_type;
    }

    /// 获取某点瓦片类型
    ///
    /// # Examples
    /// ```
    /// let map = Map::new();
    /// let in_map = Point::new(1, 1);
    ///
    /// assert_eq!(map.get_tile_type(in_map), TileType::Floor)
    /// ```
    pub fn get_tile_type(&self, point: &Point) -> TileType {
        self.tiles[map_point_idx(point)]
    }

    /// 如果点在地图中则返回其瓦片向量索引
    ///
    /// # Examples
    /// ```
    /// let map = Map::new();
    /// let in_map = Point::new(1, 1);
    /// let out_map = Point::new(-1, -1);
    ///
    /// assert_eq!(map.try_idx(&in_map), Some(map_point_idx(&in_map)));
    /// assert_eq!(map.try_idx(&out_map), None);
    /// ```
    pub fn try_idx(&self, point: &Point) -> Option<usize> {
        if !self.in_bounds(point) {
            return None;
        }
        Some(map_point_idx(point))
    }

    /// 将map渲染到界面上
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(&Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(x - camera.left_x, y - camera.top_y, YELLOW, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x - camera.left_x, y - camera.top_y, GREEN, BLACK, to_cp437('#'));
                        }
                    }
                }
            }
        }
    }

    /// 判断点是否在地图中
    ///
    /// # Examples
    /// ```
    /// let map = Map::new();
    /// let in_map = Point::new(1, 1);
    /// let out_map = Point::new(-1, -1);
    ///
    /// assert_eq!(map.in_bounds(&in_map), true);
    /// assert_eq!(map.in_bounds(&out_map), false);
    /// ```
    pub fn in_bounds(&self, point: &Point) -> bool {
        point.x >= 0 && point.x <= SCREEN_WIDTH && point.y >= 0 && point.y <= SCREEN_HEIGHT
    }

    /// 判断能否进入瓦片
    ///
    /// # Examples
    /// ```
    /// let mut map = Map::new();
    /// let in_map = Point::new(1, 1);
    /// assert_eq!(map.can_enter_tile(&in_map), true);
    ///
    /// map.set_tile_type(&in_map, TileType::Wall);
    /// assert_eq!(map.can_enter_tile(&in_map), false);
    /// ```
    pub fn can_enter_tile(&self, point: &Point) -> bool {
        self.in_bounds(point) && matches!(self.get_tile_type(point), TileType::Floor)
    }
}

/// 将地图坐标(x, y)转换成瓦片向量索引
///
/// # Examples
/// ```
/// let map = Map::new();
/// let idx = map_idx(1, 1);
///
/// assert!(matches!(map.tiles[idx], TileType::Floor))
/// ```
pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

/// 将点位置转换成瓦片向量索引
///
/// # Examples
/// ```
/// let map = Map::new();
/// let idx = map_idx(Point::new(1, 1));
///
/// assert!(matches!(map.tiles[idx], TileType::Floor))
/// ```
pub fn map_point_idx(point: &Point) -> usize {
    map_idx(point.x, point.y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_set_tile_type() {
        let mut map = Map::new();
        let in_map = Point::new(1, 1);
        assert_eq!(map.get_tile_type(&in_map), TileType::Floor);

        map.set_tile_type(&in_map, TileType::Wall);
        assert_eq!(map.get_tile_type(&in_map), TileType::Wall);
    }

    #[test]
    fn map_get_tile_type() {
        let map = Map::new();
        let in_map = Point::new(1, 1);

        assert_eq!(map.get_tile_type(&in_map), TileType::Floor);
    }

    #[test]
    fn map_try_idx() {
        let map = Map::new();
        let in_map = Point::new(1, 1);
        let out_map = Point::new(-1, -1);

        assert_eq!(map.try_idx(&in_map), Some(map_point_idx(&in_map)));
        assert_eq!(map.try_idx(&out_map), None);
    }

    #[test]
    fn map_in_bounds() {
        let map = Map::new();
        let in_map = Point::new(1, 1);
        let out_map = Point::new(-1, -1);

        assert_eq!(map.in_bounds(&in_map), true);
        assert_eq!(map.in_bounds(&out_map), false);
    }

    #[test]
    fn map_can_enter_tile() {
        let mut map = Map::new();
        let in_map = Point::new(1, 1);
        assert_eq!(map.can_enter_tile(&in_map), true);

        map.set_tile_type(&in_map, TileType::Wall);
        assert_eq!(map.can_enter_tile(&in_map), false);
    }

    #[test]
    fn mod_map_idx() {
        let map = Map::new();
        let idx = map_idx(1, 1);

        assert!(matches!(map.tiles[idx], TileType::Floor))
    }

    #[test]
    fn mod_map_point_idx() {
        let map = Map::new();
        let idx = map_idx(1, 1);

        assert!(matches!(map.tiles[idx], TileType::Floor))
    }
}
