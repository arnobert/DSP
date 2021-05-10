use f64;

enum sdrsamples{
    I = 1,
    IQ = 2
}

pub trait DSP{
    fn power(self) -> f64;
}

impl DSP for f64{
    fn power(self) -> f64{
        42.0
    }
}

impl DSP for u8{
    fn power(self) -> f64{
        23.0
    }
}


