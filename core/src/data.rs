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
    pub dimensions: (u32, u32),
    pub player1: PlayerData,
    pub player2: PlayerData,
    pub goal1: (i32, i32),
    pub goal2: (i32, i32),
    pub blocks: Vec<BlockData>,
}

impl LevelData {
    pub fn empty() -> LevelData {
        LevelData {
            dimensions: (5, 5),
            player1: PlayerData {
                position: (0, 0),
                color: (66, 134, 244),
            },
            player2: PlayerData {
                position: (0, 0),
                color: (244, 83, 65),
            },
            goal1: (4, 4),
            goal2: (4, 4),
            blocks: Vec::new(),
        }
    }
}
