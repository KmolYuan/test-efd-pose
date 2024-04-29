use efd::PosedEfd2;
use four_bar::plot::*;

fn main() {
    const LENGTH: f64 = 7.29;
    let vectors = ANGLE.iter().map(|a| [a.cos(), a.sin()]).collect::<Vec<_>>();
    let target_pose = std::iter::zip(PATH, &vectors)
        .map(|(p, v)| std::array::from_fn(|i| p[i] + LENGTH * v[i]))
        .collect::<Vec<_>>();
    let efd = PosedEfd2::from_uvec(PATH, vectors);
    dbg!(efd.harmonic());
    let (curve, pose) = efd.into_inner();
    let geo_inv = curve.as_geo().inverse();
    let target_curve = geo_inv.transform(PATH);
    let target_pose = geo_inv.transform(target_pose);
    let curve = curve.recon_norm(90);
    // let curve = curve.recon_norm_by(&tp_norm);
    let pose = pose.recon(90);
    // let pose = pose.recon_by(&tp_norm);
    let b = SVGBackend::new("test.svg", (1600, 1600));
    fb::Figure::new()
        .add_pose("Target", &target_curve, &target_pose, Style::Line, RED)
        .add_pose("EFD Recon.", &curve, &pose, Style::DashDottedLine, BLUE)
        .legend(LegendPos::UR)
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
