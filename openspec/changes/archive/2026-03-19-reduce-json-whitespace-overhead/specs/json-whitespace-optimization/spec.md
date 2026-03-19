## ADDED Requirements

### Requirement: ベンチ用 JSON パーサーは非本質的な空白の受理を維持する

oni-comb のベンチ用 JSON パーサーは、最適化前と同じ JSON 文法境界で ASCII の非本質的な空白を受理し続けなければならない。これにはトップレベル入力、配列・オブジェクトの開始、カンマ、メンバーのコロン、閉じ区切り記号が含まれる。

#### Scenario: JSON フルベンチが空白入り構造を受理する

- **WHEN** JSON フルベンチ用パーサーが ` { "a" : [ 1 , 2 ] , "b" : { "c" : true } } ` のような入力をパースする
- **THEN** パースが成功する
- **AND** 余分な空白を除いた compact な JSON と同値な構造が得られる

#### Scenario: allocation-count 用パーサーが同じ空白配置を受理する

- **WHEN** allocation-count 用 JSON パーサーが、JSON フルベンチ用パーサーと同じ空白入り入力をパースする
- **THEN** パースが成功する
- **AND** 得られる構造は JSON フルベンチ用パーサーと同じ JSON ノード種別を使う

### Requirement: ベンチ用 JSON 実装は一貫した空白処理方針を使う

oni-comb の JSON フルベンチ、allocation-count 用パーサー、JSON subset ベンチは、受理する JSON 入力を変えずに重複した空白走査を減らせるよう、一貫した文法境界ベースの空白処理方針で任意空白を消費しなければならない。

#### Scenario: subset ベンチが区切り記号まわりの空白を維持する

- **WHEN** JSON subset ベンチが `[ 1 , "two" , true , null ]` をパースする
- **THEN** パースが成功する
- **AND** この受理を維持するために、すべての区切り記号トークンへ個別の前後空白ラッパーを必須としない

#### Scenario: object member がコロンとカンマまわりの空白を維持する

- **WHEN** ベンチ用 JSON パーサーが `{ "name" : "oni-comb" , "version" : 2 }` をパースする
- **THEN** パースが成功する
- **AND** member のパースは共有された空白処理方針の下で `:` と `,` の両方の前後にある任意空白を受理する
