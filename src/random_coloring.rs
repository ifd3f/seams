use palette::{convert::TryIntoColor, Hsl, Hsv, Srgb};
use rand::{Rng, SeedableRng};

pub type ColorRng = rand_chacha::ChaCha8Rng;
pub type Rgb8 = Srgb<u8>;

pub trait ColorProfile {
    fn gen_color(&self, preseeded_rng: impl Rng) -> Rgb8;
}

pub trait ColorProfileExt: ColorProfile {
    fn for_text(&self, text: &str) -> Rgb8;
}

impl<P: ColorProfile> ColorProfileExt for P {
    fn for_text(&self, text: &str) -> Rgb8 {
        let rng = text_into_rng(text);
        self.gen_color(rng)
    }
}

pub fn text_into_rng(text: &str) -> impl Rng {
    let hash = md5::compute(text.as_bytes());
    let mut seed = [0u8; 32];
    seed[0..16].copy_from_slice(&hash[0..16]);
    seed[16..32].copy_from_slice(&hash[0..16]);

    ColorRng::from_seed(seed)
}

pub struct HueColorProfile {
    pub saturation: f32,
    pub lightness: f32,
}

pub const PASTEL: HueColorProfile = HueColorProfile {
    saturation: 0.5,
    lightness: 0.9,
};

pub const TINT: HueColorProfile = HueColorProfile {
    saturation: 0.2,
    lightness: 0.9,
};

pub const DARK: HueColorProfile = HueColorProfile {
    saturation: 0.7,
    lightness: 0.5,
};

impl ColorProfile for HueColorProfile {
    fn gen_color(&self, mut preseeded_rng: impl Rng) -> Rgb8 {
        let h: f32 = preseeded_rng.gen::<f32>() * 360.0;
        let hsl = Hsl::new_srgb(h, self.saturation, self.lightness);
        let rgbf: Srgb<f32> = hsl.try_into_color().unwrap();
        rgbf.into_format()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("")]
    #[case("uwu")]
    #[case("askldfjkl")]
    pub fn profile_generates_consistent_colors(#[case] text: &str) {
        let a = PASTEL.for_text(text);
        let b = PASTEL.for_text(text);

        assert_eq!(a, b);
    }
}
