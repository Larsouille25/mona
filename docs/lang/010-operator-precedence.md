- **Feature Name:** `operator-precedence` 
- **Zom Issue:** Not related to an issue 
- **Status:** `Implemented` since [05b2a4b](https://github.com/zom-lang/zom/commit/05b2a4bc6c713ed7ca4371185d78e3863a458f2b)

# Operator Precedence
There is the Operator Precedence Table.

|          Op. Const(s)         | Precendence Group Name | Value |
| ----------------------------- | ---------------------- | ----- |
| `OP_MUL`, `OP_DIV`, `OP_MOD`  | `PRECEDE_MUL_DIV_MOD`  |   60  |
| `OP_PLUS`, `OP_MINUS`         | `PRECEDE_ADD_SUB`      |   40  |
| `OP_COMP_LT`, `OP_COMP_GT`,   | `PRECEDE_COMP`         |   20  |
| `OP_COMP_LTE`, `OP_COMP_GTE`  | `PRECEDE_COMP`         |   20  |
| `OP_COMP_EQ`, `OP_COMP_NE`,   | `PRECEDE_EQ_NE`        |   10  |
| `OP_AND`                      | `PRECEDE_AND`          |   6   |
| `OP_OR`                       | `PRECEDE_OR`           |   5   |
| `OP_EQ`                       | `PRECEDE_EQ`           |   2   |
