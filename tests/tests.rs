use pest_test_gen::pest_tests;

#[pest_tests(
    pest_wdl_1::PestParser,
    pest_wdl_1::Rule,
    "document",
    lazy_static = true
)]
#[cfg(test)]
mod test {}

// #[pest_tests(
//     pest_wdl_1::expr::ExprParser,
//     pest_wdl_1::expr::Rule,
//     "document",
//     lazy_static = true
// )]
// #[cfg(test)]
// mod test {}
