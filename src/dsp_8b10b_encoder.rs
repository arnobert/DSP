pub struct DSP8b10bEncoder{
    disparity : i8,
}


impl DSP8b10bEncoder {
    pub fn init() -> DSP8b10bEncoder {
        let tdisp : i8 = -1;
        DSP8b10bEncoder {disparity: tdisp}
    }

    pub fn swap_disp(&mut self) {
        if self.disparity == -1 {
            self.disparity = 1;
        }
        else {
            self.disparity = -1;
        }
    }

    pub fn calc_disp(&self, word: u16) -> i8{
        let c0 = word.count_zeros() as i8 - 6 ;
        let c1 = word.count_ones() as i8;

        c1 - c0
    }

    pub fn encode(&mut self, word: u8) -> u16{
        let old_disp : i8 = self.disparity;


        // 000XXXXX
        let lobits : u8 = word & 0b00011111;
        let n_lobits : u8;


        if old_disp == -1 {
            n_lobits = match lobits {
                0 => 0b100111,
                1 => 0b011101,
                2 => 0b101101,
                3 => 0b110001,
                4 => 0b110101,
                5 => 0b101001,
                6 => 0b011001,
                7 => 0b111000,
                8 => 0b111001,
                9 => 0b100101,
                10 => 0b010101,
                11 => 0b110100,
                12 => 0b001101,
                13 => 0b101100,
                14 => 0b011100,
                15 => 0b010111,
                16 => 0b011011,
                17 => 0b100011,
                18 => 0b010011,
                19 => 0b110010,
                20 => 0b001011,
                21 => 0b101010,
                22 => 0b011010,
                23 => 0b111010,
                24 => 0b110011,
                25 => 0b100110,
                26 => 0b010110,
                27 => 0b110110,
                28 => 0b001110,
                29 => 0b101110,
                30 => 0b011110,
                31 => 0b101011,
                _ => 0
            };
        }

        else {
            n_lobits = match lobits {
                0 => 0b011000,
                1 => 0b100010,
                2 => 0b010010,
                3 => 0b110001,
                4 => 0b001010,
                5 => 0b101001,
                6 => 0b011001,
                7 => 0b000111,
                8 => 0b000110,
                9 => 0b100101,
                10 => 0b010101,
                11 => 0b110100,
                12 => 0b001101,
                13 => 0b101100,
                14 => 0b011100,
                15 => 0b101000,
                16 => 0b100100,
                17 => 0b100011,
                18 => 0b010011,
                19 => 0b110010,
                20 => 0b001011,
                21 => 0b101010,
                22 => 0b011010,
                23 => 0b000101,
                24 => 0b001100,
                25 => 0b100110,
                26 => 0b010110,
                27 => 0b001001,
                28 => 0b001110,
                29 => 0b010001,
                30 => 0b100001,
                31 => 0b010100,
                _ => 0
            };
        }


        // XXX00000
        let hibits : u8 = (word & 0b11100000) >> 5;
        let n_hibits : u16;

        if old_disp == -1 {
            n_hibits = match hibits {
                0 => 0b1011,
                1 => 0b1001,
                2 => 0b0101,
                3 => 0b1100,
                4 => 0b1101,
                5 => 0b1010,
                6 => 0b0110,
                7 => 0b1110,
                _ => 0
            };
        }
        else {
            n_hibits = match hibits {
                0 => 0b0100,
                1 => 0b1001,
                2 => 0b0101,
                3 => 0b0011,
                4 => 0b0010,
                5 => 0b1010,
                6 => 0b0110,
                7 => 0b0001,
                _ => 0
            }
        };

        let w_out : u16 = (n_hibits << 6) + n_lobits as u16;
        let n_disp = self.calc_disp(w_out);

        if n_disp != 0 {
            self.swap_disp();
        }

        w_out
    }

}

