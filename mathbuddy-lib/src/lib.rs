pub fn convert(latex: &str) -> String {
    latex2mathml::latex_to_mathml(latex, latex2mathml::DisplayStyle::Block).unwrap()
}
