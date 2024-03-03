use crate::GameData;
use crate::EntityId;

pub type TeamId = i8;

#[derive(Debug, Default, Copy, Clone)]
pub struct Unit {
    pub id: EntityId,
    // fields below could be Datamap/hashmap/vec to be a more struct of arrays instead of array of struct
}

#[derive(Debug)]
pub struct UnitData {
    pub id: EntityId,
    pub range: f32, // not really the place for range and cooldown, but better than unit, since unit is not indexes
    pub cooldown: f32,
    pub hp: f32,
    pub dead: bool,
    pub team: TeamId,
}

impl UnitData {
    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }
}


pub trait UnitSystem {

    fn units(&self) -> usize;
    fn unit(&self, idx: usize) -> &Unit;
    fn unit_data(& self, id: EntityId) -> &UnitData;
    fn unit_data_mut(&mut self, id: EntityId) -> &mut UnitData;

}

impl UnitSystem for GameData {
    fn units(&self) -> usize {
        self.units.len()
    }

    fn unit(&self, idx: usize) -> &Unit {
        self.units.get(idx).expect("System should not have called with idx outside scope")
    }

    fn unit_data(& self, id: EntityId) -> &UnitData {
        self.units_data.get(&id).expect("System should not have called with idx outside scope")
    }

    fn unit_data_mut(&mut self, id: EntityId) -> &mut UnitData{
        self.units_data.get_mut(&id).expect("System should not have called with idx outside scope")
    }
}
