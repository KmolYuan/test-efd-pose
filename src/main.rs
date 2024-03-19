use four_bar::{
    mh::{De, Solver},
    plot::{self, *},
    syn,
};

fn main() {
    // let target_fb = ron::de::from_reader::<_, four_bar::MFourBar>(
    //     std::fs::File::open("hsieh-motion.ron").unwrap(),
    // )
    // .unwrap();
    // const LENGTH: f64 = 7.29;
    // let (target_curve, vectors) = target_fb.pose(180);
    // ===
    let target_curve = PATH.to_vec();
    const LENGTH: f64 = 6.36;
    let vectors = ANGLE.iter().map(|a| [a.cos(), a.sin()]).collect::<Vec<_>>();
    // === Hsieh
    // let fb =
    //     ron::de::from_reader::<_, MFourBar>(std::fs::File::open("hsieh.ron").unwrap()).unwrap();
    let target_pose = target_curve
        .iter()
        .zip(&vectors)
        .map(|(c, v)| std::array::from_fn(|i| c[i] + LENGTH * v[i]))
        .collect::<Vec<_>>();

    let t0 = std::time::Instant::now();
    const GEN: u64 = 200;
    let mut history = Vec::with_capacity(GEN as usize);
    let func = syn::MFbSyn::from_uvec(&target_curve, vectors, syn::Mode::Open);
    let pb = indicatif::ProgressBar::new(GEN);
    pb.set_style(indicatif::ProgressStyle::with_template("{wide_bar} {msg} {pos}/{len}").unwrap());
    let s = Solver::build(De::default(), func)
        .seed(0)
        .pop_num(2000)
        .pareto_limit(20)
        .task(|ctx| ctx.gen == GEN)
        .callback(|ctx| {
            pb.set_position(ctx.gen);
            let len = ctx.best.len();
            let eval = ctx.best.get_eval();
            // pb.set_message(format!("[eval: {eval:.04}]"));
            pb.set_message(format!("[pareto: {len}/eval: {eval:.04}]"));
            history.push(eval);
        })
        .solve();
    pb.finish();
    println!("Time spent: {:?}", t0.elapsed());
    let (err, fb) = s.into_err_result();
    use four_bar::mh::Fitness as _;
    println!("Error: {}", err.eval());
    let b = SVGBackend::new("history.svg", (800, 800));
    plot::fb::history(b, history).unwrap();
    let (curve, pose) = fb.pose(60);
    let pose = curve
        .iter()
        .zip(pose)
        .map(|(c, v)| std::array::from_fn(|i| c[i] + LENGTH * v[i]))
        .collect::<Vec<_>>();
    let mut fig = plot::fb::Figure::new(None)
        .legend(LegendPos::LR)
        .add_line("Target", &target_curve, Style::Line, RED)
        .add_line("", &target_pose, Style::Circle, RED);
    for (p, v) in target_curve.iter().zip(&target_pose) {
        fig.push_line("", vec![*p, *v], Style::DashDottedLine, RED);
    }
    fig.push_line("Optimized", &curve, Style::Line, BLUE);
    fig.push_line("", &pose, Style::Circle, BLUE);
    for (p, v) in curve.iter().zip(&pose) {
        fig.push_line("", vec![*p, *v], Style::DashDottedLine, BLUE);
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
