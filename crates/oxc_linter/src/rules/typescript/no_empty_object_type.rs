use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{
    context::LintContext,
    fixer::{RuleFix, RuleFixer},
    rule::Rule,
    AstNode,
};

fn no_empty_object_type_diagnostic(span: Span) -> OxcDiagnostic {
    // See <https://oxc.rs/docs/contribute/linter/adding-rules.html#diagnostics> for details
    OxcDiagnostic::warn("Disallow accidentally using the \"empty object\" type.")
        .with_help("To avoid confusion around the {} type allowing any non-nullish value, this rule bans usage of the {} type.")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoEmptyObjectType;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```ts
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```ts
    /// FIXME: Tests will fail if examples are missing or syntactically incorrect.
    /// ```
    NoEmptyObjectType,
    nursery, // TODO: change category to `correctness`, `suspicious`, `pedantic`, `perf`, `restriction`, or `style`
             // See <https://oxc.rs/docs/contribute/linter.html#rule-category> for details

    pending  // TODO: describe fix capabilities. Remove if no fix can be done,
             // keep at 'pending' if you think one could be added but don't know how.
             // Options are 'fix', 'fix_dangerous', 'suggestion', and 'conditional_fix_suggestion'
);

impl Rule for NoEmptyObjectType {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;
    use std::path::PathBuf;

    let pass = vec![
        (
            "
			interface Base {
			  name: string;
			}
			    ",
            None,
            None,
            None,
        ),
        (
            "
			interface Base {
			  name: string;
			}
			
			interface Derived {
			  age: number;
			}
			
			// valid because extending multiple interfaces can be used instead of a union type
			interface Both extends Base, Derived {}
			    ",
            None,
            None,
            None,
        ),
        (
            "interface Base {}",
            Some(serde_json::json!([{ "allowInterfaces": "always" }])),
            None,
            None,
        ),
        (
            "
			interface Base {
			  name: string;
			}
			
			interface Derived extends Base {}
			      ",
            Some(serde_json::json!([{ "allowInterfaces": "with-single-extends" }])),
            None,
            None,
        ),
        (
            "
			interface Base {
			  props: string;
			}
			
			interface Derived extends Base {}
			
			class Derived {}
			      ",
            Some(serde_json::json!([{ "allowInterfaces": "with-single-extends" }])),
            None,
            None,
        ),
        ("let value: object;", None, None, None),
        ("let value: Object;", None, None, None),
        ("let value: { inner: true };", None, None, None),
        ("type MyNonNullable<T> = T & {};", None, None, None),
        (
            "type Base = {};",
            Some(serde_json::json!([{ "allowObjectTypes": "always" }])),
            None,
            None,
        ),
        ("type Base = {};", Some(serde_json::json!([{ "allowWithName": "Base" }])), None, None),
        (
            "type BaseProps = {};",
            Some(serde_json::json!([{ "allowWithName": "Props$" }])),
            None,
            None,
        ),
        ("interface Base {}", Some(serde_json::json!([{ "allowWithName": "Base" }])), None, None),
        (
            "interface BaseProps {}",
            Some(serde_json::json!([{ "allowWithName": "Props$" }])),
            None,
            None,
        ),
    ];

    let fail = vec![
        ("interface Base {}", None, None, None),
        (
            "interface Base {}",
            Some(serde_json::json!([{ "allowInterfaces": "never" }])),
            None,
            None,
        ),
        (
            "
			interface Base {
			  props: string;
			}
			
			interface Derived extends Base {}
			
			class Other {}
			      ",
            None,
            None,
            None,
        ),
        (
            "
			interface Base {
			  props: string;
			}
			
			interface Derived extends Base {}
			
			class Derived {}
			      ",
            None,
            None,
            None,
        ),
        (
            "
			interface Base {
			  props: string;
			}
			
			interface Derived extends Base {}
			
			const derived = class Derived {};
			      ",
            None,
            None,
            None,
        ),
        (
            "
			interface Base {
			  name: string;
			}
			
			interface Derived extends Base {}
			      ",
            None,
            None,
            None,
        ),
        ("interface Base extends Array<number> {}", None, None, None),
        ("interface Base extends Array<number | {}> {}", None, None, None),
        (
            "
			interface Derived {
			  property: string;
			}
			interface Base extends Array<Derived> {}
			      ",
            None,
            None,
            None,
        ),
        (
            "
			type R = Record<string, unknown>;
			interface Base extends R {}
			      ",
            None,
            None,
            None,
        ),
        ("interface Base<T> extends Derived<T> {}", None, None, None),
        (
            "
			declare namespace BaseAndDerived {
			  type Base = typeof base;
			  export interface Derived extends Base {}
			}
			      ",
            None,
            None,
            Some(PathBuf::from("'test.d.ts'")),
        ),
        ("type Base = {};", None, None, None),
        ("type Base = {};", Some(serde_json::json!([{ "allowObjectTypes": "never" }])), None, None),
        ("let value: {};", None, None, None),
        ("let value: {};", Some(serde_json::json!([{ "allowObjectTypes": "never" }])), None, None),
        (
            "
			let value: {
			  /* ... */
			};
			      ",
            None,
            None,
            None,
        ),
        ("type MyUnion<T> = T | {};", None, None, None),
        (
            "type Base = {} | null;",
            Some(serde_json::json!([{ "allowWithName": "Base" }])),
            None,
            None,
        ),
        ("type Base = {};", Some(serde_json::json!([{ "allowWithName": "Mismatch" }])), None, None),
        (
            "interface Base {}",
            Some(serde_json::json!([{ "allowWithName": ".*Props$" }])),
            None,
            None,
        ),
    ];

    Tester::new(NoEmptyObjectType::NAME, pass, fail).test_and_snapshot();
}
