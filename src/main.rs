use efd::{na, uvec, PosedEfd2};
use four_bar::plot2d::*;

fn main() {
    const LENGTH: f64 = 6.36;
    let vectors = ANGLE
        .iter()
        .map(|a| uvec([a.cos(), a.sin()]))
        .collect::<Vec<_>>();
    let target_pose = PATH
        .iter()
        .zip(&vectors)
        .map(|(p, v)| na::Point::from(*p) + v.into_inner() * LENGTH)
        .map(|p| [p.x, p.y])
        .collect::<Vec<_>>();
    let efd = PosedEfd2::from_vectors_harmonic(PATH, vectors, true, 10);
    let curve = efd.curve_efd().generate(90);
    let pose = curve
        .iter()
        .zip(efd.pose_efd().generate(90))
        .map(|(p, v)| na::Point::from(*p) + uvec(v).into_inner() * LENGTH)
        .map(|p| [p.x, p.y])
        .collect::<Vec<_>>();
    let b = SVGBackend::new("test.svg", (1600, 1600));
    let mut fig = Figure::new(None)
        .add_line("Target", PATH, Style::Line, RED)
        .add_line("", &target_pose, Style::Circle, RED)
        .add_line("EFD", &curve, Style::Line, BLUE)
        .add_line("", &pose, Style::Circle, BLUE)
        .legend(LegendPos::LR);
    for (p, v) in PATH.iter().zip(&target_pose) {
        fig.push_line("", vec![*p, *v], Style::Line, RED);
    }
    for (p, v) in curve.iter().zip(&pose) {
        fig.push_line("", vec![*p, *v], Style::Line, BLUE);
    }
    fig.plot(b).unwrap();
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
