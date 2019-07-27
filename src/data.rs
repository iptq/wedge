#[derive(Debug, Deserialize)]
pub struct PlayerData {
    pub position: [u32; 2],
    pub color: [u32; 3],
}

#[derive(Debug, Deserialize)]
pub struct BlockData {
    pub movable: bool,
    pub push_dir: u32,
    pub position: [u32; 2],
    pub color: [u32; 3],
    pub segments: Vec<[u32; 4]>,
}

#[derive(Debug, Deserialize)]
pub struct LevelData {
    pub dimensions: [u32; 2],
    pub player1: PlayerData,
    pub player2: PlayerData,
    pub goal1: [u32; 2],
    pub goal2: [u32; 2],
    pub blocks: Vec<BlockData>,
}
