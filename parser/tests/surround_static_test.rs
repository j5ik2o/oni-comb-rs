use oni_comb_parser_rs::prelude::*;

#[test]
fn test_surround_static() {
  // 入力文字列 "(abc)" を解析するテスト
  let text = "(abc)";
  let input = text.chars().collect::<Vec<_>>();

  // 個別のパーサーをテスト
  let left_parser = elm_ref_static('(');
  let middle_parser = take_static(3);
  let right_parser = elm_ref_static(')');

  // 左側のパーサーをテスト
  let left_result = left_parser.parse(&input);
  println!("Left parser result: {:?}", left_result);
  assert!(left_result.is_success());

  // 中央のパーサーをテスト (オフセット1から開始)
  let middle_input = &input[1..];
  let middle_result = middle_parser.parse(middle_input);
  println!("Middle parser result: {:?}", middle_result);
  assert!(middle_result.is_success());

  // 右側のパーサーをテスト (オフセット4から開始)
  let right_input = &input[4..];
  let right_result = right_parser.parse(right_input);
  println!("Right parser result: {:?}", right_result);
  assert!(right_result.is_success());

  // surround_staticパーサーをテスト
  let parser = surround_static(elm_ref_static('('), take_static(3), elm_ref_static(')'));

  let result = parser.parse(&input);
  println!("Surround parser result: {:?}", result);

  // 結果が成功であることを確認
  assert!(result.is_success());

  // 結果が期待通りであることを確認
  assert_eq!(result.success().unwrap(), &input[1..4]);
}
