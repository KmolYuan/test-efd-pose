use efd::{na, uvec, PosedEfd2};
use four_bar::plot2d::*;

fn main() {
    let vectors = ANGLE
        .iter()
        .map(|a| uvec([a.cos(), a.sin()]))
        .collect::<Vec<_>>();
    let target_pose = PATH
        .iter()
        .zip(&vectors)
        .map(|(p, v)| na::Point::from(*p) + v.into_inner())
        .map(|p| [p.x, p.y])
        .collect::<Vec<_>>();
    let efd = PosedEfd2::from_vectors_harmonic(PATH, vectors, false, 9);
    let curve = efd.curve_efd().generate(90);
    let pose = curve
        .iter()
        .zip(efd.pose_efd().generate(90))
        .map(|(p, v)| na::Point::from(*p) + uvec(v).into_inner())
        .map(|p| [p.x, p.y])
        .collect::<Vec<_>>();
    let b = SVGBackend::new("test.svg", (1600, 1600));
    Figure::new(None)
        .add_line("Target", PATH, Style::Line, RED)
        .add_line("Target Pose", target_pose, Style::Circle, RED)
        .add_line("EFD", curve, Style::Line, BLUE)
        .add_line("EFD Pose", pose, Style::Circle, BLUE)
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
