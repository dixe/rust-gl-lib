use crate::GameData;
use crate::EntityId;



pub type TeamId = i8;

#[derive(Debug, Default, Copy, Clone)]
pub struct Unit {
    pub id: EntityId,
    // fields below could be Datamap/hashmap/vec to be a more struct of arrays instead of array of struct
    pub hp: f32,
    pub dead: bool,
    pub team: TeamId,
    pub range: f32,
    pub cooldown: f32
}

impl Unit {
    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}


pub trait UnitSystem {
    fn units(&self) -> usize;
    fn unit(&self, idx: usize) -> &Unit;
    fn unit_mut(&mut self, idx: usize) -> &mut Unit;
}

impl UnitSystem for GameData {
    fn units(&self) -> usize {
        self.units.len()
    }

    fn unit(&self, idx: usize) -> &Unit {
        self.units.get(idx).expect("System should not have called with idx outside scope")
    }

    fn unit_mut(&mut self, idx: usize) -> &mut Unit {
        self.units.get_mut(idx).expect("System should not have called with idx outside scope")
    }
}
