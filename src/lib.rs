#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

#[macro_use]
extern crate assert_approx_eq;

pub mod adsr {
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
}

#[cfg(test)]
mod tests {

  use adsr::Envelope;
  use adsr::{lerp, lerp_cl};

  #[test]
  fn lerp_test1() {
    let start = 1.0;
    let end = 99.0;
    let out1 = 0.01;
    let out2 = 0.99;

    assert_approx_eq!( lerp( 1.0, start, end, out1, out2 ), 0.01 );
    assert_approx_eq!( lerp( 50.0, start, end, out1, out2 ), 0.50 );
    assert_approx_eq!( lerp( 99.0, start, end, out1, out2 ), 0.99 );

    // overshoot test

    assert_approx_eq!( lerp( 110.0, start, end, out1, out2 ), 1.10 );

    // undershoot test...
    assert_approx_eq!( lerp( 0.1, start, end, out1, out2 ), 0.001 );

    // negative undershoot?
    assert_approx_eq!( lerp( -1.0, start, end, out1, out2 ), -0.01 );
  }

  // clamped lerp tests...
  #[test]
  fn lerp_test2() {
    let start = 1.0;
    let end = 99.0;
    let out1 = 0.01;
    let out2 = 0.99;

    assert_approx_eq!( lerp_cl( 1.0, start, end, out1, out2 ), 0.01 );
    assert_approx_eq!( lerp_cl( 50.0, start, end, out1, out2 ), 0.50 );
    assert_approx_eq!( lerp_cl( 99.0, start, end, out1, out2 ), 0.99 );

    // overshoot test

    assert_approx_eq!( lerp_cl( 110.0, start, end, out1, out2 ), 0.99 );

    // undershoot test

    assert_approx_eq!( lerp_cl( 0.1, start, end, out1, out2 ), 0.01 );
  }

  #[test]
  fn fresh_envelope_test1() {
    let env1 = Envelope::new();

    assert_approx_eq!( env1.get_value( 0.0 ), 0.0 );
    assert_approx_eq!( env1.get_value( 1.0 ), 0.0 );
    assert!( !env1.is_running() );
  }

  #[test]
  fn simple_trigger_test1() {
    let mut env1 = Envelope::new();

    assert!( !env1.is_running() );
    env1.trigger( 1.0 );
    assert!( env1.is_running() );

    // since the trigger event happens at 1.0s, the 0.0s value should still show the envelope as 'off'
    assert_approx_eq!( env1.get_value( 0.0 ), 0.0 );
  }
}
