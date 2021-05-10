pub struct DSP8b10bDecoder{
    disparity : i8,
}

impl DSP8b10bDecoder {
    pub fn init() -> DSP8b10bDecoder {
        let tdisp : i8 = -1;
        DSP8b10bDecoder {disparity: tdisp}
    }
}
