mod dsp_8b10b_encoder;
mod dsp_8b10b_decoder;

fn main() {

    let mut y = dsp_8b10b_encoder::DSP8b10bEncoder::init();
    let mut z = dsp_8b10b_decoder::DSP8b10bDecoder::init();

    // TEST

    let mut d_in : u16;
    let mut d_out : u16;

    for n in 0..255 {
       d_in = y.encode(n as u8);
       d_out = z.decode(d_in);

        if d_out == n as u16 {
            print!("{} OK\n", n);
        }
        else {
            print!("ERROR AT {}\n", n);
        }

    }


}
