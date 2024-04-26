use four_bar::plot::*;

fn main() {
    let curve = vec![[0.0, 0.0], [0.5, 1.0], [0.8, 0.35], [1.0, 0.0]];
    let efd = efd::Efd2::from_curve(&curve, false);
    println!("Harmonic: {}", efd.harmonic());
    let sig = efd::PathSig::new(&curve, false);
    let curve_recon = efd.recon_norm(90);
    let curve = efd.as_geo().inverse().transform(curve);
    let t_norm = efd.recon_norm_by(sig.as_t());
    use efd::Distance as _;
    println!("t_norm is right: {}", t_norm[0].l2_err(&curve[0]) < 0.8);
    fb::Figure::new()
        .add_line("EFD Recon.", &curve_recon, Style::Line, GREEN)
        .add_line("1st Norm.", &curve_recon[..1], Style::Cross, BLACK)
        .add_line("Target", &curve, Style::Circle, RED)
        .add_line("tp Norm.", &t_norm, Style::Triangle, BLUE)
        .legend(LegendPos::UL)
        .plot(SVGBackend::new("test.svg", (1600, 1600)))
        .unwrap();
}
