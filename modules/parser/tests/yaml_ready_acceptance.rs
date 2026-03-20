//! YAML-ready acceptance criteria for downstream grammars.
//!
//! This file fixes the phase-1 contract before the parser core redesign starts.
//! The acceptance grammars in this file must stay declarative:
//! - do not call `parse_next` directly from downstream grammar code
//! - do not call `checkpoint` / `reset` directly from downstream grammar code
//! - do not discard parser results and branch on input state by hand
//! - do not fall back to imperative escape hatches such as `fn_parser`
//!
//! The ignored tests below are the executable contract for tasks 2.*-4.*.
//! They stay in `modules/parser/tests` so the readiness criteria live next to
//! other downstream grammar examples.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExpectedOutcome {
  Accept,
  Reject {
    line: usize,
    column: usize,
    reason: &'static str,
  },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LitmusGrammarCase {
  id: &'static str,
  input: &'static str,
  outcome: ExpectedOutcome,
}

const LITMUS_GRAMMAR_CASES: [LitmusGrammarCase; 10] = [
  LitmusGrammarCase {
    id: "block list",
    input: "- milk\n- eggs\n- bread\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "indent nesting",
    input: "parent:\n  child:\n    grandchild: value\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "flow/block switching",
    input: "items: [one, two]\nmapping:\n  nested: value\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "multiline block",
    input: "note: |\n  line one\n  line two\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "block scalar header",
    input: "literal: |-\n  line one\n  line two\nfolded: >2\n    line three\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "document boundary",
    input: "---\nkey: value\n...\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "simple-key gating",
    input: "plain: value\n? explicit key\n: value\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "simple-key backtrack",
    input: "plain: value\n? explicit key\n: value\nflow: [one, two]\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "flow plain scalar boundary",
    input: "{key: plain, next: [one, two]}\n",
    outcome: ExpectedOutcome::Accept,
  },
  LitmusGrammarCase {
    id: "indent error",
    input: "root:\n  child: ok\n next: wrong\n",
    outcome: ExpectedOutcome::Reject {
      line: 3,
      column: 2,
      reason: "indentation must match an active block context",
    },
  },
];

fn litmus_case(id: &'static str) -> LitmusGrammarCase {
  *LITMUS_GRAMMAR_CASES
    .iter()
    .find(|case| case.id == id)
    .unwrap_or_else(|| panic!("missing YAML-ready litmus grammar case: {id}"))
}

fn assert_pending_contract(case: LitmusGrammarCase) {
  match case.outcome {
    ExpectedOutcome::Accept => panic!("pending YAML-ready acceptance grammar: {}", case.id),
    ExpectedOutcome::Reject { reason, .. } => {
      panic!("pending YAML-ready rejection grammar: {} ({reason})", case.id)
    }
  }
}

#[test]
fn yaml_ready_litmus_grammar_catalog_is_fixed() {
  let actual_ids: [&str; 10] = core::array::from_fn(|index| LITMUS_GRAMMAR_CASES[index].id);
  assert_eq!(
    actual_ids,
    [
      "block list",
      "indent nesting",
      "flow/block switching",
      "multiline block",
      "block scalar header",
      "document boundary",
      "simple-key gating",
      "simple-key backtrack",
      "flow plain scalar boundary",
      "indent error",
    ]
  );
}

#[test]
fn yaml_ready_litmus_grammar_examples_are_fixed() {
  let accept_count = LITMUS_GRAMMAR_CASES
    .iter()
    .filter(|case| matches!(case.outcome, ExpectedOutcome::Accept))
    .count();
  let reject_count = LITMUS_GRAMMAR_CASES.len() - accept_count;

  assert_eq!(accept_count, 9);
  assert_eq!(reject_count, 1);

  let indent_error = litmus_case("indent error");
  assert_eq!(
    indent_error.outcome,
    ExpectedOutcome::Reject {
      line: 3,
      column: 2,
      reason: "indentation must match an active block context",
    }
  );
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn block_list_acceptance_contract() {
  assert_pending_contract(litmus_case("block list"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn indent_nesting_acceptance_contract() {
  assert_pending_contract(litmus_case("indent nesting"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn flow_block_switching_acceptance_contract() {
  assert_pending_contract(litmus_case("flow/block switching"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn multiline_block_acceptance_contract() {
  assert_pending_contract(litmus_case("multiline block"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn block_scalar_header_acceptance_contract() {
  assert_pending_contract(litmus_case("block scalar header"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn document_boundary_acceptance_contract() {
  assert_pending_contract(litmus_case("document boundary"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn simple_key_gating_acceptance_contract() {
  assert_pending_contract(litmus_case("simple-key gating"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn simple_key_backtrack_acceptance_contract() {
  assert_pending_contract(litmus_case("simple-key backtrack"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn flow_plain_scalar_boundary_acceptance_contract() {
  assert_pending_contract(litmus_case("flow plain scalar boundary"));
}

#[test]
#[ignore = "Pending yaml-ready parser redesign (tasks 2.*-4.*)"]
fn indent_error_acceptance_contract() {
  assert_pending_contract(litmus_case("indent error"));
}
