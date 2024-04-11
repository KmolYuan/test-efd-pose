use four_bar::plot::*;
use rand::{Rng as _, SeedableRng as _};

fn main() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(0);
    let curve = (0..4).map(|_| rng.gen()).collect::<Vec<_>>();
    let efd = efd::Efd2::from_curve(&curve, false);
    let sig = efd::PathSig::new(&curve, false);
    let curve_recon = efd.recon_norm(90);
    let curve = efd.as_geo().inverse().transform(curve);
    let t_norm = efd.recon_norm_by(sig.as_t());
    use efd::Distance as _;
    println!("t_norm is right: {}", t_norm[0].l2_err(&curve[0]) < 0.8);
    fb::Figure::new(None)
        .add_line_fp("EFD Recon.", &curve_recon[..89], Style::Line, GREEN)
        .add_line_fp("Target", &curve[..2], Style::DashedLine, BLUE)
        .add_line_fp("tp Norm.", &t_norm[..2], Style::DashDottedLine, RED)
        .legend(LegendPos::UL)
        .plot(SVGBackend::new("test.svg", (1600, 1600)))
        .unwrap();
}
