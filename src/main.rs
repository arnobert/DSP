//mod dsp_8b10b_encoder;
//mod dsp_8b10b_decoder;
mod goertzel;

use std::f64::consts::PI;

fn main() {

    //let mut y = dsp_8b10b_encoder::DSP8b10bEncoder::init();
    //let mut z = dsp_8b10b_decoder::DSP8b10bDecoder::init();
    let mut g = goertzel::DSPGoertzel::init();


    // TEST

    //let mut d_in : u16;
    //let mut d_out : u16;

    let freq : f64 = 697.0;

    for n in 0..205 {
       g.data_in(((2.0*PI*freq* n as f64) / 8000.0 ).sin());
    }



    //for n in 0..255 {
    //   d_in = y.encode(n as u8);
    //   d_out = z.decode(d_in);

    //    if d_out == n as u16 {
            //print!("{} OK\n", n);
     //   }
       // else {
            //print!("ERROR AT {}\n", n);
    //    }

    //}

}
