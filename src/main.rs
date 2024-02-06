use four_bar::{efd, plot::*};

fn main() {
    const T: &[f64] = &[
        0.0,
        0.4470294233619797,
        0.833440423024198,
        1.5495480938915984,
        2.1238246470304687,
        2.8305776616959863,
        3.4500229817089303,
        4.102175632876617,
        4.620648779537075,
        5.107200935652393,
        5.407539805003856,
        5.721174374784138,
        5.9748002222709875,
    ];
    let curve = efd::tests::CURVE2D.to_vec();
    let efd = efd::Efd2::from_curve(&curve, false);
    let mut curve_recon = efd.generate(90);
    curve_recon.drain(..curve_recon.len() - 20);
    let (t, _) = efd::get_target_pos(&curve, false);
    let t_norm = efd.generate_by(&t);
    let t = efd.generate_by(T);
    // let pose = efd.pose_efd().generate_norm(90);
    // efd.pose_efd()
    //     .as_geo()
    //     .inverse()
    //     .transform_inplace(&mut target_pose);
    let b = SVGBackend::new("test.svg", (1600, 1600));
    fb::Figure::new(None)
        // .add_line("Target", curve, Style::Line, BLUE)
        .add_line("Target", &curve, Style::Line, BLUE)
        .add_line("EFD Recon.", &curve_recon, Style::Line, GREEN)
        .add_line("t", &t, Style::Cross, RED)
        .add_line("t - θ1", &t_norm, Style::Circle, RED)
        .add_line("", &t_norm, Style::DashDottedLine, RED)
        .legend(LegendPos::UL)
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
