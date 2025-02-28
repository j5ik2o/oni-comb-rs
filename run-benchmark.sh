#!/usr/bin/env sh

set -e

export CLICOLOR_FORCE=1
export FORCE_COLOR=1

# ベンチマークの実行と結果の保存
echo "JSONパーサーのベンチマークを実行します..."

# ディレクトリを作成（存在しない場合）
mkdir -p benchmark_results

# 日時を含むファイル名を生成
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RESULT_FILE="benchmark_results/benchmark_${TIMESTAMP}.txt"

echo "ベンチマーク結果は ${RESULT_FILE} に保存されます"

# parserディレクトリに移動してベンチマークを実行
pushd parser
# ベンチマーク実行（詳細な出力をファイルに保存）
cargo bench | tee "../${RESULT_FILE}"

# フラムグラフの場所を表示
echo "\nフラムグラフは target/criterion/ ディレクトリに生成されています"
echo "各パーサーのパフォーマンスを視覚的に確認できます"
popd

echo "\nベンチマーク完了！"
