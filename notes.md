I need to figure out how error reporting works.

Line start and end is a must, I can collect errors and ast nodes and at the end report allt eh errors at once.

language that is a shell language also a typed programming language.

file extensions define default mode.

lsh: shell
lm: programming

let test = shell {
    cd /home/rom/test
    echo "Test"
}

shell function is defined by:

```sh
:func {
    cd ./test
}
```

programming function is defined by:

```sh
def test() {
    let x = "test";
}
```

flow:

```
let x = "";

if bool && bool {
    ...
} else if {
    ...
} else {
    ...
}

for x in <iterable> {
    ...
}

do {
    ...
} for x in <iterable> {
    ..
}

while expr {
    ...
}

label: do {
    ...
} while expr {
    ...
}
```

```sh
for x in $iter.space files {
    ...
}

if $bool.str str_bool {
    ...
}

if $bool.int int {
    ...
}

x += 2

# escpaes syntax and go straight to execs in path
,, x+=2 


let repo_root = :shell(git switch $branch).unwrap()

shell {

}


:(), :{} -- calls into shell context
$(), ${} -- calls into language context

```
