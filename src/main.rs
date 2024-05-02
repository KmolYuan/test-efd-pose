use efd::PosedEfd2;
use four_bar::{plot::*, MFourBar};

fn main() {
    let fb =
        ron::de::from_reader::<_, MFourBar>(std::fs::File::open("test.open.ron").unwrap()).unwrap();
    let (target_curve, vectors) = fb.pose(40);
    // let tp_norm = efd::MotionSig::new(&target_curve, &vectors).as_t().to_vec();
    let efd = PosedEfd2::from_uvec(&target_curve, &vectors);
    dbg!(efd.harmonic());
    let (curve, pose) = efd.into_inner();
    let geo = curve.as_geo();
    let curve = curve.recon(90);
    // let curve = curve.recon_by(&tp_norm);
    let pose = geo.transform(pose.recon(90));
    // let pose = geo.transform(pose.recon_by(&tp_norm));
    let b = SVGBackend::new("test.svg", (1600, 1600));
    fb::Figure::new()
        .add_pose(
            "Target",
            (&target_curve, vectors, geo.scale()),
            Style::Line,
            RED,
            false,
        )
        .add_line_data(LineData {
            label: "EFD Recon.".into(),
            line: LineType::Pose {
                curve: curve.into(),
                pose: pose.into(),
                is_frame: true,
            },
            style: Style::DashedLine,
            color: BLUE.into(),
        })
        .legend(LegendPos::LR)
        .plot(b)
        .unwrap();
}
