#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

pub struct Envelope {
  triggered: Option< f64 >,
  attack: f64,
  decay: f64,
  sustain: f64,
  release: f64,
}

// unclamped lerp...
pub fn lerp( input: f64, x1: f64, x2: f64, y1: f64, y2: f64 ) -> f64 {
  let x_range = x2 - x1;
  let y_range = y2 - y1;
  y1 + ( ( input - x1 ) / x_range * y_range )
}

//clamped lerp
pub fn lerp_cl( input: f64, x1: f64, x2: f64, y1: f64, y2: f64 ) -> f64 {
  let x_range = x2 - x1;
  let y_range = y2 - y1;
  let out = y1 + ( ( ( input - x1 ) / x_range ) * y_range );
  if out < y1 {
    y1
  } else if out > y2 {
    y2
  } else {
    out
  }
}

impl Envelope {
  pub fn new() -> Envelope {
    return Envelope { triggered: None,
                      attack: 0.0,
                      decay: 0.0,
                      sustain: 1.0,
                      release: 0.0, };
  }

  pub fn new_adsr( attack: f64, decay: f64, sustain: f64, release: f64 ) -> Envelope {
    return Envelope { triggered: None,
                      attack: attack,
                      decay: decay,
                      sustain: sustain,
                      release: release, };
  }

  pub fn trigger( &mut self, time: f64 ) {
    if self.triggered == None {
      self.triggered = Some( time );
    } else {
      // should really have a 'retriggerable' option somewhere...
      self.triggered = Some( time );
    }
  }

  pub fn set_attack( &mut self, attack: f64 ) {
    self.attack = attack;
  }

  pub fn set_decay( &mut self, decay: f64 ) {
    self.decay = decay;
  }

  pub fn set_sustain( &mut self, sustain: f64 ) {
    self.sustain = sustain;
  }

  pub fn set_release( &mut self, release: f64 ) {
    self.release = release;
  }

  pub fn get_value( &self, time: f64 ) -> f64 {
    match self.triggered {
      None => 0.0, // If the envelope is off, then return 0.0 always
      Some( value ) => {
        if time >= value {
          1.0
        } else {
          0.0
        }
      }
    }
  }

  pub fn is_running( &self ) -> bool {
    ( self.triggered != None )
  }
}
