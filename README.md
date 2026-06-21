# A Toy Compiler Targeting RISC-V 32I

## Code Generation

How many temporary register is enough?

````text
comp_expr                dest=t0  free={t0,t1,t2}
├── arith_expr (left)    dest=t0  free={t0,t1,t2}
│   ├── atom_expr        dest=t1  free={t0,t1,t2}
│   ├── (+/-)
│   └── atom_expr        dest=t2  free={t0,t2}
├── (==/>)
└── arith_expr (right)   dest=t1  free={t1,t2}
    ├── atom_expr        dest=t1  free={t1,t2}
    ├── (+/-)
    └── atom_expr        dest=t2  free={t2}  ← last free temp

Legend:
  dest  — the register this node writes its result into
  free  — registers still available for this node and its children to use

Rule: each right sibling consumes one register from the free pool,
      so max registers needed = height of the expression tree.
      ```
````

with a stack, the 3 regs could handle expressions of arbitrary depth because each push creates a new subtree with free regs:

```text
comp_expr
└── arith_expr (left)
    └── atom_expr::Group  dest=t1 ← push(t0,t2); RECURSE with free={t0,t1,t2} fresh; pop
```
