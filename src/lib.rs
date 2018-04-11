#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod ADSR {
pub struct Envelope {
  triggered: Option<f64>,
  attack: f64,
  decay: f64,
  sustain: f64,
  release: f64,
}

impl Envelope {
  pub fn trigger(&mut self, time: f64) {
    if self.triggered == None {
      self.triggered = Some(time);
    } else {
      // should really have a 'retriggerable' option somewhere...
      self.triggered = Some(time);
    }
  }

  pub fn set_attack( &mut self, attack: f64) {
    self.attack = attack;
  }

  pub fn get_value( &self, time: f64 ) -> f64 {
    0.0
  }
}

impl Default for Envelope {
  fn default() -> Envelope {
    Envelope {
      triggered: None,
      attack: 0.0,
      decay: 0.0,
      sustain: 1.0,
      release: 0.0,
    }
  }
}
}
#[cfg(test)]

use ADSR::Envelope;

mod tests {
  #[test]
  fn it_works() {

    let env1 = ADSR::Envelope{};

    assert!(env1.get_value( 0.0 ).approx_eq(0.0));
  }
}
