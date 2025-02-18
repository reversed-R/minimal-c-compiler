# memo

## asm

- レジスタ

  - rax
    - 戻り値を記録
  - 引数
    1. rdi
    1. rsi

- 命令
  - _hoge_ _dst_, _src_
    - Intel記法ではこの形式
      - メモリ参照を[]でくくるなど
    - ex:
      - add _dst_, _src_
      - mov _dst_, _src_
        - moveだが、移動ではなくコピー
  - call _dst_
    1. push スタック, 次の命令のアドレス(リターンアドレス)
    1. jump _dst_
  - ret
    1. スタックからリターンアドレスをpop
    1. jump _poped-dst_
