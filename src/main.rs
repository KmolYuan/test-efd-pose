use four_bar::plot::*;

fn main() {
    let curve = (0..10).map(|_| rand::random()).collect::<Vec<_>>();
    let efd = efd::Efd2::from_curve(&curve, false);
    let sig = efd::PathSig::new(&curve, false);
    let curve_recon = efd.recon_norm(90);
    let curve = efd.as_geo().inverse().transform(curve);
    let t_norm = efd.recon_norm_by(sig.as_t());
    use efd::Distance as _;
    println!("t_norm is right: {}", t_norm[0].l2_err(&curve[0]) < 0.8);
    fb::Figure::new(None)
        .add_line("EFD Recon.", vec![curve_recon[0]], Style::Circle, GREEN)
        .add_line("", &curve_recon[..85], Style::Line, GREEN)
        .add_line("Target", vec![curve[0]], Style::Circle, BLUE)
        .add_line("", &curve[..4], Style::Line, BLUE)
        .add_line("Norm Time Param.", vec![t_norm[0]], Style::Circle, RED)
        .add_line("", &t_norm[..4], Style::DashDottedLine, RED)
        .legend(LegendPos::UL)
        .plot(SVGBackend::new("test.svg", (1600, 1600)))
        .unwrap();
}
