extern crate adsr;

#[macro_use]
extern crate assert_approx_eq;

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
