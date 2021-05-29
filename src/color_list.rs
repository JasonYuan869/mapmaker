use image::Rgb;

// The number of colors that Minecraft supports
const COLOR_COUNT: usize = 204;

// The colors available in Minecraft maps, first colour indexed at 4
// Indexes 0-3 are transparent.
const COLOR_LIST: [Rgb<u8>; COLOR_COUNT] = [
    Rgb ( [89, 125, 39] ),
    Rgb ( [109, 153, 48] ),
    Rgb ( [127, 178, 56] ),
    Rgb ( [67, 94, 29] ),
    Rgb ( [174, 164, 115] ),
    Rgb ( [213, 201, 140] ),
    Rgb ( [247, 233, 163] ),
    Rgb ( [130, 123, 86] ),
    Rgb ( [140, 140, 140] ),
    Rgb ( [171, 171, 171] ),
    Rgb ( [199, 199, 199] ),
    Rgb ( [105, 105, 105] ),
    Rgb ( [180, 0, 0] ),
    Rgb ( [220, 0, 0] ),
    Rgb ( [255, 0, 0] ),
    Rgb ( [135, 0, 0] ),
    Rgb ( [112, 112, 180] ),
    Rgb ( [138, 138, 220] ),
    Rgb ( [160, 160, 255] ),
    Rgb ( [84, 84, 135] ),
    Rgb ( [117, 117, 117] ),
    Rgb ( [144, 144, 144] ),
    Rgb ( [167, 167, 167] ),
    Rgb ( [88, 88, 88] ),
    Rgb ( [0, 87, 0] ),
    Rgb ( [0, 106, 0] ),
    Rgb ( [0, 124, 0] ),
    Rgb ( [0, 65, 0] ),
    Rgb ( [180, 180, 180] ),
    Rgb ( [220, 220, 220] ),
    Rgb ( [255, 255, 255] ),
    Rgb ( [135, 135, 135] ),
    Rgb ( [115, 118, 129] ),
    Rgb ( [141, 144, 158] ),
    Rgb ( [164, 168, 184] ),
    Rgb ( [86, 88, 97] ),
    Rgb ( [106, 76, 54] ),
    Rgb ( [130, 94, 66] ),
    Rgb ( [151, 109, 77] ),
    Rgb ( [79, 57, 40] ),
    Rgb ( [79, 79, 79] ),
    Rgb ( [96, 96, 96] ),
    Rgb ( [112, 112, 112] ),
    Rgb ( [59, 59, 59] ),
    Rgb ( [45, 45, 180] ),
    Rgb ( [55, 55, 220] ),
    Rgb ( [64, 64, 255] ),
    Rgb ( [33, 33, 135] ),
    Rgb ( [100, 84, 50] ),
    Rgb ( [123, 102, 62] ),
    Rgb ( [143, 119, 72] ),
    Rgb ( [75, 63, 38] ),
    Rgb ( [180, 177, 172] ),
    Rgb ( [220, 217, 211] ),
    Rgb ( [255, 252, 245] ),
    Rgb ( [135, 133, 129] ),
    Rgb ( [152, 89, 36] ),
    Rgb ( [186, 109, 44] ),
    Rgb ( [216, 127, 51] ),
    Rgb ( [114, 67, 27] ),
    Rgb ( [125, 53, 152] ),
    Rgb ( [153, 65, 186] ),
    Rgb ( [178, 76, 216] ),
    Rgb ( [94, 40, 114] ),
    Rgb ( [72, 108, 152] ),
    Rgb ( [88, 132, 186] ),
    Rgb ( [102, 153, 216] ),
    Rgb ( [54, 81, 114] ),
    Rgb ( [161, 161, 36] ),
    Rgb ( [197, 197, 44] ),
    Rgb ( [229, 229, 51] ),
    Rgb ( [121, 121, 27] ),
    Rgb ( [89, 144, 17] ),
    Rgb ( [109, 176, 21] ),
    Rgb ( [127, 204, 25] ),
    Rgb ( [67, 108, 13] ),
    Rgb ( [170, 89, 116] ),
    Rgb ( [208, 109, 142] ),
    Rgb ( [242, 127, 165] ),
    Rgb ( [128, 67, 87] ),
    Rgb ( [53, 53, 53] ),
    Rgb ( [65, 65, 65] ),
    Rgb ( [76, 76, 76] ),
    Rgb ( [40, 40, 40] ),
    Rgb ( [108, 108, 108] ),
    Rgb ( [132, 132, 132] ),
    Rgb ( [153, 153, 153] ),
    Rgb ( [81, 81, 81] ),
    Rgb ( [53, 89, 108] ),
    Rgb ( [65, 109, 132] ),
    Rgb ( [76, 127, 153] ),
    Rgb ( [40, 67, 81] ),
    Rgb ( [89, 44, 125] ),
    Rgb ( [109, 54, 153] ),
    Rgb ( [127, 63, 178] ),
    Rgb ( [67, 33, 94] ),
    Rgb ( [36, 53, 125] ),
    Rgb ( [44, 65, 153] ),
    Rgb ( [51, 76, 178] ),
    Rgb ( [27, 40, 94] ),
    Rgb ( [72, 53, 36] ),
    Rgb ( [88, 65, 44] ),
    Rgb ( [102, 76, 51] ),
    Rgb ( [54, 40, 27] ),
    Rgb ( [72, 89, 36] ),
    Rgb ( [88, 109, 44] ),
    Rgb ( [102, 127, 51] ),
    Rgb ( [54, 67, 27] ),
    Rgb ( [108, 36, 36] ),
    Rgb ( [132, 44, 44] ),
    Rgb ( [153, 51, 51] ),
    Rgb ( [81, 27, 27] ),
    Rgb ( [17, 17, 17] ),
    Rgb ( [21, 21, 21] ),
    Rgb ( [25, 25, 25] ),
    Rgb ( [13, 13, 13] ),
    Rgb ( [176, 168, 54] ),
    Rgb ( [215, 205, 66] ),
    Rgb ( [250, 238, 77] ),
    Rgb ( [132, 126, 40] ),
    Rgb ( [64, 154, 150] ),
    Rgb ( [79, 188, 183] ),
    Rgb ( [92, 219, 213] ),
    Rgb ( [48, 115, 112] ),
    Rgb ( [52, 90, 180] ),
    Rgb ( [63, 110, 220] ),
    Rgb ( [74, 128, 255] ),
    Rgb ( [39, 67, 135] ),
    Rgb ( [0, 153, 40] ),
    Rgb ( [0, 187, 50] ),
    Rgb ( [0, 217, 58] ),
    Rgb ( [0, 114, 30] ),
    Rgb ( [91, 60, 34] ),
    Rgb ( [111, 74, 42] ),
    Rgb ( [129, 86, 49] ),
    Rgb ( [68, 45, 25] ),
    Rgb ( [79, 1, 0] ),
    Rgb ( [96, 1, 0] ),
    Rgb ( [112, 2, 0] ),
    Rgb ( [59, 1, 0] ),
    Rgb ( [147, 124, 113] ),
    Rgb ( [180, 152, 138] ),
    Rgb ( [209, 177, 161] ),
    Rgb ( [110, 93, 85] ),
    Rgb ( [112, 57, 25] ),
    Rgb ( [137, 70, 31] ),
    Rgb ( [159, 82, 36] ),
    Rgb ( [84, 43, 19] ),
    Rgb ( [105, 61, 76] ),
    Rgb ( [128, 75, 93] ),
    Rgb ( [149, 87, 108] ),
    Rgb ( [78, 46, 57] ),
    Rgb ( [79, 76, 97] ),
    Rgb ( [96, 93, 119] ),
    Rgb ( [112, 108, 138] ),
    Rgb ( [59, 57, 73] ),
    Rgb ( [131, 93, 25] ),
    Rgb ( [160, 114, 31] ),
    Rgb ( [186, 133, 36] ),
    Rgb ( [98, 70, 19] ),
    Rgb ( [72, 82, 37] ),
    Rgb ( [88, 100, 45] ),
    Rgb ( [103, 117, 53] ),
    Rgb ( [54, 61, 28] ),
    Rgb ( [112, 54, 55] ),
    Rgb ( [138, 66, 67] ),
    Rgb ( [160, 77, 78] ),
    Rgb ( [84, 40, 41] ),
    Rgb ( [40, 28, 24] ),
    Rgb ( [49, 35, 30] ),
    Rgb ( [57, 41, 35] ),
    Rgb ( [30, 21, 18] ),
    Rgb ( [95, 75, 69] ),
    Rgb ( [116, 92, 84] ),
    Rgb ( [135, 107, 98] ),
    Rgb ( [71, 56, 51] ),
    Rgb ( [61, 64, 64] ),
    Rgb ( [75, 79, 79] ),
    Rgb ( [87, 92, 92] ),
    Rgb ( [46, 48, 48] ),
    Rgb ( [86, 51, 62] ),
    Rgb ( [105, 62, 75] ),
    Rgb ( [122, 73, 88] ),
    Rgb ( [64, 38, 46] ),
    Rgb ( [53, 43, 64] ),
    Rgb ( [65, 53, 79] ),
    Rgb ( [76, 62, 92] ),
    Rgb ( [40, 32, 48] ),
    Rgb ( [53, 35, 24] ),
    Rgb ( [65, 43, 30] ),
    Rgb ( [76, 50, 35] ),
    Rgb ( [40, 26, 18] ),
    Rgb ( [53, 57, 29] ),
    Rgb ( [65, 70, 36] ),
    Rgb ( [76, 82, 42] ),
    Rgb ( [40, 43, 22] ),
    Rgb ( [100, 42, 32] ),
    Rgb ( [122, 51, 39] ),
    Rgb ( [142, 60, 46] ),
    Rgb ( [75, 31, 24] ),
    Rgb ( [26, 15, 11] ),
    Rgb ( [31, 18, 13] ),
    Rgb ( [37, 22, 16] ),
    Rgb ( [19, 11, 8] ),
];

pub struct RgbColorMap {
    // The size of the array is hardcoded at compile time to equal to the number of colors available
    pub colors: [Rgb<u8>; COLOR_COUNT],
}

impl RgbColorMap {
    // pub fn index_of(&self, color: &Rgb<u8>) -> usize {
    //     // Returns the first index of the specified color
    //     // If the color is not found, return 0
    //     match self.colors.iter().position(|&r| r.eq(color)) {
    //         Some(i) => i,
    //         _ => 0
    //     }
    // }

    pub fn map_indices(&self, color: &Rgb<u8>) -> (usize, [i8; 3]) {
        // Returns a tuple containing the index of the mapped color and the quantization error
        // for dithering
        if color.0 == [0 as u8, 0, 0] {
            return (119, [0, 0, 0]);
        }

        let mut distances: Vec<u32> = Vec::with_capacity(COLOR_COUNT);
        for c in &self.colors {
            // Convert to i32 to support negative numbers
            let distance = (c.0[0] as i32 - color.0[0] as i32).pow(2)
                + (c.0[1] as i32 - color.0[1] as i32).pow(2)
                + (c.0[2] as i32 - color.0[2] as i32).pow(2);
            distances.push(distance as u32);
        }
        let idx = distances.iter().enumerate().min_by(
            |x, y| x.1.cmp(y.1)).unwrap().0;

        let mut error: [i8; 3] = [0, 0, 0];
        for ((i, y), x) in error.iter_mut().zip(&color.0).zip(&self.colors[idx].0) {
            *i = (*y as i16 - *x as i16) as i8;
        }

        (idx + 4, error)
    }
}

pub static MINECRAFT_COLOR_MAP: RgbColorMap = RgbColorMap {
    colors: COLOR_LIST,
};