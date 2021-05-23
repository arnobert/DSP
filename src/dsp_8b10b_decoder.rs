pub struct DSP8b10bDecoder{
    disparity : i8,
}

impl DSP8b10bDecoder {
    pub fn init() -> DSP8b10bDecoder {
        let tdisp : i8 = -1;
        DSP8b10bDecoder {disparity: tdisp}
    }


    pub fn decode(&mut self, word: u16) -> u8{

        //0000XXXXXX

        let lobits : u16 = word & 0b0000111111;
        let n_lobits : u16;

        n_lobits = match lobits {
            0b100111 => 0,
            0b011101 => 1,
            0b101101 => 2,
            0b110001 => 3,
            0b110101 => 4,
            0b101001 => 5,
            0b011001 => 6,
            0b111000 => 7,
            0b111001 => 8,
            0b100101 => 9,
            0b010101 => 10,
            0b110100 => 11,
            0b001101 => 12,
            0b101100 => 13,
            0b011100 => 14,
            0b010111 => 15,
            0b011011 => 16,
            0b100011 => 17,
            0b010011 => 18,
            0b110010 => 19,
            0b001011 => 20,
            0b101010 => 21,
            0b011010 => 22,
            0b111010 => 23,
            0b110011 => 24,
            0b100110 => 25,
            0b010110 => 26,
            0b110110 => 27,
            0b001110 => 28,
            0b101110 => 29,
            0b011110 => 30,
            0b101011 => 31,

            0b011000 => 0,
            0b100010 => 1,
            0b010010 => 2,

            0b001010 => 4,


            0b000111 => 7,
            0b000110 => 8,






            0b101000 => 15,
            0b100100 => 16,






            0b000101 => 23,
            0b001100 => 24,


            0b001001 => 27,

            0b010001 => 29,
            0b100001 => 30,
            0b010100 => 31,



            _ => 0
        };

        //XXXX000000
        0
    }
}