use crate::GameData;
use crate::Unit;

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
