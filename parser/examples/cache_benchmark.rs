use oni_comb_parser_rs::prelude::*;
use std::time::{Duration, Instant};

fn space<'a>() -> Parser<'a, char, ()> {
    elm_of(" \t\r\n").of_many0().discard()
}

// 単純なJSONパーサーを定義
fn json_parser<'a>() -> Parser<'a, char, String> {
    // 文字列パーサー
    let string = surround(
        elm_ref('"'),
        none_ref_of("\\\"").of_many0().collect().map(String::from_iter),
        elm_ref('"')
    );

    // 数値パーサー
    let number = elm_digit_ref().of_many1().collect().map(String::from_iter);

    // 配列パーサー
    let array = surround(
        elm_ref('[') - space(),
        (string.clone() | number.clone()).of_many0_sep(elm_ref(',') - space()),
        space() * elm_ref(']')
    ).map(|items| format!("[{}]", items.join(", ")));

    // 空白パーサー
    let space = elm_of(" \t\r\n").of_many0().discard();

    // JSONパーサー
    space.clone() * (string | number | array) - space
}

// キャッシュなしでパースを実行
fn parse_without_cache(input: &[char], iterations: usize) -> Duration {
    let parser = json_parser();
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = parser.parse(input);
    }
    
    start.elapsed()
}

// キャッシュありでパースを実行
fn parse_with_cache(input: &[char], iterations: usize) -> Duration {
    let parser = json_parser().cache();
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = parser.parse(input);
    }
    
    start.elapsed()
}

fn main() {
    // テストデータ
    let test_cases = vec![
        "\"hello world\"",
        "42",
        "[1, 2, 3, 4, 5]",
        "[\"a\", \"b\", \"c\", 1, 2, 3]",
        "[1, [2, 3], 4, [5, 6, 7]]",
    ];
    
    let iterations = 1000;
    
    println!("キャッシュパーサーのパフォーマンス比較 ({} 回の繰り返し):", iterations);
    println!("{:<30} {:<15} {:<15} {:<10}", "テストケース", "キャッシュなし(ms)", "キャッシュあり(ms)", "改善率(%)");
    
    for test_case in test_cases {
        let input: Vec<char> = test_case.chars().collect();
        
        // 各テストケースを複数回実行して平均を取る
        let without_cache_time = parse_without_cache(&input, iterations);
        let with_cache_time = parse_with_cache(&input, iterations);
        
        let without_cache_ms = without_cache_time.as_millis();
        let with_cache_ms = with_cache_time.as_millis();
        
        // 改善率を計算
        let improvement = if without_cache_ms > 0 {
            100.0 * (without_cache_ms as f64 - with_cache_ms as f64) / without_cache_ms as f64
        } else {
            0.0
        };
        
        println!(
            "{:<30} {:<15} {:<15} {:<10.2}",
            test_case,
            without_cache_ms,
            with_cache_ms,
            improvement
        );
    }
}