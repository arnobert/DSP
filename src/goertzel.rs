use std::f64::consts::PI;


pub struct DSPGoertzel{
    w_n_0 : f64,
    w_n_1 : f64,
    w_n_2 : f64,
    y_n : f64,
    fs : f64,
    freq_a : f64,
    leng : usize,
}

impl DSPGoertzel {
    pub fn init() -> DSPGoertzel {
        DSPGoertzel {w_n_0: 0.0,
                     w_n_1: 0.0,
                     w_n_2: 0.0,
                     y_n: 0.0,
                     fs: 8000.0,
                     freq_a: 697.0,
                     leng: 205,
        }
    }

    pub fn data_in(&mut self, dat: f64) {

        let bin = self.f_to_index();
        let afreq = 2.0*PI*bin / self.leng as f64;

        let cos_fac = afreq.cos();

        self.w_n_2 = self.w_n_1;
        self.w_n_1 = self.w_n_0;
        self.w_n_0 = dat + 2.0 * cos_fac * self.w_n_1 - self.w_n_2;



        self.y_n = self.w_n_1.powf(2.0) - 2.0*cos_fac*self.w_n_1*self.w_n_2 + self.w_n_2.powf(2.0);

        let g_power = self.y_n.sqrt();
        print!("{}\n", g_power);
    }


    fn f_to_index(&mut self) -> f64 {
       (self.freq_a /self.fs * self.leng as f64 ).round()
    }
}
