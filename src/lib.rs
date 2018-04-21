#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

pub struct Envelope {
  triggered: Option< f64 >,
  released: Option< f64 >,
  attack: f64,
  decay: f64,
  sustain: f64,
  release: f64,
}

enum EnvelopeStage {
  Untriggered,
  Attack,
  Decay,
  Sustain,
  Release,
  PostRelease,
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

fn get_trigger_time( env: &Envelope ) -> f64 {
  match env.triggered {
    None => 0.0,
    Some( time ) => time,
  }
}

fn get_envelope_stage( env: &Envelope, time: f64 ) -> EnvelopeStage {
  match env.triggered {
    None => EnvelopeStage::Untriggered,
    Some( trigger_time ) => match env.released {
      None => {
        if time < ( trigger_time + env.attack ) {
          return EnvelopeStage::Attack;
        } else if time < ( trigger_time + env.attack + env.decay ) {
          return EnvelopeStage::Decay;
        } else {
          return EnvelopeStage::Sustain;
        }
      }
      Some( release_time ) => {
        if ( time - release_time ) < env.release {
          return EnvelopeStage::Release;
        } else {
          return EnvelopeStage::PostRelease;
        }
      }
    },
  }
}

impl Envelope {
  pub fn new() -> Envelope {
    return Envelope { triggered: None,
                      released: None,
                      attack: 0.0,
                      decay: 0.0,
                      sustain: 1.0,
                      release: 0.0, };
  }

  pub fn new_adsr( attack: f64, decay: f64, sustain: f64, release: f64 ) -> Envelope {
    return Envelope { triggered: None,
                      released: None,
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
    let stage = get_envelope_stage( self, time );
    let trig = get_trigger_time( self );

    match stage {
      EnvelopeStage::Untriggered => 0.0,
      EnvelopeStage::Attack => lerp_cl( time - trig, 0.0, self.attack, 0.0, 1.0 ),
      EnvelopeStage::Decay => lerp_cl( time - trig, self.attack, self.decay, 1.0, self.sustain ),
      EnvelopeStage::Sustain => self.sustain,
      EnvelopeStage::Release => {
        lerp_cl( time /* - rel */, 0.0, self.release, self.sustain, 0.0 )
      }
      EnvelopeStage::PostRelease => 0.0,
    }
  }

  pub fn is_running( &self ) -> bool {
    ( self.triggered != None )
  }
}
