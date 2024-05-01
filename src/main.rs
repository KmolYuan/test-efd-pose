use four_bar::{
    mh,
    plot::{self, *},
    syn,
};

fn main() {
    // === Low point number
    let target_curve = PATH.to_vec();
    const LENGTH: f64 = 6.36;
    let vectors = ANGLE.iter().map(|a| [a.cos(), a.sin()]).collect::<Vec<_>>();
    // === High point number
    // let target_fb = ron::de::from_reader::<_, four_bar::MFourBar>(
    //     std::fs::File::open("hsieh-motion.ron").unwrap(),
    // )
    // .unwrap();
    // const LENGTH: f64 = 7.29;
    // let (target_curve, vectors) = target_fb.pose(60);
    // === Hsieh Result
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
    let mut pareto = Vec::with_capacity(GEN as usize);
    let func = syn::MFbSyn::from_uvec(&target_curve, vectors, syn::Mode::Open);
    println!("Harmonic: {}", func.harmonic());
    let pb = indicatif::ProgressBar::new(GEN);
    pb.set_style(indicatif::ProgressStyle::with_template("{wide_bar} {msg} {pos}/{len}").unwrap());
    let s = mh::Solver::build(mh::De::default(), func)
        .seed(0)
        .pop_num(2000)
        .task(|ctx| ctx.gen == GEN)
        .callback(|ctx| {
            let len = ctx.best.len();
            let eval = ctx.best.get_eval();
            pb.set_position(ctx.gen);
            pb.set_message(format!("[pareto: {len}/eval: {eval:.04}]"));
            // pb.set_message(format!("[eval: {eval:.04}]"));
            pareto.push(len);
            history.push(eval);
        })
        .solve();
    pb.finish();
    println!("Time spent: {:?}", t0.elapsed());
    let b = SVGBackend::new("pareto.svg", (800, 800));
    plot::fb::pareto(b, s.as_best_set().pareto_from_product()).unwrap();
    let (err, fb) = s.into_err_result();
    use four_bar::mh::Fitness as _;
    println!("Error: {}", err.eval());
    let b = SVGBackend::new("history.svg", (1200, 800));
    // plot::fb::history(b, history).unwrap();
    plot::fb::history_pareto(b, history, pareto).unwrap();
    let fb_str = ron::ser::to_string_pretty(&fb, Default::default()).unwrap();
    std::fs::write("syn.ron", fb_str).unwrap();
    let (curve, pose) = fb.pose(60);
    let pose = curve
        .iter()
        .zip(pose)
        .map(|(c, v)| std::array::from_fn(|i| c[i] + LENGTH * v[i]))
        .collect::<Vec<_>>();
    plot::fb::Figure::new()
        .legend(LegendPos::LR)
        .add_pose(
            "Target",
            (&target_curve, &target_pose, LENGTH),
            Style::Line,
            RED,
            false,
        )
        .add_pose(
            "Optimized",
            (&curve, &pose, LENGTH),
            Style::DashDottedLine,
            BLUE,
            true,
        )
        .plot(SVGBackend::new("syn.svg", (1600, 1600)))
        .unwrap();
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
