use std::collections::HashMap;

use crate::enums::Board;
use crate::level::Segment;

#[derive(Debug)]
pub struct CellMap(HashMap<(i32, i32, Board), CellContents>);

#[derive(Copy, Clone, Debug)]
pub enum CellContents {
    Empty,
    Player,
    Single((usize, Segment)),

    // invariant: .0 < .1
    Double((usize, Segment), (usize, Segment)),
}

impl CellMap {
    pub fn new() -> Self {
        CellMap(HashMap::new())
    }

    pub fn get(&self, loc: (i32, i32, Board)) -> CellContents {
        self.0
            .get(&loc)
            .cloned()
            .unwrap_or_else(|| CellContents::Empty)
    }

    pub fn clear(&mut self, loc: (i32, i32, Board)) {
        self.0.remove(&loc);
    }

    pub fn add_player(&mut self, loc: (i32, i32, Board)) -> bool {
        let contents = self.get(loc).clone();
        match contents {
            CellContents::Empty => {
                self.0.insert(loc, CellContents::Player);
                true
            }
            _ => false,
        }
    }

    pub fn add(&mut self, loc: (i32, i32, Board), index: usize, segment: &Segment) -> bool {
        let contents = self.get(loc).clone();
        match contents {
            CellContents::Empty => {
                // just add it like normal
                self.0.insert(loc, CellContents::Single((index, *segment)));
                true
            }
            CellContents::Single((index0, existing)) => {
                if existing.shape.is_opposite(&segment.shape) {
                    self.0.insert(
                        loc,
                        if *segment < existing {
                            CellContents::Double((index, *segment), (index0, existing))
                        } else {
                            CellContents::Double((index0, existing), (index, *segment))
                        },
                    );
                    true
                } else {
                    false
                }
            }
            CellContents::Player | CellContents::Double(_, _) => false,
        }
    }
}
