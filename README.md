# Virtual Stack Machine 
* RustでVSM(Virtual Stack Machine) を作成

## VSMの構成(src/vsm.rs)
1. code
    * プログラムを格納するメモリ
2. stack
    * データを格納するメモリ 基本的にはstackとしてアクセス
3. CPU
    * 演算器といくつかのレジスタを持つ
        1. program_counter
            * 次に実行する命令を指すカウンタ
        2. stack_pointer
            * stackをアクセスするのに用いる
        3. global_top_address B0
            * global 変数の先頭アドレスを指すのに用いる
        4. frame_top_address B1
            * 関数のフレームの先頭アドレスを指すのに用いる

## 実行方法
* トレースなし
```bash
/virtual_stack_machine > cargo run <vsm_file> 
```
* トレースあり
```bash
/virtual_stack_machine > cargo run <vsm_file> -t
```
## VSM の命令セット
* 以下のように表現する
    * stack_pointer -> SP
    * stack -> M
    * global_top_address -> B0
    * frame_top_address -> B1

| 命令 | 意味 | 動作 |
|-----|-----|-----|
|EXIT|exit|exit(M[SP]);|
|||
|LC c|load constant|SP++; M[SP]=c;|
|LA b a|load address|SP++; M[SP]=Bb+a;|
|LV b a|load variable|SP++; M[SP]=M[Bb+a];|
|LI|load indirect|M[SP]=M[M[SP]];|
|||
|SI| store indirect |M[M[SP-1]]=M[SP]; SP-=2;|
|SV b a |store variable |M[Bb+a]=M[SP]; SP--;|
|||
|DUP |duplicate |SP++; M[SP]=M[SP-1];|
|ISP c| increment sp |SP+=c;|
|||
|GETC |get character |SP++; M[SP]= 一文字入力;|
|GETI |get integer |SP++; M[SP]= 空白で区切られた整数を入力;|
|PUTC |put character |M[SP] の一文字を出力; SP--;|
|PUTI |put integer |M[SP] の整数を出力; SP--;|
|||
|ADD |add |SP--; |M[SP]=M[SP]+M[SP+1];|
|SUB |subtract |SP--; M[SP]=M[SP]-M[SP+1];|
|MUL |multiply |SP--; M[SP]=M[SP]*M[SP+1];|
|DIV |divide |SP--; M[SP]=M[SP]/M[SP+1];|
|MOD |modulo |SP--; M[SP]=M[SP]%M[SP+1];|
|INV |invert |M[SP]=-M[SP];|
|||
|EQ |equal |SP--; M[SP]=(M[SP]==M[SP+1]);|
|NE |not equal |SP--; M[SP]=(M[SP]!=M[SP+1]);|
|GT |greater than |SP--; M[SP]=(M[SP]>M[SP+1]);|
|LT |less than |SP--; M[SP]=(M[SP]<M[SP+1]);|
|GE |greater or equal |SP--; M[SP]=(M[SP]>=M[SP+1]);|
|LE |less or equal than |SP--; M[SP]=(M[SP]<=M[SP+1]);|
|||
|B a |branch |PC+=a;|
|BZ a |branch if zero |if (M[SP]==0) {PC+=a;} SP--;|
|||
|SB b |set base |B[b] = M[SP]; SP--;|
|CALL a |call |M[SP+2]=B1; M[SP+3]=PC; B1=SP+1; PC=a;|
|RET |return |SP=B1; B1=M[SP+1]; PC=M[SP+2];|