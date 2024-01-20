use four_bar::{
    efd::na,
    mech::mfb::{MNormFourBar, NormFourBar, Stat, UnNorm},
    mh::{De, Solver},
    plot::{self, *},
    syn::{MFbSyn, Mode},
    MFourBar,
};

fn main() {
    let target_fb = MFourBar {
        unnorm: UnNorm {
            p1x: -2.,
            p1y: -3.,
            a: 0.2,
            l2: 12.,
        },
        norm: MNormFourBar {
            base: NormFourBar {
                l1: 5.,
                l3: 4.,
                l4: 9.,
                l5: 5.,
                g: 1.,
                stat: Stat::C1B1,
            },
            e: 0.1411971,
        },
    };
    const LENGTH: f64 = 7.29;
    let (target_curve, vectors) = target_fb.pose(60);
    // ===
    // let target_curve = PATH;
    // const LENGTH: f64 = 6.36;
    // let vectors = ANGLE
    //     .iter()
    //     .map(|a| [a.cos(), a.sin()])
    //     .collect::<Vec<_>>();
    let target_pose = target_curve
        .iter()
        .zip(&vectors)
        .map(|(c, v)| (na::Point2::from(*c) + na::Vector2::from(*v) * LENGTH).into())
        .collect::<Vec<_>>();

    const GEN: u64 = 100;
    let mut history = Vec::with_capacity(GEN as usize);
    let func = MFbSyn::from_uvec(&target_curve, vectors, Mode::Open);
    println!("harmonic: {}", func.harmonic());
    let pb = indicatif::ProgressBar::new(GEN);
    let s = Solver::build(De::default(), func)
        .seed(0)
        .pop_num(400)
        .task(|ctx| ctx.gen == GEN)
        .callback(|ctx| {
            history.push(ctx.best_f.fitness());
            pb.set_position(ctx.gen);
        })
        .solve()
        .unwrap();
    pb.finish();
    let fb = s.into_result();
    let (curve, pose) = fb.pose(60);
    let b = SVGBackend::new("history.svg", (800, 800));
    plot::fb::history(b, history).unwrap();
    let pose = curve
        .iter()
        .zip(pose)
        .map(|(c, v)| (na::Point2::from(*c) + na::Vector2::from(v) * LENGTH).into())
        .collect::<Vec<_>>();
    let mut fig = plot::fb::Figure::new(None)
        .add_line("Target", &target_curve, Style::Line, RED)
        .add_line("", &target_pose, Style::Circle, RED)
        .add_line("Optimized", &curve, Style::Line, BLUE)
        .add_line("", &pose, Style::Circle, BLUE)
        .legend(LegendPos::LR);
    for (p, v) in target_curve.iter().zip(&target_pose) {
        fig.push_line("", vec![*p, *v], Style::DashedLine, RED);
    }
    for (p, v) in curve.iter().zip(&pose) {
        fig.push_line("", vec![*p, *v], Style::DashedLine, BLUE);
    }
    let b = SVGBackend::new("syn.svg", (1600, 1600));
    fig.plot(b).unwrap();
}

#[allow(unused)]
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
#[allow(unused)]
const ANGLE: &[f64] = &[-0.9, 0., 0.7, 1.5, 2.8, -2.3, -2., -1.9, -2.1];
#[allow(unused)]
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
