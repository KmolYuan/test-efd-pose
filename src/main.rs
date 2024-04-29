use efd::PosedEfd2;
use four_bar::plot::*;

fn main() {
    let target_curve = PATH.iter().map(|&[x, y, ..]| [x, y]).collect::<Vec<_>>();
    let vectors = PATH.iter().map(|&[.., x, y]| [x, y]).collect::<Vec<_>>();
    // let tp_norm = efd::MotionSig::new(&target_curve, &vectors).as_t().to_vec();
    let efd = PosedEfd2::from_uvec(&target_curve, &vectors);
    dbg!(efd.harmonic());
    let (curve, pose) = efd.into_inner();
    let geo_inv = curve.as_geo().inverse();
    let target_curve = geo_inv.transform(&target_curve);
    let vectors = geo_inv.only_rot().transform(vectors);
    let target_pose = std::iter::zip(&target_curve, vectors)
        .map(|(p, v)| std::array::from_fn(|i| p[i] + v[i]))
        .collect::<Vec<_>>();
    let curve = curve.recon_norm(90);
    // let curve = curve.recon_norm_by(&tp_norm);
    let pose = pose.recon(90);
    // let pose = pose.recon_by(&tp_norm);
    let b = SVGBackend::new("test.svg", (1600, 1600));
    fb::Figure::new()
        .add_pose("Target", &target_curve, &target_pose, Style::Line, RED)
        .add_pose("EFD Recon.", &curve, &pose, Style::DashDottedLine, BLUE)
        .legend(LegendPos::LR)
        .plot(b)
        .unwrap();
}

#[rustfmt::skip]
const PATH: &[[f64; 4]] = &[
    [-43.9472668842624,-74.40208349773484,0.9798850574712644,0.19956270729907663],
    [-23.77112803303161,-85.90942515929798,0.8769795245630315,0.48052774477359705],
    [-13.896926364156707,-88.63667978358788,0.8105716467120921,0.5856394842788927],
    [-5.919197334564478,-89.79878406147661,0.7515094091740909,0.6597223718525914],
    [0.9535885731221967,-90.11053299889835,0.6972793377331222,0.7167995013743093],
    [7.020603652553283,-89.8653661015825,0.6470813677158922,0.7624209490530348],
    [12.429640438264647,-89.2265411606889,0.6006367299183588,0.799522056401811],
    [17.264199588962924,-88.30147017355004,0.5578834256894944,0.8299193233929153],
    [21.575360257613866,-87.16881818487083,0.5188641025608066,0.8548567383332536],
    [25.395974834120114,-85.89043011871586,0.48367510014077,0.8752476206787517],
    [28.747844793830613,-84.51719128786372,0.4524394437428518,0.8917951276754426],
    [31.64568780472227,-83.09207735705449,0.4252904780835427,0.9050569093993326],
    [34.099487753103304,-81.65175426737348,0.4023605847572664,0.9154812722463473],
    [36.11597450598068,-80.22736913354959,0.38377239759262755,0.9234277160915229],
    [37.69961805004131,-78.84486840267847,0.36963123393337,0.929178535536037],
    [38.8533508356318,-77.5250405978491,0.3600181138578077,0.9329453133460003],
    [39.57914399401952,-76.28341437412077,0.3549831189865864,0.9348727107122953],
    [39.87851261989128,-75.1301089676555,0.354539096708644,0.9350411910204912],
    [39.75299249913381,-74.06971476907017,0.3586559016038266,0.9334698411007964],
    [39.20460639596025,-73.10126692557024,0.36725549913351063,0.9301200988884156],
    [38.23631851402558,-72.21835925163978,0.3802083333333334,0.9249008721284075],
    [36.852460085363305,-71.40942681841733,0.3973313696124709,0.9176752054631737],
    [35.05909775233004,-70.65820312586142,0.4183881580068199,0.9082683244722676],
    [32.86431047034481,-69.94433324192198,0.44309112339554163,0.8964765788173592],
    [30.278340667875007,-69.24410036650953,0.47110609275326704,0.8820765552778003],
    [27.31359097234554,-68.53120284701505,0.502058843313818,0.8648334624945956],
    [23.98444733530944,-67.7775038779293,0.5355432340718339,0.8445078119472199],
    [20.30692014335888,-66.95366742477063,0.5711302987834197,0.8208594165943166],
    [16.29810340288948,-66.02958944589788,0.6083775527079678,0.7936477514370364],
    [11.975454419558403,-64.97452853434378,0.6468377002520477,0.7626276873630029],
    [7.355888074932903,-63.75682633810274,0.6860659051670401,0.7275393967114976],
    [2.4546546155148476,-62.34307142837012,0.7256247484940997,0.6880906367426275],
    [-2.7160835228568008,-60.696474723043266,0.7650858581443506,0.6439282799097448],
    [-8.149167030946437,-58.774035996993966,0.8040267671586105,0.5945931026277327],
    [-13.845260515551924,-56.52166132107374,0.8420204467791453,0.5394456109802438],
    [-19.817499115912167,-53.86539826906267,0.87861213632986,0.4775360864833982],
    [-26.100194858653744,-50.69434188479988,0.9132704366856759,0.4073537890776946],
    [-32.76786516417219,-46.82270089666399,0.9452757690279064,0.326272463577146],
    [-39.98662519028931,-41.88686646929471,0.973415304346174,0.2290472555274249],
    [-48.2144586028167,-34.94370241173306,0.9947894940209274,0.10195029468121762],
];
