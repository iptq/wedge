#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub position: (i32, i32),
    pub color: (u32, u32, u32),
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub movable: bool,
    pub orientation: u32,
    pub color: (u32, u32, u32),
    pub segments: Vec<[i32; 4]>,
}

#[derive(Debug, Deserialize)]
pub struct LevelData {
    pub dimensions: [u32; 2],
    pub player1: PlayerData,
    pub player2: PlayerData,
    pub goal1: (i32, i32),
    pub goal2: (i32, i32),
    pub blocks: Vec<BlockData>,
}
