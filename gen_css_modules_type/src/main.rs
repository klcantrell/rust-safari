use gen_css_modules_type as gen_type;

fn main() {
    let rules = gen_type::parse_rules(gen_type::TEST_CSS);

    println!("{:?}", rules);
}
