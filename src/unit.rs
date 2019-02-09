
use crate::damage;

#[derive(Debug)]
pub enum Unit {
  Dead,
  Alive {
    name:  String,
    hp:    i32,
    vulns: damage::Scales
  },
}

impl Unit {
  pub fn deal_damage(self, dmg: damage::Values) -> Unit {
    if let Unit::Alive { name, hp, vulns } = self {
      let hp = hp - dmg.resist(&vulns).sum();
      if hp > 0 {
        Unit::Alive { name, hp, vulns }
      } else {
        Unit::Dead
      }
    } else {
      Unit::Dead
    }
  }
}

