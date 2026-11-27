use super::*;
use crate::DeclaredRule;
use crate::tester::Tester;

#[test]
fn escaped_and_plain_spellings_are_reads_of_the_same_local() {
    for source in [
        r"class A { int f() { int f\u006fo = 1; return foo; } }",
        r"class A { int f() { int foo = 1; return f\u006fo; } }",
        r"class A { int f() { int \u0066oo = 1; return f\uuuu006fo; } }",
        r"class A { int f() { int \uD835\uDC82 = 1; return 𝒂; } }",
        r"class A { int f() { int 𝒂 = 1; return \uD835\uDC82; } }",
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).passes(source);
    }
}

#[test]
fn identifier_ignorable_characters_do_not_change_a_locals_identity() {
    for source in [
        r"class A { int f() { int coun\u200Ct = 1; return count; } }",
        r"class A { int f() { int count = 1; return coun\u200Ct; } }",
        "class A { int f() { int coun\u{200c}t = 1; return count; } }",
        "class A { int f() { int count = 1; return coun\u{200c}t; } }",
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).passes(source);
    }
}

#[test]
fn fixes_identifier_ignorable_write_only_locals() {
    for (source, expected) in [
        (r"class A { void f() { int coun\u200Ct; count = 1; } }", "class A { void f() { } }"),
        (r"class A { void f() { int count; coun\u0000t = 1; } }", "class A { void f() { } }"),
        (
            r"class A { void f() { int coun\u0085t = get(); count = 1; } }",
            "class A { void f() { get(); } }",
        ),
        (
            "class A { void f() { int count = get(); coun\u{200c}t = 1; } }",
            "class A { void f() { get(); } }",
        ),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(source, expected);
    }
}

#[test]
fn reports_never_read_and_write_only_locals() {
    let report = Tester::new(&UNUSED_LOCAL_VARIABLE)
        .reports("class A { void f() { int stale = 1; int assigned; assigned = 2; } }");
    insta::assert_snapshot!(report, @r#"
        1:26: warning[unused-local-variable]: local variable `stale` is never read ("stale")
        1:41: warning[unused-local-variable]: local variable `assigned` is never read ("assigned")
        "#);
}

#[test]
fn accepts_reads_and_unnamed_variables() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
        "class A { void f(java.util.List<String> values) { \
             int used = 1; consume(used); for (String value : values) consume(value); \
             int _ = sideEffect(); } }",
    );
}

#[test]
fn a_local_in_a_switch_group_is_read_in_later_groups() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
        "class A { void f(int n) { switch (n) { case 1: int y; break; case 2: y = 2; use(y); } } \
             int g(int n) { return switch (n) { case 1: int y; yield 0; default: y = 2; yield y; }; } }",
    );
}

#[test]
fn ignore_pattern_exempts_names_that_contain_a_match() {
    let source = "class A { void f() { String unused = next(); String spare = next(); \
                  String _ = next(); } }";
    let report = Tester::new(&UNUSED_LOCAL_VARIABLE)
        .configured(serde_json::json!({ "ignore-pattern": "^unused" }))
        .reports(source);
    insta::assert_snapshot!(report, @r#"1:53: warning[unused-local-variable]: local variable `spare` is never read ("spare")"#);
}

#[test]
fn settings_parse_into_the_configured_rule() {
    let configured =
        |value| UnusedLocalVariable::from_configuration(value).map(|rule| rule.to_configuration());
    assert_eq!(configured(serde_json::Value::Null).unwrap(), None);
    assert_eq!(configured(serde_json::json!({})).unwrap(), None);
    assert_eq!(configured(serde_json::json!({ "ignore-pattern": null })).unwrap(), None);
    assert_eq!(
        configured(serde_json::json!({ "ignore-pattern": "^unused" })).unwrap(),
        Some(serde_json::json!({ "ignore-pattern": "^unused" }))
    );
    for invalid in [
        serde_json::json!({ "ignore_pattern": "^unused" }),
        serde_json::json!({ "ignore-pattern": "(" }),
        serde_json::json!({ "ignore-pattern": 1 }),
    ] {
        assert!(configured(invalid.clone()).is_err(), "{invalid} should be rejected");
    }
}

#[test]
fn reused_pattern_variable_names_are_read_through_their_own_binding() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
        "class A { void f(Object a, Object b) { \
             if (a instanceof String s) consume(s); \
             if (b instanceof String s) consume(s); } \
             void g(Object a, Object b) { \
             if (a instanceof String s && s.isEmpty()) return; \
             if (b instanceof String s) consume(s.length()); } }",
    );
    let report = Tester::new(&UNUSED_LOCAL_VARIABLE).reports(
        "class A { void f(Object a, Object b) { \
             if (a instanceof String s) consume(s); \
             if (b instanceof String s) return; } }",
    );
    insta::assert_snapshot!(
        report,
        @r#"1:103: warning[unused-local-variable]: local variable `s` is never read ("s")"#
    );
}

#[test]
fn pattern_variables_are_read_where_java_flow_puts_them_in_scope() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
        "class A { void f(Object o) { \
             for (; !(o instanceof String s); ) {} \
             System.out.println(s); } \
             void g(Object o) { \
             if (o instanceof String s) {} else { return; } \
             System.out.println(s); } \
             void h(Object o, I i) { \
             if (!(o instanceof String s)) { switch (i) { case E.X -> { return; } } } \
             System.out.println(s); } \
             void k(Object o) { \
             if (!(o instanceof String s)) { while (RUN) {} } \
             System.out.println(s); } \
             sealed interface I permits E {} enum E implements I { X } \
             interface J { boolean UP = true; default void m(Object o) { \
             if (!(o instanceof String s)) { while (UP) {} } \
             System.out.println(s); } } \
             static final boolean RUN = true; }",
    );
    // `while (c)` can complete normally, so the read is of the field.
    let report = Tester::new(&UNUSED_LOCAL_VARIABLE).reports(
        "class A { String s; void f(Object o, boolean c) { \
             if (!(o instanceof String s)) { while (c) {} } \
             System.out.println(s); } }",
    );
    insta::assert_snapshot!(
        report,
        @r#"1:77: warning[unused-local-variable]: local variable `s` is never read ("s")"#
    );
    // The `else` reads the field, so only the pattern binding goes.
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { String s; void f(Object o) { \
             if (o instanceof String s) {} else { use(s); } } }",
        "class A { String s; void f(Object o) { \
             if (o instanceof String) {} else { use(s); } } }",
    );
}

#[test]
fn honors_suppression_annotations() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
            "class A { void f() { @SuppressWarnings(\"robusta:unused-local-variable\") int stale = 1; } }",
        );
}

#[test]
fn safe_deletes_sole_uninitialized_declaration() {
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .fixes("class A { void f() { int stale; } }", "class A { void f() { } }");
}

#[test]
fn safe_removes_first_declarator_and_keeps_the_type() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { int stale, kept; consume(kept); } }",
        "class A { void f() { int kept; consume(kept); } }",
    );
}

#[test]
fn safe_removes_middle_declarator() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { int first, stale, last; consume(first); consume(last); } }",
        "class A { void f() { int first, last; consume(first); consume(last); } }",
    );
}

#[test]
fn safe_removes_last_declarator() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { int kept, stale; consume(kept); } }",
        "class A { void f() { int kept; consume(kept); } }",
    );
}

#[test]
fn safe_clears_declaration_shaped_for_initializer_but_keeps_semicolon() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { for (int stale; ready(); ) step(); } }",
        "class A { void f() { for (; ready(); ) step(); } }",
    );
}

#[test]
fn safe_keeps_the_spelling_of_an_escaped_for_separator() {
    for separator in [r"\u003b", r"\uuuu003b"] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
            &format!(
                "class A {{ void f() {{ for (int stale = 0{separator} ready(); ) step(); }} }}"
            ),
            &format!("class A {{ void f() {{ for ({separator} ready(); ) step(); }} }}"),
        );
    }
}

#[test]
fn safe_keeps_a_comment_before_a_for_separator() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { for (int stale = 0 /* why */; ready(); ) step(); } }",
        "class A { void f() { for ( /* why */; ready(); ) step(); } }",
    );
}

#[test]
fn safe_preserves_crlf_and_missing_final_newline() {
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .fixes("class A {\r\n  void f() { int stale; }\r\n}", "class A {\r\n  void f() { }\r\n}");
}

#[test]
fn deletes_the_lines_left_without_a_statement() {
    let tester = Tester::new(&UNUSED_LOCAL_VARIABLE);
    tester.fixes(
        "class A {\n  void f() {\n    int stale =\n        1;\n    run();\n  }\n}\n",
        "class A {\n  void f() {\n    run();\n  }\n}\n",
    );
    tester.fixes(
        "class A {\n  void f() {\n    int stale; stale = 1;\n    run(); int other;\n  }\n}\n",
        "class A {\n  void f() {\n    run();\n  }\n}\n",
    );
    tester.fixes(
        "class A {\n  void f() {\n    int stale; // why\n  }\n}\n",
        "class A {\n  void f() {\n    // why\n  }\n}\n",
    );
}

#[test]
fn fixes_write_only_local_and_its_assignments_together() {
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .fixes("class A { void f() { int assigned; assigned = 2; } }", "class A { void f() { } }");
}

#[test]
fn fixes_writes_through_parentheses_as_plain_writes() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { int assigned; (assigned) = 2; ((assigned)) = 3; } }",
        "class A { void f() { } }",
    );
    for (source, expected) in [
        ("int x = 0; (x) += 2;", ""),
        ("int x; consume((x) = 1);", " consume(1);"),
        ("int x; ((x)) = next();", " next();"),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
            &format!("class A {{ void f() {{ {source} }} }}"),
            &format!("class A {{ void f() {{{expected} }} }}"),
        );
    }
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
        "class A { int f; void f() { int[] a = new int[1]; (a[0]) = 1; A b = this; (b.f) = 1; } }",
    );
}

#[test]
fn unsafe_fix_unwraps_a_method_call_initializer() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        "class A { void f() { int ignored = sendEmail(); } }",
        "class A { void f() { sendEmail(); } }",
    );
}

#[test]
fn unsafe_fix_covers_every_statement_expression_category() {
    // assignment, class instance creation, post- and pre- increment.
    for (source, expected) in [
        (
            "class A { void f(int target) { int ignored = target = 3; } }",
            "class A { void f(int target) { target = 3; } }",
        ),
        (
            "class A { void f() { Object ignored = new Object(); } }",
            "class A { void f() { new Object(); } }",
        ),
        (
            "class A { void f(int[] counts) { int ignored = counts[0]++; } }",
            "class A { void f(int[] counts) { counts[0]++; } }",
        ),
        (
            "class A { void f(int[] counts) { int ignored = ++counts[0]; } }",
            "class A { void f(int[] counts) { ++counts[0]; } }",
        ),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(source, expected);
    }
}

#[test]
fn unsafe_fix_is_not_applied_with_safe_fixes_only() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fix_leaves_unchanged(
        FixKind::SAFE_FIX,
        "class A { void f() { int ignored = sendEmail(); } }",
    );
}

#[test]
fn unsafe_fix_deletes_a_nested_effect() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        "class A { void f(int[] data) { int stale = compute() + 1; } }",
        "class A { void f(int[] data) { } }",
    );
}

#[test]
fn unsafe_fix_deletes_an_array_initializer() {
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .fixes_unsafe("class A { void f() { int[] stale = {1, 2}; } }", "class A { void f() { } }");
}

#[test]
fn deleting_a_nested_effect_is_not_applied_with_safe_fixes_only() {
    let source = "class A { void f(int[] data) { int stale = compute() + 1; } }";
    Tester::new(&UNUSED_LOCAL_VARIABLE).fix_leaves_unchanged(FixKind::SAFE_FIX, source);
}

#[test]
fn safe_deletes_a_literal_and_a_type_patterns_variable() {
    // The fix for `unused` spans its assignment in the `if`, so the pattern's
    // fix overlaps it and applies on the next pass.
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_to_fixpoint(
        FixKind::SAFE_FIX,
        "void draw() {
    Shape shape = getShape(), unused = null;
    if (shape instanceof final Circle circle) unused = null;
}
",
        "void draw() {
    Shape shape = getShape();
    if (shape instanceof Circle) {}
}
",
    );
}

#[test]
fn unsafe_fix_keeps_effects_and_deletes_other_values() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        "void open() throws FileNotFoundException {
    int status = send();
    int ratio = total / count;
    int retries = 0;
    retries++;
    Runnable retry = () -> send();
    FileNotFoundException missing = failure(); // After the fix, failure() could throw IOException instead and break compilation.
}
",
        concat!(
            "void open() throws FileNotFoundException {\n",
            "    send();\n",
            "    failure(); // After the fix, failure() could throw IOException instead and break compilation.\n",
            "}\n",
        ),
    );
}

#[test]
fn unsafe_fix_deletes_effects_and_values_typed_by_the_variable() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        "void read() {
    int unused = next() + 1;
    Comparator<String> order = Comparator.comparing(s -> s.length());
    String header;
    if ((header = reader.readLine()) == null) return;
    int code;
    System.out.println(code = 'a'); // After the fix, this prints a instead of 97, since 'a' is a char.
    for (int i = 0, extra = next(); i < n; i++) {}
}
",
        concat!(
            "void read() {\n",
            "    if (reader.readLine() == null) return;\n",
            "    System.out.println('a'); // After the fix, this prints a instead of 97, since 'a' is a char.\n",
            "    for (int i = 0; i < n; i++) {}\n",
            "}\n",
        ),
    );
}

#[test]
fn excludes_bindings_that_cannot_lose_their_name() {
    for source in [
        "class A { void f(int[] values) { for (int stale : values) {} } }",
        "class A { void f() { try (AutoCloseable stale = open()) {} catch (Exception e) {} } }",
        "record R(int x) {} class A { void f(Object o) { if (o instanceof R(int stale)) {} } }",
        "class A { void f(Object o) { switch (o) { case String stale -> {} default -> {} } } }",
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).passes(source);
    }
}

#[test]
fn splits_a_shared_declaration_around_a_kept_initializer() {
    for (declaration, expected) in [
        ("int stale = compute(), kept = 1;", "compute(); int kept = 1;"),
        ("int kept = 1, stale = compute();", "int kept = 1; compute();"),
        ("int kept = 1, stale = compute(), last = 2;", "int kept = 1; compute(); int last = 2;"),
    ] {
        let tail =
            if declaration.contains("last") { "consume(kept, last);" } else { "consume(kept);" };
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
            &format!("class A {{ void f() {{ {declaration} {tail} }} }}"),
            &format!("class A {{ void f() {{ {expected} {tail} }} }}"),
        );
    }
}

#[test]
fn safe_deletes_an_initializer_string_spelling_a_comment_lead() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f() { String stale = \"http://example.com\"; } }",
        "class A { void f() { } }",
    );
}

#[test]
fn comments_do_not_block_the_documented_deletion() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        r"class A { void f() { int stale = \u002f\u002ac\u002a\u002f 1; } }",
        "class A { void f() { } }",
    );
}

#[test]
fn leaves_a_semicolon_where_a_deletion_follows_a_documentation_comment() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        concat!(
            "class A { void f(int i, boolean c) { /** @deprecated */ int a = 0; class L {}\n",
            "int b; /*** d */ /* c */ b = 0; /// d\nint d;\n",
            r"/** d */\u0020int e; switch (i) { case 0: /** d */ int g = 1; }",
            "\n/* c */ int h = 0; int k; if (c) /** d */ k = 0; for (/** d */ int m = 0; ;) break; } }",
        ),
        concat!(
            "class A { void f(int i, boolean c) { /** @deprecated */ ; class L {}\n",
            "/*** d */ /* c */ ; /// d\n;\n",
            r"/** d */\u0020; switch (i) { case 0: /** d */ ; }",
            "\n/* c */ if (c) /** d */ {} for (/** d */ ; ;) break; } }",
        ),
    );
}

#[test]
fn removes_comment_frames_without_suppressing_the_fix() {
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .fixes("class A { void f() { int /* keep */ stale; } }", "class A { void f() { } }");
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        "class A { void f() { int ignored = /* effect */ sendEmail(); } }",
        "class A { void f() { sendEmail(); } }",
    );
}

#[test]
fn the_declaration_names_every_kind_the_rule_emits() {
    let declared = UNUSED_LOCAL_VARIABLE.meta().fix;
    assert_eq!(declared, RuleFixMeta::Fixable(FixKind::ALL));
    for kind in [FixKind::SAFE_FIX, FixKind::UNSAFE_FIX] {
        assert!(declared.supports_fix(kind), "{} is emitted but not declared", kind.as_str());
    }
}

#[test]
fn safe_and_unsafe_arms_are_emitted_from_one_declaration() {
    const SOURCE: &str =
        "class A { void f(int[] data) { int a; int b = sendEmail(); int c = compute() + 1; } }";

    let tester = Tester::new(&UNUSED_LOCAL_VARIABLE);
    // Safe alone reaches the uninitialized declarator and nothing else.
    tester.fixes_only(
        FixKind::SAFE_FIX,
        SOURCE,
        "class A { void f(int[] data) { int b = sendEmail(); int c = compute() + 1; } }",
    );
    tester.fixes_only(FixKind::ALL, SOURCE, "class A { void f(int[] data) { sendEmail(); } }");
}

#[test]
fn overlapping_unused_declarators_apply_deterministically_and_converge() {
    // Two unused declarators in one declaration produce overlapping edits;
    // the fixer keeps the first, and a second pass removes the rest.
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_to_fixpoint(
        FixKind::SAFE_FIX,
        "class A { void f() { int stale, other; } }",
        "class A { void f() { } }",
    );
}

#[test]
fn discarded_updates_and_own_rhs_reads_are_not_usages() {
    for update in ["x++;", "++x;", "x += 2;", "x = x + 1;", "x = x == 0 ? 1 : 2;"] {
        let source = format!("class A {{ void f() {{ int x = 0; {update} }} }}");
        Tester::new(&UNUSED_LOCAL_VARIABLE).fix_leaves_unchanged(FixKind::SAFE_FIX, &source);
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(&source, "class A { void f() { } }");
    }
    for body in [
        "consume(x++);",
        "consume(++x);",
        "consume(x += 2);",
        "consume(x = x + 1);",
        "x = consume(x);",
        "x = x + next();",
        "x = x == 0 ? send() : 0;",
        "x = (field = x) + 1;",
        "x = switch (x) { case 0 -> 1; default -> 2; };",
        "for (; x-- > 0; ) {}",
        "int y = x++; consume(y);",
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE)
            .passes(&format!("class A {{ int field; void f() {{ int x = 0; {body} }} }}"));
    }
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        "class A { void f() { A b = null; b = b.next; } }",
        "class A { void f() { } }",
    );
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .passes("class A { void f() { boolean x = false; for (; x = !x; ) {} } }");
    Tester::new(&UNUSED_LOCAL_VARIABLE).passes(
        "class A { void f() { Runnable x = null; x = x == null ? () -> x.run() : () -> {}; } }",
    );
}

#[test]
fn removes_a_long_discarded_self_update() {
    let value = vec!["x"; 2_048].join(" + ");
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
        &format!("class A {{ void f() {{ int x = 0; x = {value}; }} }}"),
        "class A { void f() { } }",
    );
}

#[test]
fn aggregates_the_most_cautious_kind_across_all_writes() {
    let source = "class A { void f() { int x = 0; x = 2; x++; x = next() + 1; } }";
    let tester = Tester::new(&UNUSED_LOCAL_VARIABLE);
    tester.fix_leaves_unchanged(FixKind::SAFE_FIX, source);
    tester.fixes_unsafe(source, "class A { void f() { } }");
    tester.fixes_unsafe(
        "class A { void f() { int x = 0; x = next(); x += next(); } }",
        "class A { void f() { next(); next(); } }",
    );
}

#[test]
fn replaces_nested_assignments_with_their_values() {
    for (source, expected) in [
        ("int x; consume(x = 'a');", " consume('a');"),
        ("int x; if ((x = next()) == 0) return;", " if (next() == 0) return;"),
        ("int x; x = x = 1;", ""),
        ("int x = x = 1;", ""),
        ("int x; x = x = next();", " next();"),
        ("int x; int y = x = 1; consume(y);", " int y = 1; consume(y);"),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
            &format!("class A {{ void f() {{ {source} }} }}"),
            &format!("class A {{ void f() {{{expected} }} }}"),
        );
    }
}

#[test]
fn keeps_the_parentheses_a_replaced_value_needs() {
    for (source, expected) in [
        ("int x; use((x = a * b) + 2);", " use(a * b + 2);"),
        ("int x; use((x = a + b) * 2);", " use((a + b) * 2);"),
        ("int x; use(2 - (x = a - b));", " use(2 - (a - b));"),
        ("int x; use(c ? (x = a) : (x = b));", " use(c ? a : b);"),
        ("int x; use((x = c ? a : b) ? 1 : 2);", " use((c ? a : b) ? 1 : 2);"),
        ("int x; use((x = s).length());", " use(s.length());"),
        ("int x; use((x = new int[1]).length);", " use(new int[1].length);"),
        ("int x; use((x = new int[1])[0]);", " use((new int[1])[0]);"),
        ("int x; use((Object) (x = -a));", " use((Object) (-a));"),
        ("int x; use(-(x = -a));", " use(- -a);"),
        ("int x; use(a+(x = ++b));", " use(a+ ++b);"),
        ("int x; use(-(x = \\u002da));", " use(- \\u002da);"),
        ("int x; use(\\u002d(x = -a));", " use(\\u002d -a);"),
        ("int x; return(x = a);", " return a;"),
        ("int x; use((/* kept */ x = a) + 1);", " use((/* kept */ a) + 1);"),
        ("int x; use((x = a /* kept */) + 1);", " use((a /* kept */) + 1);"),
        ("int x; use(x = (x = a) + 1);", " use((a) + 1);"),
        ("int x; use((x = (x = a)) + 1);", " use(((a)) + 1);"),
        ("Integer x; use((x = 1).hashCode());", " use((1).hashCode());"),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
            &format!("class A {{ void f() {{ {source} }} }}"),
            &format!("class A {{ void f() {{{expected} }} }}"),
        );
    }
}

#[test]
fn deletes_whole_statement_bodies_as_empty_blocks() {
    for (statement, expected) in [
        ("if (ready()) x = 1; else x = 2;", "if (ready()) {} else {}"),
        ("while (ready()) x = 1;", "while (ready()) {}"),
        ("do x = 1; while (ready());", "do {} while (ready());"),
        ("label: x = 1;", "label: {}"),
        (
            "switch (n) { case 0: x = 1; break; default: x = 2; }",
            "switch (n) { case 0: break; default: }",
        ),
        (
            "switch (n) { case 0 -> x = 1; default -> x = 2; }",
            "switch (n) { case 0 -> {} default -> {} }",
        ),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
            &format!("class A {{ void f(int n) {{ int x; {statement} }} }}"),
            &format!("class A {{ void f(int n) {{ {expected} }} }}"),
        );
    }
}

#[test]
fn preserves_for_lists_and_shared_initializer_policy() {
    for (body, expected) in [
        ("for (int x = next(); ready(); ) {}", "for (next(); ready(); ) {}"),
        (r"for (int x = next()\u003b ready(); ) {}", r"for (next()\u003b ready(); ) {}"),
        (
            r"for (int i = 0, x = 0\u003b i < 3; i++, x++) {}",
            r"for (int i = 0\u003b i < 3; i++) {}",
        ),
        ("for (int i = 0, x = 0; i < 3; i++, x++) {}", "for (int i = 0; i < 3; i++) {}"),
        ("int x = 0; for (x = 1, x = 2; ready(); x++, x++) {}", "for (; ready(); ) {}"),
        (
            "int x = 0; for (x = 1, next(); ready(); next(), x++, next()) {}",
            "for (next(); ready(); next(), next()) {}",
        ),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
            &format!("class A {{ void f() {{ {body} }} }}"),
            &format!("class A {{ void f() {{ {expected} }} }}"),
        );
    }
    let source = "class A { void f() { for (int i = 0, x = next(); i < 3; i++) {} } }";
    Tester::new(&UNUSED_LOCAL_VARIABLE).fix_leaves_unchanged(FixKind::SAFE_FIX, source);
    Tester::new(&UNUSED_LOCAL_VARIABLE)
        .fixes_unsafe(source, "class A { void f() { for (int i = 0; i < 3; i++) {} } }");
}

#[test]
fn removes_consecutive_for_expressions_as_one_group() {
    for (expressions, expected) in [
        ("x = 1, x = 2, next()", "next()"),
        ("next(), x = 1, x = 2", "next()"),
        ("next(), x = 1, x = 2, next()", "next(), next()"),
        ("x = 1, x = 2", ""),
        ("x = 1, next(), x = 2", "next()"),
        ("x += 1, x += 2, next()", "next()"),
        ("x++, ++x, next()", "next()"),
        ("x += next(), x = 1, x = 2, next()", "next(), next()"),
        ("x = 1, x = 2, x += next()", "next()"),
        ("x = 1, x = 2, x = next(), x = 3, x = 4", "next()"),
    ] {
        Tester::new(&UNUSED_LOCAL_VARIABLE).fixes_unsafe(
            &format!("class A {{ void f() {{ int x = 0; for ({expressions}; ready(); {expressions}) {{}} }} }}"),
            &format!("class A {{ void f() {{ for ({expected}; ready(); {expected}) {{}} }} }}"),
        );
    }
}

#[test]
fn classifies_evaluated_effects_and_target_typing() {
    let tester = Tester::new(&UNUSED_LOCAL_VARIABLE);
    for value in [
        "total / count",
        "data[0] + 1",
        "() -> send()",
        "String::length",
        "new int[] {1}",
        "{1, 2}",
    ] {
        let source = format!("class A {{ void f() {{ Object x = {value}; }} }}");
        tester.fix_leaves_unchanged(FixKind::SAFE_FIX, &source);
        tester.fixes_unsafe(&source, "class A { void f() { } }");
    }
    for value in [
        "next() + 1",
        "(next())",
        "switch (n) { default -> 1; }",
        "compare(s -> s.length())",
        "compare(String::length)",
        "new Box<>() {}",
        "new int[] {next()}",
        "factory()::run",
    ] {
        let source = format!("class A {{ void f() {{ Object x = {value}; }} }}");
        tester.fix_leaves_unchanged(FixKind::SAFE_FIX, &source);
        tester.fixes_unsafe(&source, "class A { void f() { } }");
    }
    for value in ["0", "1.0", "'x'", "\"hi\"", "true", "null"] {
        tester.fixes(
            &format!("class A {{ void f() {{ Object x = {value}; }} }}"),
            "class A { void f() { } }",
        );
    }
}

#[test]
fn removes_the_instanceof_pattern_binding_and_modifiers() {
    Tester::new(&UNUSED_LOCAL_VARIABLE).fixes(
        "class A { void f(Object o) { if (o instanceof final String s) {} } }",
        "class A { void f(Object o) { if (o instanceof String) {} } }",
    );
}
