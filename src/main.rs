use efd::PosedEfd2;
use four_bar::plot::*;

fn main() {
    let vectors = ANGLE.iter().map(|a| [a.cos(), a.sin()]).collect::<Vec<_>>();
    let mut target_pose = vectors.clone();
    target_pose.insert(0, [0.; 2]);
    let mut pos_i = 0;
    target_pose.iter_mut().reduce(|a, b| {
        (0..2).for_each(|i| b[i] += POS[pos_i] * a[i]);
        pos_i += 1;
        b
    });
    target_pose.pop();
    let efd = PosedEfd2::from_uvec(PATH, vectors, true);
    // let curve = efd.curve_efd().generate(90);
    let pose = efd.pose_efd().generate_norm(90);
    efd.pose_efd()
        .as_geo()
        .inverse()
        .transform_inplace(&mut target_pose);
    let b = SVGBackend::new("test.svg", (1600, 1600));
    fb::Figure::new(None)
        .add_line("Target Pose", target_pose, Style::Line, RED)
        .add_line("EFD Pose", pose, Style::Line, BLUE)
        .legend(LegendPos::Hide)
        .plot(b)
        .unwrap();
}

const PATH: &[[f64; 2]] = &[
    [18.8, 12.1],
    [13.3, 18.1],
    [6.3, 19.8],
    [-0.4, 17.1],
    [-2.7, 10.3],
    [-1.1, 6.0],
    [0.2, 1.7],
    [3.4, -2.2],
    [7.8, -4.9],
];
const ANGLE: &[f64] = &[-0.9, 0., 0.7, 1.5, 2.8, -2.3, -2., -1.9, -2.1];
const POS: &[f64] = &[
    0.0,
    0.5215074810676462,
    0.46154009683390473,
    0.4628280149313514,
    0.45993628469767844,
    0.29396366382157324,
    0.28782478070497364,
    0.3232298754173244,
    0.33076245611534105,
];
