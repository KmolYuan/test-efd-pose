use efd::PosedEfd2;
use four_bar::plot::*;

fn main() {
    const LENGTH: f64 = 7.29;
    let vectors = ANGLE.iter().map(|a| [a.cos(), a.sin()]).collect::<Vec<_>>();
    let target_pose = std::iter::zip(PATH, &vectors)
        .map(|(p, v)| std::array::from_fn(|i| p[i] + LENGTH * v[i]))
        .collect::<Vec<_>>();
    let tp_norm = efd::MotionSig::new(PATH, &vectors).as_t().to_vec();
    let efd = PosedEfd2::from_uvec(PATH, vectors);
    dbg!(efd.harmonic());
    let (curve, pose) = efd.into_inner();
    let target_curve = curve.as_geo().inverse().transform(PATH);
    let target_pose = curve.as_geo().inverse().transform(target_pose);
    let q_trans = pose.as_geo().trans();
    let curve = curve.recon_norm_by(&tp_norm);
    let pose = pose
        .recon_norm_by(&tp_norm)
        .into_iter()
        .map(|q| std::array::from_fn(|i| q_trans[i] + q[i]))
        .collect::<Vec<_>>();
    let b = SVGBackend::new("test.svg", (1600, 1600));
    let mut fig = fb::Figure::new(None)
        .add_line("Target", &target_curve, Style::Line, RED)
        .add_line("", &target_pose, Style::Circle, RED)
        .add_line("EFD Recon.", &curve, Style::DashDottedLine, BLUE)
        .add_line("", &pose, Style::Circle, BLUE)
        .legend(LegendPos::UR);
    for (p, q) in target_curve.iter().zip(&target_pose) {
        fig.push_line("", vec![*p, *q], Style::Line, RED);
    }
    for (p, q) in curve.iter().zip(&pose) {
        fig.push_line("", vec![*p, *q], Style::DashDottedLine, BLUE);
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
