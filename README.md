# pest-wdl-1

[Pest](https://pest.rs/) parser grammar for Workflow Description Language (WDL).

## Usage

```wdl
use pest::iterators::Pair;
use pest_wdl_1 as wdl;

fn main() {
    let text = r#"
    version 1.0

    workflow foo {
    }
    "#;
    
    let root: Pair<'_, wdl::Rule> = 
        wdl::parse_document(text).expect("Error parsing WDL document");
    ...
}
