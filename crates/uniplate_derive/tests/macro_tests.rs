use uniplate::uniplate::Uniplate;
use uniplate_derive::Uniplate;

#[derive(Clone, Debug, PartialEq, Eq, Uniplate)]
enum TestEnum {
    A(i32),
    B(Box<TestEnum>),
    C(Vec<TestEnum>),
    D(bool, Box<TestEnum>),
    E(Box<TestEnum>, Box<TestEnum>),
    F((Box<TestEnum>, Box<TestEnum>)),
    G((Box<TestEnum>, (Box<TestEnum>, i32))),
    H(Vec<Vec<TestEnum>>),
    I(Vec<TestEnum>, i32, Vec<TestEnum>),
}

#[test]
fn increase_number_of_children() {
    let c = TestEnum::C(vec![TestEnum::A(42)]);
    let context = c.uniplate().1;
    assert_eq!(
        context(vec![TestEnum::A(42), TestEnum::A(42)]),
        Err(uniplate::uniplate::UniplateError::WrongNumberOfChildren(
            1, 2
        ))
    );
}

#[test]
fn decrease_number_of_children() {
    let c = TestEnum::C(vec![TestEnum::A(42)]);
    let context = c.uniplate().1;
    assert_eq!(
        context(vec![]),
        Err(uniplate::uniplate::UniplateError::WrongNumberOfChildren(
            1, 0
        ))
    );
}

#[test]
fn derive_context_empty() {
    let a = TestEnum::A(42);
    let context = a.uniplate().1;
    assert_eq!(context(vec![]).unwrap(), a)
}

#[test]
fn derive_context_box() {
    let a = TestEnum::A(42);
    let b = TestEnum::B(Box::new(a.clone()));
    let context = b.uniplate().1;
    assert_eq!(context(vec![a.clone()]).unwrap(), b);
}

#[test]
fn derive_context_vec() {
    let a = TestEnum::A(1);
    let b = TestEnum::B(Box::new(TestEnum::A(2)));
    let c = TestEnum::C(vec![a.clone(), b.clone()]);
    let context = c.uniplate().1;
    assert_eq!(context(vec![a.clone(), b.clone()]).unwrap(), c);
}

#[test]
fn derive_context_two() {
    let d = TestEnum::D(true, Box::new(TestEnum::A(42)));
    let context = d.uniplate().1;
    assert_eq!(context(vec![TestEnum::A(42)]).unwrap(), d);
}

#[test]
fn derive_context_tuple() {
    let e = TestEnum::F((Box::new(TestEnum::A(1)), Box::new(TestEnum::A(2))));
    let context = e.uniplate().1;
    assert_eq!(context(vec![TestEnum::A(1), TestEnum::A(2)]).unwrap(), e);
}

#[test]
fn derive_context_different_variants() {
    let f = TestEnum::E(
        Box::new(TestEnum::A(1)),
        Box::new(TestEnum::B(Box::new(TestEnum::A(2)))),
    );
    let context = f.uniplate().1;
    assert_eq!(
        context(vec![TestEnum::A(1), TestEnum::B(Box::new(TestEnum::A(2)))]).unwrap(),
        f
    );
}

#[test]
fn derive_context_nested_tuples() {
    let g = TestEnum::G((Box::new(TestEnum::A(1)), (Box::new(TestEnum::A(2)), 42)));
    let context = g.uniplate().1;
    assert_eq!(context(vec![TestEnum::A(1), TestEnum::A(2)]).unwrap(), g);
}

#[test]
fn derive_context_nested_vectors() {
    let h = TestEnum::H(vec![
        vec![TestEnum::A(1), TestEnum::A(2)],
        vec![TestEnum::A(3), TestEnum::A(4)],
    ]);
    let context = h.uniplate().1;
    assert_eq!(
        context(vec![
            TestEnum::A(1),
            TestEnum::A(2),
            TestEnum::A(3),
            TestEnum::A(4)
        ])
        .unwrap(),
        h
    );
}

#[test]
fn derive_context_multiple_vecs() {
    let i = TestEnum::I(
        vec![TestEnum::A(1), TestEnum::A(2)],
        42,
        vec![TestEnum::A(3), TestEnum::A(4)],
    );
    let context = i.uniplate().1;
    assert_eq!(
        context(vec![
            TestEnum::A(1),
            TestEnum::A(2),
            TestEnum::A(3),
            TestEnum::A(4)
        ])
        .unwrap(),
        i
    );
}

#[test]
fn box_change_child() {
    let b = TestEnum::B(Box::new(TestEnum::A(1)));
    let context = b.uniplate().1;
    assert_eq!(
        context(vec![TestEnum::C(vec![TestEnum::A(41), TestEnum::A(42)])]).unwrap(),
        TestEnum::B(Box::new(TestEnum::C(vec![
            TestEnum::A(41),
            TestEnum::A(42)
        ])))
    );
}

#[test]
fn derive_children_empty() {
    let a = TestEnum::A(42);
    let children = a.uniplate().0;
    assert_eq!(children, vec![]);
}

#[test]
fn derive_children_box() {
    let b = TestEnum::B(Box::new(TestEnum::A(42)));
    let children = b.uniplate().0;
    assert_eq!(children, vec![TestEnum::A(42)]);
}

#[test]
fn derive_children_vec() {
    let c = TestEnum::C(vec![TestEnum::A(1), TestEnum::B(Box::new(TestEnum::A(2)))]);
    let children = c.uniplate().0;
    assert_eq!(
        children,
        vec![TestEnum::A(1), TestEnum::B(Box::new(TestEnum::A(2))),]
    );
}

#[test]
fn derive_children_two() {
    let d = TestEnum::D(true, Box::new(TestEnum::A(42)));
    let children = d.uniplate().0;
    assert_eq!(children, vec![TestEnum::A(42)]);
}

#[test]
fn derive_children_tuple() {
    let e = TestEnum::F((Box::new(TestEnum::A(1)), Box::new(TestEnum::A(2))));
    let children = e.uniplate().0;
    assert_eq!(children, vec![TestEnum::A(1), TestEnum::A(2),]);
}

#[test]
fn derive_children_different_variants() {
    let f = TestEnum::E(
        Box::new(TestEnum::A(1)),
        Box::new(TestEnum::B(Box::new(TestEnum::A(2)))),
    );
    let children = f.uniplate().0;
    assert_eq!(
        children,
        vec![TestEnum::A(1), TestEnum::B(Box::new(TestEnum::A(2)))]
    );
}

#[test]
fn derive_children_nested_tuples() {
    let g = TestEnum::G((Box::new(TestEnum::A(1)), (Box::new(TestEnum::A(2)), 42)));
    let children = g.uniplate().0;
    assert_eq!(children, vec![TestEnum::A(1), TestEnum::A(2)])
}

#[test]
fn derive_children_nested_vectors() {
    let h = TestEnum::H(vec![
        vec![TestEnum::A(1), TestEnum::A(2)],
        vec![TestEnum::A(3), TestEnum::A(4)],
    ]);
    let children = h.uniplate().0;
    assert_eq!(
        children,
        vec![
            TestEnum::A(1),
            TestEnum::A(2),
            TestEnum::A(3),
            TestEnum::A(4)
        ]
    )
}

#[test]
fn derive_children_multiple_vecs() {
    let i = TestEnum::I(
        vec![TestEnum::A(1), TestEnum::A(2)],
        42,
        vec![TestEnum::A(3), TestEnum::A(4)],
    );
    let children = i.uniplate().0;
    assert_eq!(
        children,
        vec![
            TestEnum::A(1),
            TestEnum::A(2),
            TestEnum::A(3),
            TestEnum::A(4)
        ]
    );
}
