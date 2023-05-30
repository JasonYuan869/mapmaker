use image::Rgb;
use kd_tree::KdTree3;
use typenum::U3;

use lazy_static::lazy_static;

/// The number of colors that Minecraft supports, excluding the 4 transparent ones.
const COLOR_COUNT: usize = 244;

pub const BLACK_INDEX: MapColor = 119;

/// A wrapper around `Rgb<u8>` that implements the `KdPoint` trait for use with the `kd-tree`.
/// Contains the original `Rgb<u8>` value and the index of that color in the Minecraft color list.
#[derive(Debug, Clone, Copy)]
pub struct MinecraftRgb(Rgb<u8>, MapColor);
impl kd_tree::KdPoint for MinecraftRgb {
    type Scalar = isize;
    type Dim = U3;

    fn at(&self, k: usize) -> Self::Scalar {
        self.0[k] as isize
    }
}

/// This array contains `COLOR_COUNT` entries, indexed starting from 4 (colors 0-3 are transparent).
/// Each element in this array represents the corresponding Minecraft map color at that index.
/// As of Minecraft 1.17, there are 244 colours in this array.
#[allow(overflowing_literals)]
const COLOR_LIST: [MinecraftRgb; COLOR_COUNT] = [
    MinecraftRgb(Rgb([89, 125, 39]), 4),
    MinecraftRgb(Rgb([109, 153, 48]), 5),
    MinecraftRgb(Rgb([127, 178, 56]), 6),
    MinecraftRgb(Rgb([67, 94, 29]), 7),
    MinecraftRgb(Rgb([174, 164, 115]), 8),
    MinecraftRgb(Rgb([213, 201, 140]), 9),
    MinecraftRgb(Rgb([247, 233, 163]), 10),
    MinecraftRgb(Rgb([130, 123, 86]), 11),
    MinecraftRgb(Rgb([140, 140, 140]), 12),
    MinecraftRgb(Rgb([171, 171, 171]), 13),
    MinecraftRgb(Rgb([199, 199, 199]), 14),
    MinecraftRgb(Rgb([105, 105, 105]), 15),
    MinecraftRgb(Rgb([180, 0, 0]), 16),
    MinecraftRgb(Rgb([220, 0, 0]), 17),
    MinecraftRgb(Rgb([255, 0, 0]), 18),
    MinecraftRgb(Rgb([135, 0, 0]), 19),
    MinecraftRgb(Rgb([112, 112, 180]), 20),
    MinecraftRgb(Rgb([138, 138, 220]), 21),
    MinecraftRgb(Rgb([160, 160, 255]), 22),
    MinecraftRgb(Rgb([84, 84, 135]), 23),
    MinecraftRgb(Rgb([117, 117, 117]), 24),
    MinecraftRgb(Rgb([144, 144, 144]), 25),
    MinecraftRgb(Rgb([167, 167, 167]), 26),
    MinecraftRgb(Rgb([88, 88, 88]), 27),
    MinecraftRgb(Rgb([0, 87, 0]), 28),
    MinecraftRgb(Rgb([0, 106, 0]), 29),
    MinecraftRgb(Rgb([0, 124, 0]), 30),
    MinecraftRgb(Rgb([0, 65, 0]), 31),
    MinecraftRgb(Rgb([180, 180, 180]), 32),
    MinecraftRgb(Rgb([220, 220, 220]), 33),
    MinecraftRgb(Rgb([255, 255, 255]), 34),
    MinecraftRgb(Rgb([135, 135, 135]), 35),
    MinecraftRgb(Rgb([115, 118, 129]), 36),
    MinecraftRgb(Rgb([141, 144, 158]), 37),
    MinecraftRgb(Rgb([164, 168, 184]), 38),
    MinecraftRgb(Rgb([86, 88, 97]), 39),
    MinecraftRgb(Rgb([106, 76, 54]), 40),
    MinecraftRgb(Rgb([130, 94, 66]), 41),
    MinecraftRgb(Rgb([151, 109, 77]), 42),
    MinecraftRgb(Rgb([79, 57, 40]), 43),
    MinecraftRgb(Rgb([79, 79, 79]), 44),
    MinecraftRgb(Rgb([96, 96, 96]), 45),
    MinecraftRgb(Rgb([112, 112, 112]), 46),
    MinecraftRgb(Rgb([59, 59, 59]), 47),
    MinecraftRgb(Rgb([45, 45, 180]), 48),
    MinecraftRgb(Rgb([55, 55, 220]), 49),
    MinecraftRgb(Rgb([64, 64, 255]), 50),
    MinecraftRgb(Rgb([33, 33, 135]), 51),
    MinecraftRgb(Rgb([100, 84, 50]), 52),
    MinecraftRgb(Rgb([123, 102, 62]), 53),
    MinecraftRgb(Rgb([143, 119, 72]), 54),
    MinecraftRgb(Rgb([75, 63, 38]), 55),
    MinecraftRgb(Rgb([180, 177, 172]), 56),
    MinecraftRgb(Rgb([220, 217, 211]), 57),
    MinecraftRgb(Rgb([255, 252, 245]), 58),
    MinecraftRgb(Rgb([135, 133, 129]), 59),
    MinecraftRgb(Rgb([152, 89, 36]), 60),
    MinecraftRgb(Rgb([186, 109, 44]), 61),
    MinecraftRgb(Rgb([216, 127, 51]), 62),
    MinecraftRgb(Rgb([114, 67, 27]), 63),
    MinecraftRgb(Rgb([125, 53, 152]), 64),
    MinecraftRgb(Rgb([153, 65, 186]), 65),
    MinecraftRgb(Rgb([178, 76, 216]), 66),
    MinecraftRgb(Rgb([94, 40, 114]), 67),
    MinecraftRgb(Rgb([72, 108, 152]), 68),
    MinecraftRgb(Rgb([88, 132, 186]), 69),
    MinecraftRgb(Rgb([102, 153, 216]), 70),
    MinecraftRgb(Rgb([54, 81, 114]), 71),
    MinecraftRgb(Rgb([161, 161, 36]), 72),
    MinecraftRgb(Rgb([197, 197, 44]), 73),
    MinecraftRgb(Rgb([229, 229, 51]), 74),
    MinecraftRgb(Rgb([121, 121, 27]), 75),
    MinecraftRgb(Rgb([89, 144, 17]), 76),
    MinecraftRgb(Rgb([109, 176, 21]), 77),
    MinecraftRgb(Rgb([127, 204, 25]), 78),
    MinecraftRgb(Rgb([67, 108, 13]), 79),
    MinecraftRgb(Rgb([170, 89, 116]), 80),
    MinecraftRgb(Rgb([208, 109, 142]), 81),
    MinecraftRgb(Rgb([242, 127, 165]), 82),
    MinecraftRgb(Rgb([128, 67, 87]), 83),
    MinecraftRgb(Rgb([53, 53, 53]), 84),
    MinecraftRgb(Rgb([65, 65, 65]), 85),
    MinecraftRgb(Rgb([76, 76, 76]), 86),
    MinecraftRgb(Rgb([40, 40, 40]), 87),
    MinecraftRgb(Rgb([108, 108, 108]), 88),
    MinecraftRgb(Rgb([132, 132, 132]), 89),
    MinecraftRgb(Rgb([153, 153, 153]), 90),
    MinecraftRgb(Rgb([81, 81, 81]), 91),
    MinecraftRgb(Rgb([53, 89, 108]), 92),
    MinecraftRgb(Rgb([65, 109, 132]), 93),
    MinecraftRgb(Rgb([76, 127, 153]), 94),
    MinecraftRgb(Rgb([40, 67, 81]), 95),
    MinecraftRgb(Rgb([89, 44, 125]), 96),
    MinecraftRgb(Rgb([109, 54, 153]), 97),
    MinecraftRgb(Rgb([127, 63, 178]), 98),
    MinecraftRgb(Rgb([67, 33, 94]), 99),
    MinecraftRgb(Rgb([36, 53, 125]), 100),
    MinecraftRgb(Rgb([44, 65, 153]), 101),
    MinecraftRgb(Rgb([51, 76, 178]), 102),
    MinecraftRgb(Rgb([27, 40, 94]), 103),
    MinecraftRgb(Rgb([72, 53, 36]), 104),
    MinecraftRgb(Rgb([88, 65, 44]), 105),
    MinecraftRgb(Rgb([102, 76, 51]), 106),
    MinecraftRgb(Rgb([54, 40, 27]), 107),
    MinecraftRgb(Rgb([72, 89, 36]), 108),
    MinecraftRgb(Rgb([88, 109, 44]), 109),
    MinecraftRgb(Rgb([102, 127, 51]), 110),
    MinecraftRgb(Rgb([54, 67, 27]), 111),
    MinecraftRgb(Rgb([108, 36, 36]), 112),
    MinecraftRgb(Rgb([132, 44, 44]), 113),
    MinecraftRgb(Rgb([153, 51, 51]), 114),
    MinecraftRgb(Rgb([81, 27, 27]), 115),
    MinecraftRgb(Rgb([17, 17, 17]), 116),
    MinecraftRgb(Rgb([21, 21, 21]), 117),
    MinecraftRgb(Rgb([25, 25, 25]), 118),
    MinecraftRgb(Rgb([13, 13, 13]), 119),
    MinecraftRgb(Rgb([176, 168, 54]), 120),
    MinecraftRgb(Rgb([215, 205, 66]), 121),
    MinecraftRgb(Rgb([250, 238, 77]), 122),
    MinecraftRgb(Rgb([132, 126, 40]), 123),
    MinecraftRgb(Rgb([64, 154, 150]), 124),
    MinecraftRgb(Rgb([79, 188, 183]), 125),
    MinecraftRgb(Rgb([92, 219, 213]), 126),
    MinecraftRgb(Rgb([48, 115, 112]), 127),
    MinecraftRgb(Rgb([52, 90, 180]), 128),
    MinecraftRgb(Rgb([63, 110, 220]), 129),
    MinecraftRgb(Rgb([74, 128, 255]), 130),
    MinecraftRgb(Rgb([39, 67, 135]), 131),
    MinecraftRgb(Rgb([0, 153, 40]), 132),
    MinecraftRgb(Rgb([0, 187, 50]), 133),
    MinecraftRgb(Rgb([0, 217, 58]), 134),
    MinecraftRgb(Rgb([0, 114, 30]), 135),
    MinecraftRgb(Rgb([91, 60, 34]), 136),
    MinecraftRgb(Rgb([111, 74, 42]), 137),
    MinecraftRgb(Rgb([129, 86, 49]), 138),
    MinecraftRgb(Rgb([68, 45, 25]), 139),
    MinecraftRgb(Rgb([79, 1, 0]), 140),
    MinecraftRgb(Rgb([96, 1, 0]), 141),
    MinecraftRgb(Rgb([112, 2, 0]), 142),
    MinecraftRgb(Rgb([59, 1, 0]), 143),
    MinecraftRgb(Rgb([147, 124, 113]), 144),
    MinecraftRgb(Rgb([180, 152, 138]), 145),
    MinecraftRgb(Rgb([209, 177, 161]), 146),
    MinecraftRgb(Rgb([110, 93, 85]), 147),
    MinecraftRgb(Rgb([112, 57, 25]), 148),
    MinecraftRgb(Rgb([137, 70, 31]), 149),
    MinecraftRgb(Rgb([159, 82, 36]), 150),
    MinecraftRgb(Rgb([84, 43, 19]), 151),
    MinecraftRgb(Rgb([105, 61, 76]), 152),
    MinecraftRgb(Rgb([128, 75, 93]), 153),
    MinecraftRgb(Rgb([149, 87, 108]), 154),
    MinecraftRgb(Rgb([78, 46, 57]), 155),
    MinecraftRgb(Rgb([79, 76, 97]), 156),
    MinecraftRgb(Rgb([96, 93, 119]), 157),
    MinecraftRgb(Rgb([112, 108, 138]), 158),
    MinecraftRgb(Rgb([59, 57, 73]), 159),
    MinecraftRgb(Rgb([131, 93, 25]), 160),
    MinecraftRgb(Rgb([160, 114, 31]), 161),
    MinecraftRgb(Rgb([186, 133, 36]), 162),
    MinecraftRgb(Rgb([98, 70, 19]), 163),
    MinecraftRgb(Rgb([72, 82, 37]), 164),
    MinecraftRgb(Rgb([88, 100, 45]), 165),
    MinecraftRgb(Rgb([103, 117, 53]), 166),
    MinecraftRgb(Rgb([54, 61, 28]), 167),
    MinecraftRgb(Rgb([112, 54, 55]), 168),
    MinecraftRgb(Rgb([138, 66, 67]), 169),
    MinecraftRgb(Rgb([160, 77, 78]), 170),
    MinecraftRgb(Rgb([84, 40, 41]), 171),
    MinecraftRgb(Rgb([40, 28, 24]), 172),
    MinecraftRgb(Rgb([49, 35, 30]), 173),
    MinecraftRgb(Rgb([57, 41, 35]), 174),
    MinecraftRgb(Rgb([30, 21, 18]), 175),
    MinecraftRgb(Rgb([95, 75, 69]), 176),
    MinecraftRgb(Rgb([116, 92, 84]), 177),
    MinecraftRgb(Rgb([135, 107, 98]), 178),
    MinecraftRgb(Rgb([71, 56, 51]), 179),
    MinecraftRgb(Rgb([61, 64, 64]), 180),
    MinecraftRgb(Rgb([75, 79, 79]), 181),
    MinecraftRgb(Rgb([87, 92, 92]), 182),
    MinecraftRgb(Rgb([46, 48, 48]), 183),
    MinecraftRgb(Rgb([86, 51, 62]), 184),
    MinecraftRgb(Rgb([105, 62, 75]), 185),
    MinecraftRgb(Rgb([122, 73, 88]), 186),
    MinecraftRgb(Rgb([64, 38, 46]), 187),
    MinecraftRgb(Rgb([53, 43, 64]), 188),
    MinecraftRgb(Rgb([65, 53, 79]), 189),
    MinecraftRgb(Rgb([76, 62, 92]), 190),
    MinecraftRgb(Rgb([40, 32, 48]), 191),
    MinecraftRgb(Rgb([53, 35, 24]), 192),
    MinecraftRgb(Rgb([65, 43, 30]), 193),
    MinecraftRgb(Rgb([76, 50, 35]), 194),
    MinecraftRgb(Rgb([40, 26, 18]), 195),
    MinecraftRgb(Rgb([53, 57, 29]), 196),
    MinecraftRgb(Rgb([65, 70, 36]), 197),
    MinecraftRgb(Rgb([76, 82, 42]), 198),
    MinecraftRgb(Rgb([40, 43, 22]), 199),
    MinecraftRgb(Rgb([100, 42, 32]), 200),
    MinecraftRgb(Rgb([122, 51, 39]), 201),
    MinecraftRgb(Rgb([142, 60, 46]), 202),
    MinecraftRgb(Rgb([75, 31, 24]), 203),
    MinecraftRgb(Rgb([26, 15, 11]), 204),
    MinecraftRgb(Rgb([31, 18, 13]), 205),
    MinecraftRgb(Rgb([37, 22, 16]), 206),
    MinecraftRgb(Rgb([19, 11, 8]), 207),
    MinecraftRgb(Rgb([133, 33, 34]), 208),
    MinecraftRgb(Rgb([163, 41, 42]), 209),
    MinecraftRgb(Rgb([189, 48, 49]), 210),
    MinecraftRgb(Rgb([100, 25, 25]), 211),
    MinecraftRgb(Rgb([104, 44, 68]), 212),
    MinecraftRgb(Rgb([127, 54, 83]), 213),
    MinecraftRgb(Rgb([148, 63, 97]), 214),
    MinecraftRgb(Rgb([78, 33, 51]), 215),
    MinecraftRgb(Rgb([64, 17, 20]), 216),
    MinecraftRgb(Rgb([79, 21, 25]), 217),
    MinecraftRgb(Rgb([92, 25, 29]), 218),
    MinecraftRgb(Rgb([48, 13, 15]), 219),
    MinecraftRgb(Rgb([15, 88, 94]), 220),
    MinecraftRgb(Rgb([18, 108, 115]), 221),
    MinecraftRgb(Rgb([22, 126, 134]), 222),
    MinecraftRgb(Rgb([11, 66, 70]), 223),
    MinecraftRgb(Rgb([40, 100, 98]), 224),
    MinecraftRgb(Rgb([50, 122, 120]), 225),
    MinecraftRgb(Rgb([58, 142, 140]), 226),
    MinecraftRgb(Rgb([30, 75, 74]), 227),
    MinecraftRgb(Rgb([60, 31, 43]), 228),
    MinecraftRgb(Rgb([74, 37, 53]), 229),
    MinecraftRgb(Rgb([86, 44, 62]), 230),
    MinecraftRgb(Rgb([45, 23, 32]), 231),
    MinecraftRgb(Rgb([14, 127, 93]), 232),
    MinecraftRgb(Rgb([17, 155, 114]), 233),
    MinecraftRgb(Rgb([20, 180, 133]), 234),
    MinecraftRgb(Rgb([10, 95, 70]), 235),
    MinecraftRgb(Rgb([70, 70, 70]), 236),
    MinecraftRgb(Rgb([86, 86, 86]), 237),
    MinecraftRgb(Rgb([100, 100, 100]), 238),
    MinecraftRgb(Rgb([52, 52, 52]), 239),
    MinecraftRgb(Rgb([152, 123, 103]), 240),
    MinecraftRgb(Rgb([186, 150, 126]), 241),
    MinecraftRgb(Rgb([216, 175, 147]), 242),
    MinecraftRgb(Rgb([114, 92, 77]), 243),
    MinecraftRgb(Rgb([89, 117, 105]), 244),
    MinecraftRgb(Rgb([109, 144, 129]), 245),
    MinecraftRgb(Rgb([127, 167, 150]), 246),
    MinecraftRgb(Rgb([67, 88, 79]), 247),
];

/// Wrapper around KdTree3
pub struct MinecraftColorTree(KdTree3<MinecraftRgb>);
pub type MapColor = i8;
pub type RgbDifference = [i16; 3];

impl MinecraftColorTree {
    /// Returns the closest color in the Minecraft color palette and the distance to it.
    pub fn find_closest(&self, color: &Rgb<u8>) -> (MapColor, RgbDifference) {
        // Check if black, don't propagate the error
        if color == &Rgb([0, 0, 0]) {
            return (BLACK_INDEX, [0, 0, 0]);
        }

        // Cast to MinecraftRgb to use the KdTree, the index is ignored
        let mc_rgb = MinecraftRgb(*color, 0);
        let nearest = self.0.nearest(&mc_rgb).unwrap();

        // KdTree returns the squared distance in `nearest`, but we want the actual distance
        let distance = [
            color.0[0] as i16 - nearest.item.0[0] as i16,
            color.0[1] as i16 - nearest.item.0[1] as i16,
            color.0[2] as i16 - nearest.item.0[2] as i16,
        ];

        // Return the MC index of the color and the distance
        (nearest.item.1, distance)
    }
}

lazy_static! {
    pub static ref MINECRAFT_COLOR_TREE: MinecraftColorTree =
        MinecraftColorTree(KdTree3::build(Vec::from(COLOR_LIST)));
}
