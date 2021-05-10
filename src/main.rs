mod dsp_8b10b_encoder;

fn main() {

    let mut y = dsp_8b10b_encoder::DSP8b10bEncoder::init();


    let da = y.encode(4);
    print!("Word 1: {}\n",da);
    let db = y.encode(4);
    print!("Word 2: {}\n",db);
    let dc = y.encode(4);
    print!("Word 3: {}\n",dc);

}
