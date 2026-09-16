# Shell / Language Context Model

```text
:   Shell → Language
$   Language → Shell
```

### Language expression

```text
:()
```

evaluates a language expression.

For example:

```text
echo :(foo + 10)
```

The `foo + 10` portion is evaluated by the language.

### Language block

```text
:{}
```

enters a language block.

```text
echo :{
    var x = calculate()
    x
}
```

### Language variable

A language variable can also be referenced directly:

```text
:foo
```

Variables belong to the language side of the language. They are not inherently shell variables.

---

## Language → Shell

From language context, `$` enters shell context.

### Shell command substitution

```text
$()
```

evaluates a shell command and produces its result as a language value.

For example:

```text
var result = $(git status)
```

The contents of `$()` are parsed as shell syntax.

### Shell line

A `$` at the appropriate expression boundary can switch the remainder of the construct into shell context:

```text
var result = $ git status
```

This allows an entire shell command to be used without wrapping it in parentheses.

---

# Command Substitution

Parentheses are used for command substitution.

Unlike traditional shells where `$()` is required, this language uses:

```text
()
```

for command substitution while in shell context.

For example:

```text
echo (git status)
```

runs `git status` and substitutes its result.

This works because the shell grammar already provides a syntactic boundary around `(...)`.

When the surrounding context is language context, `$()` is used instead:

```text
var result = $(git status)
```

The `$` explicitly switches from language context into shell context.

Thus:

```text
Shell → Shell command
(git status)

Language → Shell command
$(git status)
```

---

# Variables

A command substitution can be used explicitly:

```text
var result = (git status)
```

And a language expression can be introduced explicitly:

```text
var result = :(foo + bar)
```

Alternatively, the entire assignment can switch to language context:

```text
var result =:
    foo + bar
```

The exact block/line syntax may be refined, but the principle is the same: `:` explicitly moves evaluation to the language side.

---

# Context-Dependent Constructs

Some constructs are available in both contexts.

Their **subexpressions determine which context is used**.

For example:

```text
if condition {
    echo "true"
} else {
    echo "false"
}
```

The `if` construct is usable from shell context, but `condition` is a language expression.

Therefore:

```text
if x > 10 {
    echo "large"
}
```

means:

```text
if <language expression> {
    <shell body>
}
```

Similarly:

```text
while condition {
    echo "running"
}
```

has a language expression as its condition.

And:

```text
loop {
    echo "running"
}
```

is a shell-context loop whose body remains shell context.

The general rule is:

> **When a grammar construct requires an expression, that expression is parsed in language context.**

This avoids requiring explicit context switches for common control-flow constructs.

---

# Context Rules

# Summary

The language has two distinct computational domains:

| Context      | Primary purpose                   | Escape into other context   |
| ------------ | --------------------------------- | --------------------------- |
| **Shell**    | Commands and shell syntax         | `:(...)`, `:{...}`, `:name` |
| **Language** | Typed expressions and programming | `$(...)`, `$ ...`           |

$<further config>()
:<further config>()

