//! Operator metadata complexity counts toward expression complexity.

use dynamic_expressions::expression_algebra::__apply_postfix;
use dynamic_expressions::operator_enum::builtin::Add;
use dynamic_expressions::{Metadata, OperatorSet, PNode, PostfixExpr};
use symbolic_regression::{Options, compute_complexity};

dynamic_expressions::op!(Heavy for f64 {
    complexity: 3,
    eval: |[x]| { x },
    partial: |[_x], _idx| { 1.0 },
});

dynamic_expressions::opset! {
    pub HeavyOps for f64 { Heavy, Add }
}

#[test]
fn complexity_uses_operator_metadata() {
    let operator_set = HeavyOps::new();
    let options = Options::<f64, HeavyOps, 2> {
        operators: operator_set.from_names(["heavy", "add"]).unwrap(),
        ..Default::default()
    };
    let heavy = options.operators.set.lookup("heavy").unwrap();
    let leaf = PostfixExpr::<f64, HeavyOps, 2>::new(vec![PNode::Var { feature: 0 }], vec![], Metadata::default());
    let tree = __apply_postfix::<f64, HeavyOps, 2, 1>(heavy.id, [leaf]);
    assert_eq!(compute_complexity(&tree.nodes, &options), 3 + 1);
}
