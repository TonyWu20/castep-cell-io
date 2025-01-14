mod param;
mod parser;

#[cfg(test)]
mod tests {
    use super::param::CastepParam;

    #[test]
    fn it_works() {
        let geom_opt = CastepParam::geom_opt_param_template(380.0, 3.0).unwrap();
        println!("{geom_opt}");
    }
}
