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
    let efd = PosedEfd2::from_vectors(PATH, vectors, true);
    dbg!(efd.harmonic());
    let (curve, pose) = efd.generate_by(THETA, LENGTH);
    let b = SVGBackend::new("posed.svg", (1600, 1600));
    let mut fig = Figure::new(None)
        .add_line("Target", PATH, Style::Line, RED)
        .add_line("", &target_pose, Style::Circle, RED)
        .add_line("EFD Recon.", &curve, Style::Line, BLUE)
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
const THETA: &[f64] = &[
    0.,
    0.5215074810676462,
    0.983047577901551,
    1.4458755928329023,
    1.9058118775305808,
    2.199775541352154,
    2.4876003220571277,
    2.810830197474452,
    std::f64::consts::PI,
];
